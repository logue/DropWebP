import { createI18n } from 'vue-i18n';

const messages = {
  ja: {},
  en: {},
  ko: {},
  'zh-tw': {}
};

export default defineNuxtPlugin(nuxtApp => {
  // URLから言語コードを取得する関数
  const getLocaleFromRoute = (): string => {
    if (process.client) {
      const path = window.location.pathname;
      const pathSegments = path.split('/').filter(Boolean);
      const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];

      // パスの先頭から言語コードを抽出（例: /en/getting-started -> en）
      if (pathSegments.length > 0 && supportedLocales.includes(pathSegments[0])) {
        return pathSegments[0];
      }
    }
    // パスに言語コードがない場合はデフォルト（日本語）
    return 'ja';
  };

  // ブラウザの言語設定を取得（フォールバック用）
  const browserLocale = process.client ? navigator.language.slice(0, 2) || 'ja' : 'ja';

  // URLから言語を優先的に取得、なければブラウザ設定
  const routeLocale = getLocaleFromRoute();
  let defaultLocale = routeLocale;

  // 言語コード正規化
  if (browserLocale === 'zh' && defaultLocale === 'ja') {
    defaultLocale = 'zh-tw';
  }

  // 対応言語でない場合は日本語をデフォルトに
  const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];
  if (!supportedLocales.includes(defaultLocale!)) {
    defaultLocale = 'ja';
  }

  // i18nインスタンスを作成
  const i18n = createI18n({
    legacy: false,
    locale: defaultLocale,
    fallbackLocale: 'ja',
    messages,
    globalInjection: true
  });

  // VueアプリインスタンスにVue I18nをインストール
  nuxtApp.vueApp.use(i18n);

  // i18nインスタンスをNuxtアプリで利用可能にする
  return {
    provide: {
      i18n: i18n.global
    }
  };
});
