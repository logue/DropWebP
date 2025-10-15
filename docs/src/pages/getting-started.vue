<script setup lang="ts">
const { t } = useI18n();
// メタデータ設定
useHead({
  title: t('getting_started.title'),
  meta: [{ name: 'description', content: t('getting_started.meta_description') }]
});

// インストールステップ
const installationSteps = ['1', '2', '3'];

// 使用方法
const usageMethods = [
  { key: 'drag_drop', icon: 'mdi-drag' },
  { key: 'file_menu', icon: 'mdi-file-multiple' },
  { key: 'batch_convert', icon: 'mdi-package-variant' },
  { key: 'settings', icon: 'mdi-cog' }
];

// サポート機能
const supportedFeatures = [
  { key: 'webp_support', icon: 'mdi-check-circle' },
  { key: 'avif_support', icon: 'mdi-check-circle' },
  { key: 'jxl_support', icon: 'mdi-check-circle' },
  { key: 'batch_processing', icon: 'mdi-check-circle' },
  { key: 'quality_control', icon: 'mdi-check-circle' },
  { key: 'cross_platform', icon: 'mdi-check-circle' }
];
</script>

<template>
  <v-card class="mb-6" tag="section">
    <v-card-title class="text-h3 text-center pa-6" tag="h2">
      {{ t('title') }}
    </v-card-title>
    <v-card-text>
      <p class="text-h6 mb-4">{{ t('subtitle') }}</p>
    </v-card-text>
  </v-card>

  <v-card class="mb-6" tag="section">
    <v-card-title class="text-h4 pa-4 bg-primary text-white" tag="h2">
      {{ t('installation.title') }}
    </v-card-title>
    <v-card-text class="pa-6">
      <v-stepper :items="installationSteps" variant="text">
        <template #item.1>
          <div>
            <h3 class="text-h6 mb-3">{{ t('installation.step1_title') }}</h3>
            <p class="mb-3">{{ t('installation.step1_desc') }}</p>
            <v-btn
              color="primary"
              variant="outlined"
              prepend-icon="mdi-download"
              href="https://github.com/logue/DropWebP/releases"
              target="_blank"
            >
              {{ t('download') }}
            </v-btn>
          </div>
        </template>

        <template #item.2>
          <div>
            <h3 class="text-h6 mb-3">{{ t('installation.step2_title') }}</h3>
            <p>{{ t('installation.step2_desc') }}</p>
          </div>
        </template>

        <template #item.3>
          <div>
            <h3 class="text-h6 mb-3">{{ t('installation.step3_title') }}</h3>
            <p>{{ t('installation.step3_desc') }}</p>
          </div>
        </template>
      </v-stepper>
    </v-card-text>
  </v-card>

  <v-card class="mb-6" tag="section">
    <v-card-title class="text-h4 pa-4 bg-secondary text-white" tag="h2">
      {{ t('usage.title') }}
    </v-card-title>
    <v-card-text class="pa-6">
      <v-row>
        <v-col v-for="method in usageMethods" :key="method.key" cols="12" md="6">
          <v-card elevation="2" class="h-100">
            <v-card-text class="text-center pa-6">
              <v-icon :icon="method.icon" size="48" color="primary" class="mb-4" />
              <h4 class="text-h6 mb-3">
                {{ t(`usage.methods.${method.key}.title`) }}
              </h4>
              <p>{{ t(`usage.methods.${method.key}.description`) }}</p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>

  <v-card tag="section">
    <v-card-title class="text-h4 pa-4 bg-success text-white" tag="h2">
      {{ t('features.title') }}
    </v-card-title>
    <v-card-text class="pa-6">
      <v-list>
        <v-list-item v-for="feature in supportedFeatures" :key="feature.key">
          <template #prepend>
            <v-icon :icon="feature.icon" color="success" />
          </template>
          <v-list-item-title>
            {{ t(`features.list.${feature.key}.title`) }}
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ t(`features.list.${feature.key}.description`) }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>

<i18n lang="yaml">
ja:
  getting_started:
    title: はじめに
    meta_description: Drop Compress Imageの使い方とインストール方法
  title: Drop Compress Image - はじめに
  subtitle: 高品質画像変換ツールの使い方をご紹介します
  installation:
    title: インストール
    step1_title: ダウンロード
    step1_desc: 公式サイトから最新版をダウンロードしてください
    step2_title: インストール
    step2_desc: ダウンロードしたファイルを実行してインストールを開始
    step3_title: 完了
    step3_desc: インストール完了後、アプリを起動してください
  usage:
    title: 使用方法
    methods:
      drag_drop:
        title: ドラッグ&ドロップ
        description: ファイルをアプリにドラッグ&ドロップするだけで変換開始
      file_menu:
        title: ファイルメニュー
        description: メニューバーからファイルを選択して変換
      batch_convert:
        title: 一括変換
        description: 複数ファイルを一度に変換可能
      settings:
        title: 設定
        description: 品質や形式などの詳細設定が可能
  features:
    title: 対応機能
    list:
      webp_support:
        title: WebP対応
        description: 高品質でファイルサイズの小さいWebP形式に変換
      avif_support:
        title: AVIF対応
        description: 次世代画像形式AVIFでさらに高い圧縮率を実現
      jxl_support:
        title: JPEG XL対応
        description: 最新のJPEG XL形式で最高品質の画像変換
      batch_processing:
        title: 一括処理
        description: 複数ファイルを同時に効率的に処理
      quality_control:
        title: 品質制御
        description: 圧縮品質を細かく調整可能
      cross_platform:
        title: クロスプラットフォーム
        description: Windows、macOSで動作

en:
  getting_started:
    title: Getting Started
    meta_description: How to install and use Drop Compress Image
  title: Drop Compress Image - Getting Started
  subtitle: Learn how to use this high-quality image conversion tool
  installation:
    title: Installation
    step1_title: Download
    step1_desc: Download the latest version from the official website
    step2_title: Install
    step2_desc: Run the downloaded file to start installation
    step3_title: Complete
    step3_desc: Launch the app after installation is complete
  usage:
    title: Usage
    methods:
      drag_drop:
        title: Drag & Drop
        description: Simply drag and drop files to the app to start conversion
      file_menu:
        title: File Menu
        description: Select files from the menu bar for conversion
      batch_convert:
        title: Batch Convert
        description: Convert multiple files at once
      settings:
        title: Settings
        description: Configure quality and format settings
  features:
    title: Supported Features
    list:
      webp_support:
        title: WebP Support
        description: Convert to high-quality, small file size WebP format
      avif_support:
        title: AVIF Support
        description: Achieve higher compression rates with next-gen AVIF format
      jxl_support:
        title: JPEG XL Support
        description: Best quality image conversion with latest JPEG XL format
      batch_processing:
        title: Batch Processing
        description: Process multiple files simultaneously and efficiently
      quality_control:
        title: Quality Control
        description: Fine-tune compression quality settings
      cross_platform:
        title: Cross Platform
        description: Works on Windows and macOS
</i18n>
