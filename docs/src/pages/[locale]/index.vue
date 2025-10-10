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

const baseUrl = `https://github.com/logue/DropWebP/releases/download/${version}/`;
</script>

<template>
  <v-card class="mb-6 bg-transparent" tag="section" elevation="0">
    <v-img :src="logo" alt="Drop Compress Image Logo" max-width="256" class="mx-auto mb-4" />
    <v-card-title class="text-h4 text-center pa-3" tag="h2">Drop Compress Image</v-card-title>
    <v-card-text class="text-center">
      <p class="text-h6 mb-4">{{ t('subtitle') }}</p>
      <p class="mb-6">{{ t('information[0]') }}</p>
      <p class="mb-6">{{ t('information[1]') }}</p>
      <!-- Download Buttons -->
      <div class="d-flex justify-center flex-wrap">
        <v-btn
          :href="`${baseUrl}drop-compress-image_${version}_x64_en-US.msi`"
          class="mr-1"
          prepend-icon="mdi-microsoft-windows"
          size="large"
          stacked
        >
          {{ t('download.windows') }}
          <br />
          <small class="text-secondary">({{ t('download.window_requirement') }})</small>
        </v-btn>
        <v-btn
          :href="`drop-compress-image_${version}_aarch64.dmg`"
          class="ml-1"
          prepend-icon="mdi-apple"
          size="large"
          stacked
        >
          {{ t('download.macos') }}
          <br />
          <small class="text-secondary">({{ t('download.macos_requirement') }})</small>
        </v-btn>
      </div>
      <v-btn class="ma-4" color="primary" size="large" :to="r('/getting-started')">
        <v-icon start>mdi-rocket</v-icon>
        {{ t('start_button') }}
      </v-btn>
    </v-card-text>
  </v-card>

  <v-card class="mb-6 bg-transparent" tag="section" elevation="0">
    <v-card-title class="text-h5 text-center pa-6" tag="h2">{{ t('features.title') }}</v-card-title>
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
    </v-card-text>
  </v-card>
</template>

<i18n lang="yaml">
en:
  subtitle: The Modern Image Converter
  information:
    - Drop Compress Image is a powerful, fast, and versatile GUI tool for converting your images into next-generation formats. Built with performance in mind, it leverages modern codecs to bring you the best in speed, quality, and file size. Say goodbye to outdated formats and hello to the future of web images!
    - This project was created to provide a comprehensive conversion solution, supporting a wide range of input formats and exporting to highly efficient formats like AVIF, JPEG XL, and WebP.
  start_button: Get Started
  download:
    windows: Download for Windows
    window_requirement: Windows 11 or later
    macos: Download for MacOS
    macos_requirement: M1 or later
  features:
    title: Features
    multiple_formats:
      title: Multiple Format Support
      description: Support for modern image formats like WebP, AVIF, JPEG XL.
    high_speed:
      title: High-Speed Conversion
      description: Fast image processing with Rust-based engine.
    drag_drop:
      title: Drag & Drop
      description: Easy batch conversion with simple operations.
fr:
  subtitle: Le convertisseur d'images moderne
  start_button: Commencer
  information:
    - Drop Compress Image est un outil GUI puissant, rapide et polyvalent pour convertir vos images en formats de nouvelle génération. Conçu pour la performance, il utilise des codecs modernes pour vous offrir le meilleur en termes de vitesse, de qualité et de taille de fichier. Dites adieu aux formats obsolètes et bonjour à l'avenir des images web !
    - Ce projet a été créé pour fournir une solution de conversion complète, prenant en charge une large gamme de formats d'entrée et exportant vers des formats très efficaces comme AVIF, JPEG XL et WebP.
  download:
    windows: Télécharger pour Windows
    window_requirement: Windows 11 ou version ultérieure
    macos: Télécharger pour MacOS
    macos_requirement: M1 ou version ultérieure
  features:
    title: Fonctionnalités
    multiple_formats:
      title: Prise en charge de plusieurs formats
      description: Prise en charge des formats d'image modernes tels que WebP, AVIF, JXL.
    high_speed:
      title: Conversion haute vitesse
      description: Traitement rapide des images avec un moteur basé sur Rust.
    drag_drop:
      title: Glisser-Déposer
      description: Conversion par lots facile avec des opérations simples.
