import { SupportedLocales, type SupportedLocale } from '@/types/SupportedLocales';

export default defineNuxtRouteMiddleware(to => {
  // ルートパス（/）の場合のみリダイレクト処理
  if (to.path !== '/') {
    return;
  }
  let targetLocale = 'en'; // フォールバックは英語

  if (import.meta.client) {
    // 1. localStorage から優先的に取得
    const savedLocale = localStorage.getItem('locale') as SupportedLocale;
    if (savedLocale && SupportedLocales.includes(savedLocale)) {
      targetLocale = savedLocale;
    } else {
      // 2. ブラウザの言語設定から判定
      const locale = navigator.language.slice(0, 2);
      if (locale === 'zh') {
        // 中国語の詳細なロケールを確認
        const fullLocale = navigator.language.toLowerCase();
        if (fullLocale === 'zh-cn' || fullLocale === 'zh-sg') {
          targetLocale = 'zhHans'; // 簡体字中国語
        } else {
          targetLocale = 'zhHant'; // 繁体字中国語
        }
      } else if (SupportedLocales.includes(locale as SupportedLocale)) {
        targetLocale = locale;
      }
    }

    // 適切な言語のページにリダイレクト
    return navigateTo(`/${targetLocale}`, { redirectCode: 302 });
  }
});
