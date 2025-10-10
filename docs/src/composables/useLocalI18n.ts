import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

/**
 * コンポーネントローカルのi18nメッセージを使用するcomposable
 * @param messages ローカルメッセージオブジェクト
 * @returns ローカル優先のt関数
 */
export function useLocalI18n(messages: Record<string, any>) {
  const { t: globalT, locale } = useI18n({ useScope: 'global' });

  /**
   * ローカルメッセージを優先して翻訳を返すt関数
   */
  const t = (key: string, fallback?: string): string => {
    const keys = key.split('.');
    let value: any = messages[locale.value];

    // ローカルメッセージから検索
    for (const k of keys) {
      if (!k) continue; // 空文字列の場合はスキップ

      if (value && typeof value === 'object') {
        // 配列のインデックス記法をサポート（例：items[0], items[1]）
        const arrayRegex = /^(.+)\[(\d+)\]$/;
        const arrayMatch = arrayRegex.exec(k);

        if (arrayMatch?.[1] && arrayMatch?.[2]) {
          const arrayKey = arrayMatch[1];
          const indexStr = arrayMatch[2];
          const index = parseInt(indexStr, 10);

          if (arrayKey in value && Array.isArray(value[arrayKey]) && !isNaN(index)) {
            value = value[arrayKey][index];
            continue;
          }
        }

        // 通常のオブジェクトプロパティアクセス
        if (k in value) {
          value = value[k];
        } else {
          // ローカルメッセージにない場合はグローバルから
          return globalT(key) || fallback || key;
        }
      } else {
        // ローカルメッセージにない場合はグローバルから
        return globalT(key) || fallback || key;
      }
    }

    // 配列の場合は文字列化
    if (Array.isArray(value)) {
      return value.join(' ');
    }

    return (typeof value === 'string' ? value : String(value)) || globalT(key) || fallback || key;
  };

  return {
    t,
    locale: computed(() => locale.value),
    globalT
  };
}

/**
 * YAMLファイルからメッセージを読み込むヘルパー
 */
export async function loadYamlMessages(yamlContent: string): Promise<Record<string, any>> {
  try {
    const { parse } = await import('yaml');
    return parse(yamlContent) || {};
  } catch (error) {
    console.error('Failed to parse YAML messages:', error);
    return {};
  }
}
