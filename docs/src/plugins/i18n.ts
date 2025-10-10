import { watch } from 'vue';
import { createI18n } from 'vue-i18n';

import { en, fr, ja, ko, zhHans, zhHant } from 'vuetify/locale';

import type { SupportedLocale } from '@/types/SupportedLocales';
import { detectBrowserLocale, getLocaleFromPath, updateHtmlLang } from '@/utils/locale';

// メッセージにVuetifyロケールを統合
const messages = {
  ja: {
    $vuetify: ja
  },
  fr: {
    $vuetify: fr
  },
  en: {
    $vuetify: en
  },
  ko: {
    $vuetify: ko
  },
  zhHant: {
    $vuetify: zhHant
  },
  zhHans: {
    $vuetify: zhHans
  }
};

export default defineNuxtPlugin(nuxtApp => {
  // SSR対応: Nuxtアプリのrouteからロケールを取得
  const route = useRoute();
  const routeLocale = getLocaleFromPath(route.path);

  // クライアントサイドでのフォールバック処理
  let locale: SupportedLocale = routeLocale || 'ja';

  // クライアントサイドかつルートにロケールがない場合のみブラウザ設定から判定
  if (import.meta.client && !routeLocale) {
    const browserLocale = detectBrowserLocale();
    locale = browserLocale || 'ja';
  }

  // i18nインスタンスを作成
  const i18n = createI18n({
    legacy: false,
    locale,
    fallbackLocale: 'ja',
    messages,
    globalInjection: false
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
