<template>
  <div>
    <TableOfContents />
    <MarkdownContent
      content-path="build-linux"
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
  title: Build Drop Compress Image for Linux
  description: Learn how to build Drop Compress Image application on Linux systems (Ubuntu 24.04 LTS) with detailed instructions for setting up the development environment.
fr:
  title: Construire Drop Compress Image pour Linux
  description: Apprenez à construire l'application Drop Compress Image sur les systèmes Linux (Ubuntu 24.04 LTS) avec des instructions détaillées pour configurer l'environnement de développement.
ja:
  title: Linux 用 Drop Compress Image のビルド
  description: Linux システム（Ubuntu 24.04 LTS）で Drop Compress Image アプリケーションをビルドする方法と、開発環境のセットアップに関する詳細な手順を学びます。
ko:
  title: Linux용 Drop Compress Image 빌드
  description: Linux 시스템(Ubuntu 24.04 LTS)에서 Drop Compress Image 애플리케이션을 빌드하는 방법과 개발 환경 설정에 대한 자세한 지침을 알아보세요.
zhHant:
  title: 為 Linux 建置 Drop Compress Image
  description: 了解如何在 Linux 系統（Ubuntu 24.04 LTS）上建置 Drop Compress Image 應用程式，並獲取設置開發環境的詳細說明。
zhHans:
  title: 为 Linux 构建 Drop Compress Image
  description: 了解如何在 Linux 系统（Ubuntu 24.04 LTS）上构建 Drop Compress Image 应用程序，并获取设置开发环境的详细说明。
</i18n>
