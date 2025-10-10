import { fileURLToPath, URL } from 'node:url';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  // SSG設定
  ssr: true,

  // 1. ソースコードディレクトリの変更
  srcDir: './src/',

  // 2. アプリ設定
  app: {
    baseURL: process.env.NODE_ENV === 'production' ? '/DropWebP/' : '/',
    buildAssetsDir: '_nuxt/',
    head: {
      link: [
        // app側と同じGoogle Fontsを読み込み
        {
          rel: 'preconnect',
          href: 'https://fonts.googleapis.com'
        },
        {
          rel: 'preconnect',
          href: 'https://fonts.gstatic.com',
          crossorigin: 'anonymous'
        },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Noto+Sans+JP:wght@100..900&family=Noto+Sans+KR:wght@100..900&family=Noto+Sans+Mono:wght@100..900&family=Noto+Sans+TC:wght@100..900&family=Noto+Sans+SC:wght@100..900&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap'
        }
      ]
    }
  },

  // 3. SSG設定（静的サイト生成）
  nitro: {
    prerender: {
      routes: [
        '/', // ルートはミドルウェアでリダイレクト
        '/ja',
        '/en',
        '/fr',
        '/ko',
        '/zhHant',
        '/zhHans',
        '/ja/format-guide',
        '/en/format-guide',
        '/fr/format-guide',
        '/ko/format-guide',
        '/zhHant/format-guide',
        '/zhHans/format-guide',
        '/ja/getting-started',
        '/en/getting-started',
        '/fr/getting-started',
        '/ko/getting-started',
        '/zhHant/getting-started',
        '/zhHans/getting-started'
      ]
    }
  },

  // 4. ルーティング設定
  router: {
    options: {
      strict: false
    }
  },

  modules: ['@nuxt/eslint', 'vuetify-nuxt-module', '@pinia/nuxt'],

  // CSS設定（Vuetifyのフォント設定を含む）
  css: ['@/styles/settings.scss'],

  // Vuetify設定
  vuetify: {
    vuetifyOptions: {
      theme: {
        defaultTheme: 'light',
        themes: {
          light: {
            colors: {
              primary: '#1976d2',
              secondary: '#424242',
              accent: '#82b1ff',
              error: '#ff5252',
              info: '#2196f3',
              success: '#4caf50',
              warning: '#ffc107'
            }
          }
        }
      }
    }
  },

  // TypeScript パスエイリアス設定
  alias: {
    '@': fileURLToPath(new URL('./src', import.meta.url))
  },

  build: {
    transpile: ['vue-i18n']
  },

  imports: {
    // Auto-import directories
    dirs: ['~/composables', '~/utils'],
    // グローバル関数のauto-import設定
    global: true
  },

  vite: {
    // YAMLファイルをロードできるようにアセットとして処理
    assetsInclude: ['**/*.yaml', '**/*.yml'],
    plugins: []
  },

  hooks: {
    'vite:extend': ({ config }) => {
      config.plugins = config.plugins || [];

      // 動的インポート不要な形でプラグインを追加
      config.plugins.push({
        name: 'vue-i18n-loader-inline',
        enforce: 'pre',
        transform(code: string, id: string) {
          // .vueファイルで<i18n>ブロックが含まれている場合のみ処理
          if (!id.endsWith('.vue') || !code.includes('<i18n')) {
            return null;
          }

          console.log(`[i18n-loader] Processing: ${id}`);

          // <i18n>ブロック抽出関数
          const extractI18nBlocks = (code: string) => {
            const blocks: Array<{ content: string; lang: string; start: number; end: number }> = [];

            // YAML形式
            const yamlRegex = /<i18n\s+lang=["']yaml["']>([\s\S]*?)<\/i18n>/g;
            let match;
            while ((match = yamlRegex.exec(code)) !== null) {
              if (match[1]) {
                blocks.push({
                  content: match[1],
                  lang: 'yaml',
                  start: match.index,
                  end: match.index + match[0].length
                });
              }
            }
            return blocks;
          };

          const i18nBlocks = extractI18nBlocks(code);
          if (i18nBlocks.length === 0) return null;

          // YAMLをパース
          const yaml = require('yaml');
          const messages: Record<string, any> = {};

          for (const block of i18nBlocks) {
            try {
              const parsed = yaml.parse(block.content.trim());
              Object.assign(messages, parsed);
            } catch (error) {
              console.error(`[i18n-loader] Failed to parse YAML:`, error);
            }
          }

          // <i18n>ブロックを削除
          let transformedCode = code;
          for (let i = i18nBlocks.length - 1; i >= 0; i--) {
            const block = i18nBlocks[i];
            if (block) {
              transformedCode =
                transformedCode.slice(0, block.start) + transformedCode.slice(block.end);
            }
          }

          // scriptタグに統合コードを追加
          const scriptSetupRegex = /(<script[^>]*setup[^>]*>)/;
          if (scriptSetupRegex.test(transformedCode)) {
            const integrationCode = `
// Auto-generated from <i18n> blocks
import { useLocalI18n } from '@/composables/useLocalI18n';

const __i18nMessages = ${JSON.stringify(messages, null, 2)};
const { t } = useLocalI18n(__i18nMessages);

// t関数をテンプレートで使用可能にする（自動expose）
defineExpose({ t });
`;

            transformedCode = transformedCode.replace(scriptSetupRegex, `$1${integrationCode}`);
            console.log(`[i18n-loader] Transformed: ${id}`);

            return { code: transformedCode, map: null };
          }

          return null;
        }
      });
    }
  }
});
