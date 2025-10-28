<template>
  <div>
    <TableOfContents />
    <MarkdownContent
      content-path="build-macos"
      :title="t('title')"
      :description="t('description')"
    />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import MarkdownContent from '@/components/MarkdownContent.vue';
import TableOfContents from '@/components/TableOfContents.vue';

const { t } = useI18n();
</script>

<i18n lang="yaml">
en:
  title: Build Drop Compress Image for macOS
  description: Learn how to build Drop Compress Image application on macOS systems with detailed instructions for setting up the development environment.
fr:
  title: Construire Drop Compress Image pour macOS
  description: Apprenez à construire l'application Drop Compress Image sur les systèmes macOS avec des instructions détaillées pour configurer l'environnement de développement.
ja:
  title: macOS 用 Drop Compress Image のビルド
  description: macOS システムで Drop Compress Image アプリケーションをビルドする方法と、開発環境のセットアップに関する詳細な手順を学びます。
ko:
  title: macOS용 Drop Compress Image 빌드
  description: macOS 시스템에서 Drop Compress Image 애플리케이션을 빌드 하는 방법과 개발 환경 설정에 대한 자세한 지침을 알아보세요.
zhHant:
  title: 為 macOS 建置 Drop Compress Image
  description: 了解如何在 macOS 系統上建置 Drop Compress Image 應用程式，並獲取設置開發環境的詳細說明。
zhHans:
  title: 为 macOS 构建 Drop Compress Image
  description: 了解如何在 macOS 系统上构建 Drop Compress Image 应用程序，并获取设置开发环境的详细说明。
</i18n>
