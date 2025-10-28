<template>
  <div>
    <TableOfContents />
    <MarkdownContent
      content-path="format-guide"
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
  title: Image Format Guide
  description: A comprehensive guide to various image formats including PNG, JPEG, WebP, and AVIF, detailing their features and best use cases.
fr:
  title: Guide des formats d'image
  description: Un guide complet des différents formats d'image, y compris PNG, JPEG, WebP et AVIF, détaillant leurs caractéristiques et leurs meilleurs cas d'utilisation.
ja:
  title: 画像フォーマットガイド
  description: PNG、JPEG、WebP、AVIFなどのさまざまな画像フォーマットの特徴と最適な使用例を詳述した包括的なガイド。
ko:
  title: 이미지 포맷 가이드
  description: PNG, JPEG, WebP 및 AVIF를 포함한 다양한 이미지 포맷에 대한 포괄적인 가이드로, 그 특징과 최적의 사용 사례를 자세히 설명합니다.
zhHant:
  title: 圖像格式指南
  description: 包含 PNG、JPEG、WebP 和 AVIF 等各種圖像格式的綜合指南，詳細介紹其特點和最佳使用案例。
zhHans:
  title: 图像格式指南
  description: 包含 PNG、JPEG、WebP 和 AVIF 等各种图像格式的综合指南，详细介绍其特点和最佳使用案例。
</i18n>
