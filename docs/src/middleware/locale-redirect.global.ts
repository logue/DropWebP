export default defineNuxtRouteMiddleware(to => {
  // ルートパス（/）の場合のみリダイレクト処理
  if (to.path === '/') {
    const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];
    let targetLocale = 'en'; // フォールバックは英語

    if (process.client) {
      // 1. localStorage から優先的に取得
      const savedLocale = localStorage.getItem('locale');
      if (savedLocale && supportedLocales.includes(savedLocale)) {
        targetLocale = savedLocale;
      } else {
        // 2. ブラウザの言語設定から判定
        const browserLang = navigator.language.toLowerCase();

        if (browserLang.startsWith('ja')) {
          targetLocale = 'ja';
        } else if (browserLang.startsWith('ko')) {
          targetLocale = 'ko';
        } else if (browserLang.startsWith('zh-tw') || browserLang.startsWith('zh-hant')) {
          targetLocale = 'zh-tw';
        } else {
          // その他は英語（フォールバック）
          targetLocale = 'en';
        }
      }
    }

    // 適切な言語のページにリダイレクト
    return navigateTo(`/${targetLocale}`, { redirectCode: 302 });
  }
});
