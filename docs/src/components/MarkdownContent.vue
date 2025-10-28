<template>
  <div class="content-with-toc">
    <!-- eslint-disable-next-line vue/no-v-html -->
    <article v-if="compiledHtml" class="markdown-body" v-html="compiledHtml" />
    <v-alert v-else-if="pending" :title="t('loading')" color="info" />
    <v-alert v-else-if="error" :title="t('error')" color="error">{{ error.message }}</v-alert>
  </div>
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

<style scoped>
.content-with-toc {
  margin-left: 280px;
  padding: 24px;
}

@media (max-width: 1264px) {
  .content-with-toc {
    margin-left: 0;
  }
}

/* GitHub Markdown CSS ダークモード切り替え */
.markdown-body {
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  margin: 0 auto;
  padding: 45px;
  /* OSのcolor-schemeに依存しない設定 */
  color-scheme: none;
  /* Vuetifyのライトテーマを明示的に適用 */
  background-color: rgb(var(--v-theme-surface));
  color: rgb(var(--v-theme-on-surface));
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

.v-theme--dark .markdown-body table th,
.v-theme--dark .markdown-body table td {
  border-color: rgb(var(--v-theme-outline)) !important;
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
