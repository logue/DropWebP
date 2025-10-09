import type { Composer } from 'vue-i18n';
import { supportedLocales } from '@/plugins/i18n';
import { getLocaleFromPath, updateHtmlLang } from '@/utils/locale';

export default defineNuxtRouteMiddleware(to => {
  // URLパスから言語コードを抽出
  const locale = getLocaleFromPath(to.path);

  // 言語コードがパスに含まれている場合、i18nの言語を設定
  if (locale && supportedLocales.includes(locale)) {
    // クライアントサイドでi18nの言語を設定
    if (import.meta.client) {
      const { $i18n } = useNuxtApp();
      const i18n = $i18n as Composer;
      if (i18n?.locale?.value !== locale) {
        i18n.locale.value = locale;
        updateHtmlLang(locale);
      }
    }
    return;
  }

  // デフォルトは英語（フォールバック）
  if (import.meta.client) {
    const { $i18n } = useNuxtApp();
    const i18n = $i18n as Composer;
    if (i18n?.locale?.value !== 'en') {
      i18n.locale.value = 'en';
      updateHtmlLang('en');
    }
  }
});
