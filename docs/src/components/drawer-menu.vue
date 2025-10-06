<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import type DrawerMenuItem from '@/interfaces/DrawerMenuItemInterface';

// 現在のルートから言語を取得
const route = useRoute();
const currentLocale = (route.params.locale as string) || 'ja';

// i18nのt関数を取得
const { t } = useI18n();

// 多言語対応のルート生成関数
const getLocalizedRoute = (path: string) => {
  // 全ての言語でロケールプレフィックスを使用
  const locale = currentLocale || 'en'; // フォールバックは英語
  return `/${locale}${path}`;
};

/** Drawer menu items */
const items: ComputedRef<DrawerMenuItem[]> = computed(() => [
  {
    title: t('home'),
    icon: 'mdi-home',
    to: getLocalizedRoute('/')
  },
  {
    title: '-' // Divider
  },
  {
    title: t('getting_started'),
    icon: 'mdi-power',
    to: getLocalizedRoute('/getting-started')
  }
]);
</script>

<template>
  <v-list nav>
    <template v-for="item in items" :key="item.title">
      <v-divider v-if="item.title === '-'" />
      <template v-else>
        <!-- Menu Item -->
        <v-list-item
          v-if="!item.items"
          :disabled="!item.to"
          :prepend-icon="item.icon"
          :title="item.title"
          :to="item.to"
          link
        />
        <!-- Sub menu -->
        <v-list-group v-else-if="item.items" v-model="item.active">
          <template #activator="{ props }">
            <v-list-item v-bind="props" :prepend-icon="item.icon" :title="item.title" />
          </template>
          <!-- Sub menu item -->
          <template v-for="subItem in item.items" :key="subItem.title">
            <v-divider v-if="subItem.title === '-'" />
            <v-list-item
              v-else
              :disabled="!subItem.to"
              :prepend-icon="subItem.icon"
              :title="subItem.title"
              :to="subItem.to"
              link
            />
          </template>
        </v-list-group>
      </template>
    </template>
  </v-list>
</template>

<i18n lang="yaml">
en:
  home: Home
  getting_started: Getting Started
ja:
  home: ホーム
  getting_started: はじめに
ko:
  home: 홈
  getting_started: 시작하기
zh-tw:
  home: 首頁
  getting_started: 入門
</i18n>
