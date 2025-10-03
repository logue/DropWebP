export default defineNuxtRouteMiddleware(to => {
  // サポートしている言語
  const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];

  // URLパスから言語コードを抽出
  const pathSegments = to.path.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] || '';

  // 言語コードがパスに含まれている場合
  if (supportedLocales.includes(firstSegment)) {
    // 言語コードが既にあるので、何もしない
    return;
  }

  // ルートパス（/）の場合、日本語のインデックスページとして扱う
  if (to.path === '/') {
    return;
  }

  // 言語コードがない場合、ブラウザの言語設定を確認
  if (process.client) {
    const browserLocale = navigator.language.slice(0, 2) || 'ja';
    let redirectLocale = 'ja'; // デフォルトは日本語

    // ブラウザの言語がサポートしている言語の場合
    if (supportedLocales.includes(browserLocale)) {
      redirectLocale = browserLocale;
    } else if (browserLocale === 'zh') {
      redirectLocale = 'zh-tw';
    }

    // 日本語以外の場合は言語コード付きURLにリダイレクト
    if (redirectLocale !== 'ja') {
      return navigateTo(`/${redirectLocale}${to.path}`, { redirectCode: 302 });
    }
  }
});
