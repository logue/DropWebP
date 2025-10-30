<template>
  <v-sheet class="content-with-toc">
    <!-- eslint-disable-next-line vue/no-v-html -->
    <article v-if="compiledHtml" class="markdown-body" v-html="compiledHtml" />
    <v-alert v-else-if="pending" :title="t('loading')" color="info" variant="tonal" />
    <v-alert v-else-if="error" :title="t('error')" color="error" variant="tonal">
      {{ error.message }}
    </v-alert>
  </v-sheet>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import { marked } from 'marked';
import Prism from 'prismjs';

import { Locale } from '@/types/LocaleType';

const { t } = useI18n();

interface Props {
  contentPath: string; // 'build-windows' または 'build-macos'
  title?: string;
  description?: string;
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  description: ''
});

const { locale } = useI18n();

const compiledHtml = ref('');
const pending = ref(true);
const error = ref<Error | null>(null);

// Markdownファイルを動的にインポートする関数
const importMarkdownFiles = async () => {
  const markdownMap: Record<string, string> = {};

  for (const loc of Object.values(Locale)) {
    try {
      // 動的インポートでMarkdownファイルを取得
      const module = await import(`../../content/${props.contentPath}/${loc}.md?raw`);
      markdownMap[loc] = module.default;
    } catch (err) {
      console.warn(`Failed to load ${loc}.md for ${props.contentPath}:`, err);
      // フォールバック：英語版があれば使用
      if (loc !== 'en' && markdownMap.en) {
        markdownMap[loc] = markdownMap.en;
      }
    }
  }

  return markdownMap;
};

const renderMarkdown = async () => {
  try {
    pending.value = true;
    const markdownMap = await importMarkdownFiles();
    const localeCode = unref(locale);
    const markdownContent = markdownMap[localeCode] || markdownMap.en || '';

    if (!markdownContent) {
      throw new Error('No markdown content found');
    }

    // markedでMarkdownをHTMLに変換（SSR対応）
    try {
      compiledHtml.value = await marked(markdownContent, {
        gfm: true,
        breaks: true
      });
    } catch (parseError) {
      console.error('Markdown parsing error:', parseError);
      // フォールバック: プリフォーマット表示
      compiledHtml.value = `<pre>${markdownContent}</pre>`;
    }
    error.value = null;
  } catch (err) {
    console.error('Failed to render markdown:', err);
    error.value = err as Error;
    compiledHtml.value = '<p>Error loading content</p>';
  } finally {
    pending.value = false;
  }
};

// SSR/SSG対応: サーバー側でレンダリング
if (import.meta.server) {
  await renderMarkdown();
}

// クライアント側: マウント時にレンダリングとハイライト
onMounted(async () => {
  // SSRでレンダリングされていない場合のみ実行
  if (!compiledHtml.value) {
    await renderMarkdown();
  }

  // シンタックスハイライトを適用
  await nextTick();
  if (import.meta.client) {
    Prism.highlightAll();
  }
});

// ロケール変更時に再レンダリング
watch(locale, async () => {
  await renderMarkdown();

  // 再ハイライト
  await nextTick();
  if (import.meta.client) {
    Prism.highlightAll();
  }
});

// コンテンツ変更時に再ハイライト
watch(compiledHtml, async () => {
  await nextTick();
  if (import.meta.client) {
    Prism.highlightAll();
  }
});

// テーマ変更に対応するための強制リセット
onMounted(() => {
  if (import.meta.client) {
    const observer = new MutationObserver(mutations => {
      for (const mutation of mutations) {
        if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
          // テーマクラスが変更された時にテーブルスタイルを強制適用
          const tables = document.querySelectorAll('.markdown-body table');
          for (const table of tables) {
            // 強制的にスタイルを再適用
            (table as HTMLElement).style.cssText = '';
          }
        }
      }
    });

    const htmlElement = document.documentElement;
    observer.observe(htmlElement, { attributes: true, attributeFilter: ['class'] });

    onUnmounted(() => {
      observer.disconnect();
    });
  }
});

const i18nHead = useLocaleHead();

console.log(unref(i18nHead));

// SEO設定
useHead(() => ({
  title: props.title,
  htmlAttrs: {
    lang: i18nHead.value.htmlAttrs.lang
  },
  link: [...(i18nHead.value.link || [])],
  meta: [
    {
      name: 'description',
      content: props.description
    },
    ...(i18nHead.value.meta || [])
  ]
}));
</script>

<style scoped lang="scss">
/* GitHub Markdown CSS ダークモード切り替え */
.markdown-body {
  box-sizing: border-box;
  /* OSのcolor-schemeに依存しない設定 */
  color-scheme: none;
  /* Vuetifyのライトテーマを明示的に適用 */
  background-color: rgb(var(--v-theme-surface));
  color: rgb(var(--v-theme-on-surface));
}

/* GitHub Markdown CSSより優先させるための基本テーブルスタイル */
.markdown-body table {
  background-color: transparent !important;
  border-collapse: collapse !important;
}

.markdown-body table tr {
  background-color: transparent !important;
}

.markdown-body table th,
.markdown-body table td {
  background-color: transparent !important;
}

