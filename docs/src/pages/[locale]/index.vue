<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import logo from '@/assets/logo.png';

const { t } = useI18n();
const route = useRoute();

// 現在のロケールを取得
const currentLocale = (route.params.locale as string) || 'en';

// ロケール対応のルート生成関数
const getLocalizedRoute = (path: string) => {
  return `/${currentLocale}${path}`;
};

const features = [
  {
    icon: 'mdi-image-multiple',
    key: 'multiple_formats'
  },
  {
    icon: 'mdi-lightning-bolt',
    key: 'high_speed'
  },
  {
    icon: 'mdi-drag',
    key: 'drag_drop'
  }
];
</script>

<template>
  <v-card class="mb-6" tag="section">
    <v-img :src="logo" alt="Drop Compress Image Logo" max-width="256" class="mx-auto mb-4" />
    <v-card-title class="text-h4 text-center pa-6" tag="h2">
      {{ t('title') }}
    </v-card-title>
    <v-card-text class="text-center">
      <p class="text-h6 mb-4">{{ t('subtitle') }}</p>
      <v-btn color="primary" size="large" :to="getLocalizedRoute('/getting-started')">
        <v-icon start>mdi-rocket</v-icon>
        {{ t('start_button') }}
      </v-btn>
    </v-card-text>
  </v-card>

  <v-row>
    <v-col v-for="item in features" :key="item.key" cols="12" md="4">
      <v-card class="h-100">
        <v-card-text class="text-center">
          <v-icon :icon="item.icon" size="64" color="primary" class="mb-4" />
          <h3 class="text-h6 mb-2">{{ t(`features.${item.key}.title`) }}</h3>
          <p>{{ t(`features.${item.key}.description`) }}</p>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<i18n lang="yaml">
en:
  title: Drop Compress Image Help Site
  subtitle: Help and guide for image compression app
  start_button: Get Started
  features:
    multiple_formats:
      title: Multiple Format Support
      description: Support for modern image formats like PNG(Zopfli), WebP, AVIF, JPEG XL.
    high_speed:
      title: High-Speed Conversion
      description: Fast image processing with Rust-based engine.
    drag_drop:
      title: Drag & Drop
      description: Easy batch conversion with simple operations.
fr:
  title: Site d'aide Drop Compress Image
  subtitle: Aide et guide pour l'application de compression d'images
  start_button: Commencer
  features:
    multiple_formats:
      title: Prise en charge de plusieurs formats
      description: Prise en charge des formats d'image modernes tels que PNG(Zopfli), WebP, AVIF, JXL.
    high_speed:
      title: Conversion haute vitesse
      description: Traitement rapide des images avec un moteur basé sur Rust.
    drag_drop:
      title: Glisser-Déposer
      description: Conversion par lots facile avec des opérations simples.
ja:
  title: Drop Compress Image ヘルプサイト
  subtitle: 画像圧縮アプリのヘルプとガイド
  start_button: はじめに
  features:
    multiple_formats:
      title: 複数形式対応
      description: PNG(Zopfli)、WebP、AVIF、JPEG XLなどの最新画像形式に対応。
    high_speed:
      title: 高速変換
      description: Rust基盤で高速な画像処理を実現。
    drag_drop:
      title: ドラッグ&ドロップ
      description: 簡単な操作で画像を一括変換可能。
ko:
  title: Drop Compress Image 도움말 사이트
  subtitle: 이미지 압축 앱의 도움말과 가이드
  start_button: 시작하기
  features:
    multiple_formats:
      title: 다중 형식 지원
      description: PNG(Zopfli), WebP, AVIF, JPEG XL 등 최신 이미지 형식 지원.
    high_speed:
      title: 고속 변환
      description: Rust 기반의 고속 이미지 처리.
    drag_drop:
      title: 드래그 & 드롭
      description: 간단한 조작으로 이미지 일괄 변환 가능.
zhHant:
  title: Drop Compress Image 幫助網站
  subtitle: 圖像壓縮應用程式的幫助和指南
  start_button: 入門
  features:
    multiple_formats:
      title: 多格式支援
      description: 支援 PNG(Zopfli)、WebP、AVIF、JPEG XL 等現代圖像格式。
    high_speed:
      title: 高速轉換
      description: 使用基於 Rust 的引擎進行快速圖像處理。
    drag_drop:
      title: 拖放功能
      description: 通過簡單操作輕鬆進行批量轉換。
zhHans:
  title: Drop Compress Image 帮助网站
  subtitle: 图像压缩应用程序的帮助和指南
  start_button: 入门
  features:
    multiple_formats:
      title: 多格式支持
      description: 支持 PNG(Zopfli)、WebP、AVIF、JPEG XL 等现代图像格式。
    high_speed:
      title: 高速转换
      description: 使用基于 Rust 的引擎进行快速图像处理。
    drag_drop:
      title: 拖放功能
      description: 通过简单操作轻松进行批量转换。
</i18n>
