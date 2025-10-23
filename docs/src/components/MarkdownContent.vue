<template>
  <div class="content-with-toc">
    <!-- eslint-disable-next-line vue/no-v-html -->
    <div v-if="compiledHtml" class="markdown-body" v-html="compiledHtml" />
    <div v-else-if="pending" class="loading">Loading...</div>
    <div v-else-if="error" class="error">Error loading content: {{ error.message }}</div>
  </div>
</template>

<script setup lang="ts">
import { parse, ready } from '@logue/markdown-wasm';
import Prism from 'prismjs';

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

  const locales = ['en', 'ja', 'fr', 'ko', 'zhHans', 'zhHant'];

  for (const loc of locales) {
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

    // @logue/markdown-wasmでMarkdownをHTMLに変換
    await ready();
    const result = parse(markdownContent);
    compiledHtml.value = typeof result === 'string' ? result : '<p>Error rendering markdown</p>';
    error.value = null;
  } catch (err) {
    console.error('Failed to render markdown:', err);
    error.value = err as Error;
    compiledHtml.value = '<p>Error loading content</p>';
  } finally {
    pending.value = false;
  }
};

// 初回レンダリング
await renderMarkdown();

// ロケール変更時に再レンダリング
watch(locale, renderMarkdown);

// SEO設定
useHead({
  title: props.title,
  meta: [
    {
      name: 'description',
      content: props.description
    }
  ]
});

onMounted(() => {
  // Highlight code blocks after markdown is rendered
  nextTick(() => {
    Prism.highlightAll();
  });
});

// Re-highlight when content changes
watch(compiledHtml, () => {
  nextTick(() => {
    Prism.highlightAll();
  });
});
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

.loading,
.error {
  padding: 20px;
  text-align: center;
  font-size: 16px;
}

.error {
  color: #f44336;
  background-color: #ffebee;
  border-radius: 4px;
  border: 1px solid #e57373;
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
