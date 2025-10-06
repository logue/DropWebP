export default defineNuxtRouteMiddleware(to => {
  // サポートしている言語
  const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];

  // URLパスから言語コードを抽出
  const pathSegments = to.path.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] || '';

  // 言語コードがパスに含まれている場合、i18nの言語を設定
  if (supportedLocales.includes(firstSegment)) {
    // クライアントサイドでi18nの言語を設定
    if (process.client) {
      const { $i18n } = useNuxtApp();
      const i18n = $i18n as any;
      if (i18n && i18n.locale && i18n.locale.value !== firstSegment) {
        i18n.locale.value = firstSegment as 'ja' | 'en' | 'ko' | 'zh-tw';
      }
    }
    return;
  }

  // デフォルトは英語（フォールバック）
  if (process.client) {
    const { $i18n } = useNuxtApp();
    const i18n = $i18n as any;
    if (i18n && i18n.locale && i18n.locale.value !== 'en') {
      i18n.locale.value = 'en';
    }
  }
});
