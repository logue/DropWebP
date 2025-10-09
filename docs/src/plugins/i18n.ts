import { watch } from 'vue';
import { createI18n } from 'vue-i18n';

import { en, fr, ja, ko, zhHans, zhHant } from 'vuetify/locale';
import { detectBrowserLocale, getLocaleFromPath, updateHtmlLang } from '@/utils/locale';

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
export type SupportedLocale = (typeof supportedLocales)[number];

export default defineNuxtPlugin(nuxtApp => {
  // URLから言語コードを取得（ユーティリティ関数を使用）
  const getLocaleFromRoute = (): SupportedLocale => {
    if (import.meta.client) {
      const pathLocale = getLocaleFromPath(window.location.pathname);
      return pathLocale || 'en';
    }
    return 'en';
  };

  // URLから言語を優先的に取得
  const routeLocale = getLocaleFromRoute();
  let locale: SupportedLocale = routeLocale;

  // URLに言語コードがない場合はブラウザ設定から判定
  if (!routeLocale || routeLocale === 'en') {
    const browserLocale = detectBrowserLocale();
    locale = browserLocale;
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
      i18n: i18n.global
    }
  };
});
