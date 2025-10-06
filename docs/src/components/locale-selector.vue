<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const route = useRoute();
const nuxtApp = useNuxtApp();

// i18nインスタンスを取得
const i18n = nuxtApp.$i18n;
const { t } = useI18n();

const currentLocale = computed(() => {
  // i18nのlocaleを優先し、フォールバックとしてルートパラメータを使用
  if (i18n && i18n.locale) {
    return i18n.locale.value;
  }
  return (route.params.locale as string) || 'ja';
});

const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];

const setLocale = async (newLocale: string) => {
  // i18nのロケールを更新
  if (i18n && i18n.locale) {
    i18n.locale.value = newLocale as 'ja' | 'en' | 'ko' | 'zh-tw';
  }

  // ローカルストレージに保存
  if (process.client) {
    localStorage.setItem('locale', newLocale);
  }

  // URLを言語対応に変更
  const currentPath = route.path;

  // 現在のパスから言語コードを除去
  const pathSegments = currentPath.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] || '';

  let cleanPath = currentPath;
  if (supportedLocales.includes(firstSegment)) {
    // 言語コードを除去したパスを作成
    cleanPath = '/' + pathSegments.slice(1).join('/');
    if (cleanPath === '/') {
      cleanPath = '';
    }
  }

  // 新しい言語のURLを構築（全ての言語でプレフィックス使用）
  const newPath = `/${newLocale}${cleanPath || ''}`;

  // 同じパスの場合は何もしない
  if (newPath !== currentPath) {
    console.log(`Navigating from ${currentPath} to ${newPath}`);
    await navigateTo(newPath);
  }
};
</script>

<template>
  <v-menu>
    <template #activator="{ props }">
      <v-btn v-bind="props" variant="plain" icon="mdi-translate" />
    </template>
    <v-list>
      <v-list-item
        v-for="locale of supportedLocales"
        :key="locale"
        :active="locale === currentLocale"
        @click="setLocale(locale)"
      >
        <v-list-item-title>{{ t(locale) }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<i18n lang="yaml">
en:
  en: 🇺🇸 English
  ja: 🇯🇵 Japanese
  ko: 🇰🇷 Korean
  zh-tw: 🇹🇼 Chinese
ja:
  en: 🇺🇸 英語
  ja: 🇯🇵 日本語
  ko: 🇰🇷 韓国語
  zh-tw: 🇹🇼 中国語
ko:
  en: 🇺🇸 영어
  ja: 🇯🇵 일본어
  ko: 🇰🇷 한국어
  zh-tw: 🇹🇼 중국어
zh-tw:
  en: 🇺🇸 英語
  ja: 🇯🇵 日語
  ko: 🇰🇷 韓語
  zh-tw: 🇹🇼 中文
</i18n>

<style scoped>
.v-list-item-title {
  /* グローバルフォント設定を継承し、絵文字表示を最適化 */
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* 絵文字の最適化はグローバル設定で適用 */
:deep(.v-list-item-title) {
  font-variant-emoji: unicode;
}
</style>
