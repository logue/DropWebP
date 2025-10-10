import { parse } from 'yaml';

import type { Plugin } from 'vite';

interface I18nBlock {
  content: string;
  lang: string;
  start: number;
  end: number;
}

export function vueI18nLoaderPlugin(): Plugin {
  return {
    name: 'vue-i18n-loader',
    enforce: 'pre',
    transform(code: string, id: string) {
      // .vueファイルで<i18n>ブロックが含まれている場合のみ処理
      if (!id.endsWith('.vue') || !code.includes('<i18n')) {
        return null;
      }

      console.log(`[i18n-loader] Processing: ${id}`);

      // <i18n>ブロックを抽出
      const i18nBlocks: I18nBlock[] = [];

      // YAML形式の<i18n>ブロック
      const yamlRegex = /<i18n\s+lang=["']yaml["']>([\s\S]*?)<\/i18n>/g;
      let match;

      while ((match = yamlRegex.exec(code)) !== null) {
        if (match[1]) {
          i18nBlocks.push({
            content: match[1],
            lang: 'yaml',
            start: match.index,
            end: match.index + match[0].length
          });
        }
      }

      // JSON形式の<i18n>ブロック
      const jsonRegex = /<i18n\s+lang=["']json["']>([\s\S]*?)<\/i18n>/g;
      while ((match = jsonRegex.exec(code)) !== null) {
        if (match[1]) {
          i18nBlocks.push({
            content: match[1],
            lang: 'json',
            start: match.index,
            end: match.index + match[0].length
          });
        }
      }

      // lang属性なしの<i18n>ブロック（JSON想定）
      const defaultRegex = /<i18n>([\s\S]*?)<\/i18n>/g;
      while ((match = defaultRegex.exec(code)) !== null) {
        if (match[1]) {
          i18nBlocks.push({
            content: match[1],
            lang: 'json',
            start: match.index,
            end: match.index + match[0].length
          });
        }
      }

      if (i18nBlocks.length === 0) {
        return null;
      }

      // メッセージをパースして統合
      const messages: Record<string, any> = {};

      for (const block of i18nBlocks) {
        try {
          let parsed: any;
          if (block.lang === 'yaml') {
            parsed = parse(block.content.trim());
            console.log(`[i18n-loader] Parsed YAML:`, parsed);
          } else {
            parsed = JSON.parse(block.content.trim());
            console.log(`[i18n-loader] Parsed JSON:`, parsed);
          }

          // メッセージをマージ
          Object.assign(messages, parsed);
        } catch (error) {
          console.error(`[i18n-loader] Failed to parse ${block.lang.toUpperCase()}:`, error);
          console.error('Content:', block.content);
        }
      }

      // <i18n>ブロックを削除（後ろから順に）
      let transformedCode = code;
      for (let i = i18nBlocks.length - 1; i >= 0; i--) {
        const block = i18nBlocks[i];
        if (block) {
          transformedCode =
            transformedCode.slice(0, block.start) + transformedCode.slice(block.end);
        }
      }

      // scriptタグにi18nメッセージとuseLocalI18n統合を追加
      const scriptSetupRegex = /(<script[^>]*setup[^>]*>)/;
      if (scriptSetupRegex.test(transformedCode)) {
        const i18nIntegrationCode = `
// Auto-generated i18n messages from <i18n> blocks
import { useLocalI18n } from '@/composables/useLocalI18n';

const __i18nMessages = ${JSON.stringify(messages, null, 2)};
const { t } = useLocalI18n(__i18nMessages);
`;

        transformedCode = transformedCode.replace(scriptSetupRegex, `$1${i18nIntegrationCode}`);

        console.log(`[i18n-loader] Successfully processed ${id}`);
        return {
          code: transformedCode,
          map: null
        };
      }

      return null;
    }
  };
}
