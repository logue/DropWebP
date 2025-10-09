<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useI18nLocale } from '@/composables/useI18nLocale';

import logo from '@/assets/logo.png';

const { t } = useI18n();
const { r } = useI18nLocale();
const version = '2.2.1';
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
  <v-card class="mb-6" tag="section" elevation="0">
    <v-img :src="logo" alt="Drop Compress Image Logo" max-width="256" class="mx-auto mb-4" />
    <v-card-title class="text-h4 text-center pa-6" tag="h2">Drop Compress Image</v-card-title>
    <v-card-text class="text-center">
      <p class="text-h6 mb-4">{{ t('subtitle') }}</p>
      <div class="my-4">
        <v-btn
          size="large"
          :href="`https://github.com/logue/DropWebP/releases/download/${version}/drop-compress-image_${version}_x64_en-US.msi`"
        >
          {{ t('download.windows') }}
          <br />
          <small class="text-secondary">({{ t('download.window_requirement') }})</small>
        </v-btn>
        <v-btn
          size="large"
          class="ml-4"
          :href="`https://github.com/logue/DropWebP/releases/download/${version}/drop-compress-image_${version}_aarch64.dmg`"
        >
          {{ t('download.macos') }}
          <br />
          <small class="text-secondary">({{ t('download.macos_requirement') }})</small>
        </v-btn>
      </div>
    </v-card-text>
  </v-card>

  <v-card class="mb-6" tag="section" elevation="0">
    <v-card-text>
      <v-row class="mb-5">
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
      <v-btn color="primary" size="large" :to="r('/getting-started')">
        <v-icon start>mdi-rocket</v-icon>
        {{ t('start_button') }}
      </v-btn>
    </v-card-text>
  </v-card>
</template>

<i18n lang="yaml">
en:
  subtitle: Help and guide for image compression app
  start_button: Get Started
  download:
    windows: Download for Windows
    window_requirement: Windows 11 or later
    macos: Download for MacOS
    macos_requirement: M1 or later
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
  subtitle: Aide et guide pour l'application de compression d'images
  start_button: Commencer
  download:
    windows: Télécharger pour Windows
    window_requirement: Windows 11 ou version ultérieure
    macos: Télécharger pour MacOS
    macos_requirement: M1 ou version ultérieure
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
  subtitle: 画像圧縮アプリのヘルプとガイド
  start_button: はじめに
  download:
    windows: Windows版をダウンロード
    window_requirement: Windows 11以降
    macos: MacOS版をダウンロード
    macos_requirement: M1以降
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
  subtitle: 이미지 압축 앱의 도움말과 가이드
  start_button: 시작하기
  download:
    windows: Windows용 다운로드
    window_requirement: Windows 11 이상
    macos: MacOS용 다운로드
    macos_requirement: M1 이상
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
  subtitle: 圖像壓縮應用程式的幫助和指南
  start_button: 入門
  download:
    windows: 下載 Windows 版
    window_requirement: Windows 11 或更新版本
    macos: 下載 MacOS 版
    macos_requirement: M1 或更新版本
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
  subtitle: 图像压缩应用程序的帮助和指南
  start_button: 入门
  download:
    windows: 下载安装 Windows 版
    window_requirement: Windows 11 或更新版本
    macos: 下载安装 MacOS 版
    macos_requirement: M1 或更新版本
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
