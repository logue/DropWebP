import { createI18n } from 'vue-i18n';
import { watch } from 'vue';

import { en, fr, ja, ko, zhHans, zhHant } from 'vuetify/locale';

const messages = {
  ja: {
    $vuetify: { ja }
  },
  fr: { $vuetify: { fr } },
  en: { $vuetify: { en } },
  ko: { $vuetify: { ko } },
  zhHant: { $vuetify: { zhHant } },
  zhHans: { $vuetify: { zhHans } }
};

export const supportedLocales = ['en', 'fr', 'ja', 'ko', 'zhHant', 'zhHans'] as const;

export default defineNuxtPlugin(nuxtApp => {
  // URLから言語コードを取得する関数
  const getLocaleFromRoute = (): string => {
    if (import.meta.client) {
      const path = window.location.pathname;
      const pathSegments = path.split('/').filter(Boolean) as Array<
        (typeof supportedLocales)[number]
      >;

      // パスの先頭から言語コードを抽出（例: /en/getting-started -> en）
      if (pathSegments.length > 0 && supportedLocales.includes(pathSegments[0]!)) {
        return pathSegments[0]!;
      }
    }
    // パスに言語コードがない場合はデフォルト（日本語）
    return 'en';
  };

  // ブラウザの言語設定を取得（フォールバック用）
  let locale = import.meta.client ? navigator.language.slice(0, 2) || 'en' : 'en';

  // URLから言語を優先的に取得、なければブラウザ設定
  const routeLocale = getLocaleFromRoute();
  locale = routeLocale || locale;

  // 言語コード正規化
  if (locale === 'zh' && import.meta.client) {
    // 中国語の詳細なロケールを確認
    const fullLocale = navigator.language.toLowerCase();
    if (fullLocale === 'zh-cn' || fullLocale === 'zh-sg') {
      locale = 'zhHans'; // 簡体字中国語
    } else {
      locale = 'zhHant'; // 繁体字中国語
    }
  }

  // 対応言語でない場合は英語をデフォルトに
  if (!supportedLocales.includes(routeLocale as (typeof supportedLocales)[number])) {
    locale = 'en';
  }

  // i18nインスタンスを作成
  const i18n = createI18n({
    legacy: false,
    locale,
    fallbackLocale: 'ja',
    messages,
    globalInjection: true
  });

  // VueアプリインスタンスにVue I18nをインストール
  nuxtApp.vueApp.use(i18n);

  // HTML lang属性を更新する関数
  const updateHtmlLang = (newLocale: string) => {
    if (import.meta.client) {
      document.documentElement.lang = newLocale;
    }
  };

  // 初期のlang属性設定
  updateHtmlLang(locale);

  // ロケール変更時にlang属性を更新
  if (import.meta.client) {
    watch(
      () => i18n.global.locale.value,
      newLocale => {
        updateHtmlLang(newLocale);
      }
    );
  }

  // i18nインスタンスをNuxtアプリで利用可能にする
  return {
    provide: {
      i18n: i18n.global,
      updateHtmlLang
    }
  };
});