ja:
  subtitle: モダンな画像変換ツール
  start_button: はじめに
  information:
    - Drop Compress Imageは、次世代フォーマットへの画像変換を強力かつ高速に行う多機能なGUIツールです。パフォーマンスを重視して設計されており、最新のコーデックを活用して、速度、品質、ファイルサイズのすべてにおいて最高の体験を提供します。古いフォーマットに別れを告げ、ウェブ画像の未来へようこそ！
    - このプロジェクトは、幅広い入力フォーマットに対応し、AVIF、JPEG XL、WebPなどの高効率フォーマットへのエクスポートをサポートする包括的な変換ソリューションを提供するために作成されました。
  download:
    windows: Windows版をダウンロード
    window_requirement: Windows 11以降
    macos: MacOS版をダウンロード
    macos_requirement: M1以降
  features:
    title: 機能
    multiple_formats:
      title: 複数形式対応
      description: WebP、AVIF、JPEG XLなどの最新画像形式に対応。
    high_speed:
      title: 高速変換
      description: Rust基盤で高速な画像処理を実現。
    drag_drop:
      title: ドラッグ&ドロップ
      description: 簡単な操作で画像を一括変換可能。
ko:
  subtitle: 모던 이미지 변환기
  start_button: 시작하기
  information:
    - Drop Compress Image는 차세대 포맷으로 이미지를 변환하는 강력하고 빠르며 다재다능한 GUI 도구입니다. 성능을 염두에 두고 설계되었으며 최신 코덱을 활용하여 속도, 품질 및 파일 크기 측면에서 최고의 경험을 제공합니다. 구식 포맷과 작별하고 웹 이미지의 미래에 오신 것을 환영합니다!
    - 이 프로젝트는 광범위한 입력 포맷을 지원하고 AVIF, JPEG XL 및 WebP와 같은 고효율 포맷으로 내보내는 포괄적인 변환 솔루션을 제공하기 위해 만들어졌습니다.
  download:
    windows: Windows용 다운로드
    window_requirement: Windows 11 이상
    macos: MacOS용 다운로드
    macos_requirement: M1 이상
  features:
    title: 기능
    multiple_formats:
      title: 다중 형식 지원
      description: WebP, AVIF, JPEG XL 등 최신 이미지 형식 지원.
    high_speed:
      title: 고속 변환
      description: Rust 기반의 고속 이미지 처리.
    drag_drop:
      title: 드래그 & 드롭
      description: 간단한 조작으로 이미지 일괄 변환 가능.
zhHant:
  subtitle: 現代圖像轉換器
  start_button: 入門
  information:
    - Drop Compress Image 是一款強大、快速且多功能的 GUI 工具，可將您的圖像轉換為新一代格式。它以性能為設計理念，利用現代編解碼器為您帶來速度、質量和文件大小方面的最佳體驗。告別過時的格式，迎接網絡圖像的未來！
    - 該項目旨在提供全面的轉換解決方案，支持廣泛的輸入格式，並導出高效的格式，如 AVIF、JPEG XL 和 WebP。
  download:
    windows: 下載 Windows 版
    window_requirement: Windows 11 或更新版本
    macos: 下載 MacOS 版
    macos_requirement: M1 或更新版本
  features:
    title: 功能
    multiple_formats:
      title: 多格式支援
      description: 支援 WebP、AVIF、JPEG XL 等現代圖像格式。
    high_speed:
      title: 高速轉換
      description: 使用基於 Rust 的引擎進行快速圖像處理。
    drag_drop:
      title: 拖放功能
      description: 通過簡單操作輕鬆進行批量轉換。
zhHans:
  subtitle: 现代图像转换器
  start_button: 入门
  information:
    - Drop Compress Image 是一款强大、快速且多功能的 GUI 工具，可将您的图像转换为新一代格式。它以性能为设计理念，利用现代编解码器为您带来速度、质量和文件大小方面的最佳体验。告别过时的格式，迎接网络图像的未来！
    - 该项目旨在提供全面的转换解决方案，支持广泛的输入格式，并导出高效的格式，如 AVIF、JPEG XL 和 WebP。
  download:
    windows: 下载安装 Windows 版
    window_requirement: Windows 11 或更新版本
    macos: 下载安装 MacOS 版
    macos_requirement: M1 或更新版本
  features:
    title: 功能
    multiple_formats:
      title: 多格式支持
      description: 支持 WebP、AVIF、JPEG XL 等现代图像格式。
    high_speed:
      title: 高速转换
      description: 使用基于 Rust 的引擎进行快速图像处理。
    drag_drop:
      title: 拖放功能
      description: 通过简单操作轻松进行批量转换。
</i18n>
