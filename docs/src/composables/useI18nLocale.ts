import { computed, type ComputedRef } from 'vue';
import { useI18n, type Composer } from 'vue-i18n';

import { supportedLocales, type SupportedLocale } from '@/plugins/i18n';
import {
  getLocaleFromPath,
  removeLocaleFromPath,
  buildLocalizedPath,
  detectBrowserLocale,
  getLocaleFromStorage,
  updateHtmlLang,
  saveLocaleToStorage,
  resolveLocale as utilResolveLocale
} from '@/utils/locale';

/**
 * ロケール管理用composable
 * 複数のコンポーネントで使用される共通のロケール処理を提供
 */
export function useI18nLocale() {
  const route = useRoute();
  const nuxtApp = useNuxtApp();
  const { t } = useI18n();

  // i18nインスタンスを取得
  const i18n = nuxtApp.$i18n as Composer;

  /**
   * 現在のロケール（算出プロパティ）
   */
  const currentLocale: ComputedRef<SupportedLocale> = computed(() => {
    if (i18n?.locale?.value) {
      return i18n.locale.value as SupportedLocale;
    }
    return (route.params.locale as SupportedLocale) || 'en';
  });

  // ユーティリティ関数をラップして公開

  /**
   * ロケールを変更（i18n、HTML lang、localStorage全て更新）
   */
  const setLocale = (newLocale: SupportedLocale): void => {
    // i18nのロケールを更新
    if (i18n?.locale) {
      i18n.locale.value = newLocale;
    }

    // HTML lang属性を更新
    updateHtmlLang(newLocale);

    // localStorageに保存
    saveLocaleToStorage(newLocale);
  };

  /**
   * ロケール変更とページ遷移
   */
  const setLocaleAndNavigate = async (newLocale: SupportedLocale): Promise<void> => {
    // ロケール設定を更新
    setLocale(newLocale);

    // 現在のパスから言語コードを除去
    const cleanPath = removeLocaleFromPath(route.path);

    // 新しい言語のURLを構築
    const newPath = buildLocalizedPath(newLocale, cleanPath);

    // ページ遷移
    await navigateTo(newPath);
  };

  /**
   * 最適なロケールを決定（優先度：URL > localStorage > ブラウザ設定 > デフォルト）
   */
  const resolveLocale = utilResolveLocale;

  /** ロケール対応済みのルート */
  const r = (path: string) => {
    return `/${currentLocale}${path}`;
  };

  return {
    // 状態
    currentLocale,
    supportedLocales,
    i18n,
    t,
    r,
    route,

    // ユーティリティ関数（utils/locale.tsから再公開）
    updateHtmlLang,
    saveLocaleToStorage,
    getLocaleFromStorage,
    getLocaleFromPath,
    removeLocaleFromPath,
    buildLocalizedPath,
    detectBrowserLocale,
    resolveLocale,

    // メイン操作
    setLocale,
    setLocaleAndNavigate
  };
}

export type UseI18nLocaleReturn = ReturnType<typeof useI18nLocale>;
