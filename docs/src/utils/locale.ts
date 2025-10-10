import { SupportedLocales, type SupportedLocale } from '@/types/SupportedLocales';

/**
 * ロケール関連のユーティリティ関数
 * プラグインやmiddleware、composableで共通利用可能
 */

/**
 * URLパスから言語コードを抽出
 */
export function getLocaleFromPath(path: string): SupportedLocale | null {
  const pathSegments = path.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] as SupportedLocale;

  if (firstSegment && SupportedLocales.includes(firstSegment)) {
    return firstSegment;
  }
  return null;
}

/**
 * パスから言語コードを除去
 */
export function removeLocaleFromPath(path: string): string {
  const pathSegments = path.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] as SupportedLocale;

  if (firstSegment && SupportedLocales.includes(firstSegment)) {
    const cleanPath = '/' + pathSegments.slice(1).join('/');
    return cleanPath === '/' ? '' : cleanPath;
  }
  return path;
}

/**
 * 新しいロケール用のパスを構築
 */
export function buildLocalizedPath(locale: SupportedLocale, basePath: string = ''): string {
  const cleanPath = basePath.startsWith('/') ? basePath : `/${basePath}`;
  return `/${locale}${cleanPath || ''}`;
}

/**
 * ブラウザの言語設定から適切なロケールを判定
 */
export function detectBrowserLocale(): SupportedLocale {
  if (typeof navigator === 'undefined') {
    return 'en';
  }

  const browserLang = navigator.language.toLowerCase();

  // 言語コードの詳細な判定
  if (browserLang.startsWith('ja')) {
    return 'ja';
  } else if (browserLang.startsWith('fr')) {
    return 'fr';
  } else if (browserLang.startsWith('ko')) {
    return 'ko';
  } else if (browserLang.startsWith('zh-cn') || browserLang.startsWith('zh-sg')) {
    return 'zhHans'; // 簡体字中国語
  } else if (
    browserLang.startsWith('zh-tw') ||
    browserLang.startsWith('zh-hk') ||
    browserLang.startsWith('zh-hant')
  ) {
    return 'zhHant'; // 繁体字中国語
  }

  return 'en'; // デフォルト
}

/**
 * localStorageからロケールを取得
 */
export function getLocaleFromStorage(): SupportedLocale | null {
  if (typeof localStorage === 'undefined') {
    return null;
  }

  const saved = localStorage.getItem('locale');
  if (saved && SupportedLocales.includes(saved as SupportedLocale)) {
    return saved as SupportedLocale;
  }
  return null;
}

/**
 * HTML lang属性を更新
 */
export function updateHtmlLang(locale: SupportedLocale): void {
  if (typeof document !== 'undefined') {
    document.documentElement.lang = locale;
  }
}

/**
 * localStorageにロケールを保存
 */
export function saveLocaleToStorage(locale: SupportedLocale): void {
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('locale', locale);
  }
}

/**
 * 最適なロケールを決定（優先度：URL > localStorage > ブラウザ設定 > デフォルト）
 */
export function resolveLocale(currentPath?: string): SupportedLocale {
  const path = currentPath || (typeof window !== 'undefined' ? window.location.pathname : '');

  // 1. URLから取得を試行
  const pathLocale = getLocaleFromPath(path);
  if (pathLocale) {
    return pathLocale;
  }

  // 2. localStorageから取得を試行
  const savedLocale = getLocaleFromStorage();
  if (savedLocale) {
    return savedLocale;
  }

  // 3. ブラウザ設定から判定
  return detectBrowserLocale();
}