/* Vuetifyのダークテーマと明示的に同期 */
.v-theme--dark .markdown-body {
  /* OSに関係なくVuetifyのダークテーマを強制適用 */
  color-scheme: none !important;
  background-color: rgb(var(--v-theme-surface)) !important;
  color: rgb(var(--v-theme-on-surface)) !important;
}

.v-theme--dark .markdown-body h1,
.v-theme--dark .markdown-body h2,
.v-theme--dark .markdown-body h3,
.v-theme--dark .markdown-body h4,
.v-theme--dark .markdown-body h5,
.v-theme--dark .markdown-body h6 {
  color: rgb(var(--v-theme-on-surface)) !important;
}

.v-theme--dark .markdown-body pre,
.v-theme--dark .markdown-body code {
  background-color: rgb(var(--v-theme-surface-variant)) !important;
  color: rgb(var(--v-theme-on-surface-variant)) !important;
}

.v-theme--dark .markdown-body blockquote {
  border-left-color: rgb(var(--v-theme-outline)) !important;
  color: rgb(var(--v-theme-on-surface-variant)) !important;
}

.v-theme--dark .markdown-body table,
.v-theme--dark .markdown-body table tbody,
.v-theme--dark .markdown-body table thead {
  background-color: rgb(var(--v-theme-surface)) !important;
}

.v-theme--dark .markdown-body table th,
.v-theme--dark .markdown-body thead th {
  background-color: rgb(var(--v-theme-surface-variant)) !important;
  color: rgb(var(--v-theme-on-surface-variant)) !important;
  border-color: rgb(var(--v-theme-outline)) !important;
}

.v-theme--dark .markdown-body table td,
.v-theme--dark .markdown-body tbody td {
  background-color: rgb(var(--v-theme-surface)) !important;
  color: rgb(var(--v-theme-on-surface)) !important;
  border-color: rgb(var(--v-theme-outline)) !important;
}

.v-theme--dark .markdown-body table tr {
  background-color: rgb(var(--v-theme-surface)) !important;
}

.v-theme--dark .markdown-body table tr:nth-child(even) {
  background-color: rgba(var(--v-theme-on-surface), 0.1) !important;
}

.v-theme--dark .markdown-body table tr:nth-child(even) td {
  background-color: rgba(var(--v-theme-on-surface), 0.1) !important;
}

.v-theme--dark .markdown-body a {
  color: rgb(var(--v-theme-primary)) !important;
}

.v-theme--light .markdown-body {
  /* OSに関係なくVuetifyのライトテーマを強制適用 */
  color-scheme: none !important;
  background-color: rgb(var(--v-theme-surface)) !important;
  color: rgb(var(--v-theme-on-surface)) !important;
}

.v-theme--light .markdown-body h1,
.v-theme--light .markdown-body h2,
.v-theme--light .markdown-body h3,
.v-theme--light .markdown-body h4,
.v-theme--light .markdown-body h5,
.v-theme--light .markdown-body h6 {
  color: rgb(var(--v-theme-on-surface)) !important;
}

.v-theme--light .markdown-body pre,
.v-theme--light .markdown-body code {
  background-color: rgb(var(--v-theme-surface-variant)) !important;
  color: rgb(var(--v-theme-on-surface-variant)) !important;
}

.v-theme--light .markdown-body blockquote {
  border-left-color: rgb(var(--v-theme-outline)) !important;
  color: rgb(var(--v-theme-on-surface-variant)) !important;
}

.v-theme--light .markdown-body table,
.v-theme--light .markdown-body table tbody,
.v-theme--light .markdown-body table thead {
  background-color: rgb(var(--v-theme-surface)) !important;
}

.v-theme--light .markdown-body table th,
.v-theme--light .markdown-body thead th {
  background-color: rgb(var(--v-theme-surface-variant)) !important;
  color: rgb(var(--v-theme-on-surface-variant)) !important;
  border-color: rgb(var(--v-theme-outline)) !important;
}

.v-theme--light .markdown-body table td,
.v-theme--light .markdown-body tbody td {
  background-color: rgb(var(--v-theme-surface)) !important;
  color: rgb(var(--v-theme-on-surface)) !important;
  border-color: rgb(var(--v-theme-outline)) !important;
}

.v-theme--light .markdown-body table tr {
  background-color: rgb(var(--v-theme-surface)) !important;
}

.v-theme--light .markdown-body table tr:nth-child(even) {
  background-color: rgba(var(--v-theme-on-surface), 0.05) !important;
}

.v-theme--light .markdown-body table tr:nth-child(even) td {
  background-color: rgba(var(--v-theme-on-surface), 0.05) !important;
}

.v-theme--light .markdown-body a {
  color: rgb(var(--v-theme-primary)) !important;
}

@media (max-width: 767px) {
  .markdown-body {
    padding: 15px;
  }
}
</style>

<i18n lang="yaml">
en:
  loading: Loading...
  error: 'Error loading content:'
fr:
  loading: Chargement...
  error: 'Erreur lors du chargement du contenu :'
ja:
  loading: 読み込んでいます…
  error: 内容を取得時にエラーが発生しました：
ko:
  loading: 로딩 중...
  error: '콘텐츠를 불러오는 중 오류 발생:'
zhHant:
  loading: 加載中...
  error: '載入內容時出錯：'
zhHans:
  loading: 加载中...
  error: '加载内容时出错：'
</i18n>
