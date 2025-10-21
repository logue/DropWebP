<template>
  <v-navigation-drawer permanent location="right" width="280" class="toc-drawer">
    <v-list density="compact" nav>
      <v-list-subheader>{{ t('toc') }}</v-list-subheader>
      <v-list-item
        v-for="heading in headings"
        :key="heading.id"
        :to="`#${heading.id}`"
        :class="`toc-level-${heading.level}`"
        :value="heading.id"
        :active="activeId === heading.id"
      >
        <v-list-item-title>{{ heading.text }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

interface Heading {
  id: string;
  text: string;
  level: number;
}
const { t } = useI18n();
const headings = ref<Heading[]>([]);
const activeId = ref<string>('');

const generateId = (text: string): string => {
  return text
    .toLowerCase()
    .replaceAll(/\s+/g, '-')
    .replaceAll(/[^\w\-\u3040-\u309F\u30A0-\u30FF\u4E00-\u9FFF]/g, '');
};

const extractHeadings = () => {
  const headingElements = document.querySelectorAll('h2, h3');
  const result: Heading[] = [];

  for (const element of headingElements) {
    const text = element.textContent?.trim() || '';
    let id = element.id;

    // IDが無い場合は生成
    if (!id) {
      id = generateId(text);
      element.id = id;
    }

    result.push({
      id,
      text,
      level: Number.parseInt(element.tagName.substring(1))
    });
  }

  headings.value = result;
};

const updateActiveHeading = () => {
  const scrollPosition = window.scrollY + 100;
  const headingElements = document.querySelectorAll('h2, h3');

  let currentId = '';
  for (const element of headingElements) {
    const top = (element as HTMLElement).offsetTop;
    if (scrollPosition >= top) {
      currentId = element.id;
    }
  }

  activeId.value = currentId;
};

let observer: MutationObserver | null = null;

onMounted(() => {
  // 初回抽出
  setTimeout(() => {
    extractHeadings();
    updateActiveHeading();
  }, 100);

  // スクロールイベント
  window.addEventListener('scroll', updateActiveHeading);

  // DOM変更を監視（動的コンテンツ対応）
  observer = new MutationObserver(() => {
    extractHeadings();
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
});

onUnmounted(() => {
  window.removeEventListener('scroll', updateActiveHeading);
  if (observer) {
    observer.disconnect();
  }
});
</script>

<style scoped>
.toc-drawer {
  position: fixed !important;
  top: 64px !important; /* AppBar分のオフセット */
  height: calc(100vh - 64px) !important;
  overflow-y: auto;
}

.toc-level-2 {
  padding-left: 16px !important;
}

.toc-level-3 {
  padding-left: 32px !important;
  font-size: 0.875rem;
}

:deep(.v-list-item__overlay) {
  opacity: 0.08;
}
</style>

<i18n lang="yaml">
en:
  toc: Table of Contents
fr:
  toc: Table des matières
ja:
  toc: 目次
ko:
  toc: 목차
zhHant:
  toc: 目錄
zhHans:
  toc: 目录
</i18n>
