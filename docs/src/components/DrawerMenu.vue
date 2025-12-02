<script setup lang="ts">
import type DrawerMenuItem from '@/interfaces/DrawerMenuItemInterface';

const { t } = useI18n();
const localePath = useLocalePath();

/** Drawer menu items */
const items: ComputedRef<DrawerMenuItem[]> = computed(() => [
  {
    title: t('home'),
    icon: 'mdi-home',
    to: localePath('/')
  },
  {
    title: '-' // Divider
  },
  {
    title: t('getting_started'),
    icon: 'mdi-power'
    // to: localePath('/getting-started')
  },
  {
    title: t('format_guide'),
    icon: 'mdi-file-image',
    to: localePath('/format-guide')
  },
  {
    title: '-' // Divider
  },
  {
    title: t('build_windows'),
    icon: 'mdi-microsoft-windows',
    to: localePath('/build-windows')
  },
  {
    title: t('build_macos'),
    icon: 'mdi-apple',
    to: localePath('/build-macos')
  },
  {
    title: t('build_linux'),
    icon: 'mdi-linux',
    to: localePath('/build-linux')
  },
  {
    title: t('build_linux_docker'),
    icon: 'mdi-docker',
    to: localePath('/build-linux-docker')
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
  format_guide: Format Guide
  build_windows: Windows Build Instructions
  build_macos: macOS Build Instructions
  build_linux: Linux Build Instructions
  build_linux_docker: Linux Build Instructions (docker)
fr:
  home: Accueil
  getting_started: Commencer
  format_guide: Guide des formats
  build_windows: Windows Instructions de construction
  build_macos: macOS Instructions de construction
  build_linux: Linux Instructions de construction
  build_linux_docker: Linux Instructions de construction (docker)
ja:
  home: ホーム
  getting_started: はじめに
  format_guide: フォーマットガイド
  build_windows: Windows ビルド手順
  build_macos: macOS ビルド手順
  build_linux: Linux ビルド手順
  build_linux_docker: Linux ビルド手順（docker）
ko:
  home: 홈
  getting_started: 시작하기
  format_guide: 포맷 가이드
  build_windows: Windows 빌드 지침
  build_macos: macOS 빌드 지침
  build_linux: Linux 빌드 지침
  build_linux_docker: Linux 빌드 지침 (docker)
zhHant:
  home: 首頁
  getting_started: 入門
  format_guide: 格式指南
  build_windows: Windows 建置說明
  build_macos: macOS 建置說明
  build_linux: Linux 建置說明
  build_linux_docker: Linux 建置說明 (docker)
zhHans:
  home: 主页
  getting_started: 入门
  format_guide: 格式指南
  build_windows: Windows 构建说明
  build_macos: macOS 构建说明
  build_linux: Linux 构建说明
  build_linux_docker: Linux 构建说明 (docker)
</i18n>
