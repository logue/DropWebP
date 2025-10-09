export default defineNuxtRouteMiddleware(to => {
  // サポートしている言語
  const supportedLocales = ['ja', 'en', 'ko', 'zhHant', 'zhHans'];

  // URLパスから言語コードを抽出
  const pathSegments = to.path.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] || '';

  // 言語コードがパスに含まれている場合、i18nの言語を設定
  if (supportedLocales.includes(firstSegment)) {
    // クライアントサイドでi18nの言語を設定
    if (import.meta.client) {
      const { $i18n } = useNuxtApp();
      const i18n = $i18n as any;
      if (i18n && i18n.locale && i18n.locale.value !== firstSegment) {
        i18n.locale.value = firstSegment as 'ja' | 'en' | 'ko' | 'zhHant' | 'zhHans';
        document.documentElement.lang = firstSegment;
      }
    }
    return;
  }

  // デフォルトは英語（フォールバック）
  if (import.meta.client) {
    const { $i18n } = useNuxtApp();
    const i18n = $i18n as any;
    if (i18n && i18n.locale && i18n.locale.value !== 'en') {
      i18n.locale.value = 'en';
      document.documentElement.lang = 'en';
    }
  }
});
