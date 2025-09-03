import { defineStore } from 'pinia';
import { ref, watch, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';

/** Config Store */
export default defineStore(
  'config',
  () => {
    // 1. i18nインスタンスからlocaleを取得
    const { locale } = useI18n({ useScope: 'global' });

    // 2. Piniaのstateとして言語を定義（デフォルト値やlocalStorageからの復元など）
    const currentLocale = ref(locale.value); // 初期値をi18nから拝借

    // 3. stateが変更されたら、i18nのlocaleにも反映させる watchを設置
    watch(currentLocale, newLocale => {
      locale.value = newLocale;
      // 必要ならlocalStorageに保存する処理もここに追加
      // localStorage.setItem('locale', newLocale)
    });

    /** Dark Theme mode */
    const theme: Ref<boolean> = ref(window.matchMedia('(prefers-color-scheme: dark)').matches);

    /** Toggle Dark/Light mode */
    const toggleTheme = () => (theme.value = !theme.value);
    /**
     * Set Locale.
     *
     * @param locale - Locale
     */
    const setLocale = (l: string) => (locale.value = l);

    return { theme, locale, toggleTheme, setLocale };
  },
  // Data persistence destination
  {
    persist: {
      storage: window.localStorage
    }
  }
);
