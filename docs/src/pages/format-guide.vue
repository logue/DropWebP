<script setup lang="ts">
import avifLogo from '@/assets/Avif-logo-rgb.svg';
import jxlLogo from '@/assets/JPEG_XL_logo.svg';
import webpLogo from '@/assets/WebPLogo.svg';

const { t } = useI18n();
// メタデータ設定
useHead({
  title: t('title'),
  meta: [{ name: 'description', content: t('meta_description') }]
});
</script>

<template>
  <h2 class="h2 mb-3">{{ t('title') }}</h2>
  <v-card class="mb-6" tag="section">
    <v-card-title tag="h3">
      {{ t('webp.title') }}
    </v-card-title>
    <v-card-text>
      <div class="d-flex justify-center mb-4">
        <div class="mr-4">
          <p>{{ t('webp.description') }}</p>
        </div>
        <v-img :src="webpLogo" alt="WebP Logo" width="256" />
      </div>
    </v-card-text>
  </v-card>
  <v-card class="mb-6" tag="section">
    <v-card-title tag="h3">
      {{ t('avif.title') }}
    </v-card-title>
    <v-card-text>
      <div class="d-flex justify-center mb-4">
        <div class="mr-4">
          <p>{{ t('avif.description[0]') }}</p>
          <p>{{ t('avif.description[1]') }}</p>
        </div>
        <v-img :src="avifLogo" alt="AVIF Logo" width="256" />
      </div>
    </v-card-text>
  </v-card>
  <v-card class="mb-6" tag="section">
    <v-card-title tag="h3">
      {{ t('jxl.title') }}
    </v-card-title>
    <v-card-text>
      <div class="d-flex justify-center mb-4">
        <div class="mr-4">
          <p>{{ t('jxl.description[0]') }}</p>
          <p>{{ t('jxl.description[1]') }}</p>
          <p>{{ t('jxl.description[2]') }}</p>
        </div>
        <v-img :src="jxlLogo" alt="JPEG XL Logo" width="256" />
      </div>
    </v-card-text>
  </v-card>
</template>

<i18n lang="yaml">
en:
  title: Format Guide
  meta_description: Guide to image formats supported by Drop Compress Image.
  webp:
    title: WebP
    description: WebP is a modern image format that provides superior lossless and lossy compression for images on the web. It is developed by Google and is widely supported across browsers and platforms.
  avif:
    title: AVIF (AV1 Image File Format)
    description:
      - AVIF is a next-generation image format based on the AV1 video codec.
      - It offers excellent compression efficiency and image quality, making it ideal for web use. AVIF is supported by major browsers and is gaining popularity.
  jxl:
    title: JPEG XL
    description:
      - JPEG XL is a modern image format designed to be a successor to JPEG.
      - It provides better compression and quality, especially for high-resolution images.
      - JPEG XL supports both lossy and lossless compression and is optimized for web performance.
fr:
  title: Guide des formats
  meta_description: Guide des formats d'image pris en charge par Drop Compress Image.
  webp:
    title: WebP
    description: WebP est un format d'image moderne qui offre une compression sans perte et avec perte supérieure pour les images sur le web. Il est développé par Google et est largement pris en charge par les navigateurs et les plateformes.
  avif:
    title: AVIF (AV1 Image File Format)
    description:
      - AVIF est un format d'image de nouvelle génération basé sur le codec vidéo AV1.
      - Il offre une excellente efficacité de compression et une qualité d'image, ce qui le rend idéal pour une utilisation sur le web. AVIF est pris en charge par les principaux navigateurs et gagne en popularité.
  jxl:
    title: JPEG XL
    description:
      - JPEG XL est un format d'image moderne conçu pour être le successeur de JPEG.
      - Il offre une meilleure compression et qualité, en particulier pour les images haute résolution.
      - JPEG XL prend en charge la compression avec et sans perte et est optimisé pour les performances sur le web.
ja:
  title: 対応フォーマットガイド
  meta_description: Drop Compress Imageでサポートされている画像フォーマットのガイド。
  webp:
    title: WebP
    description: WebPは、ウェブ上の画像に対して優れた可逆圧縮と非可逆圧縮を提供するモダンな画像フォーマットです。Googleによって開発され、ブラウザやプラットフォームで広くサポートされています。
  avif:
    title: AVIF (AV1 Image File Format)
    description:
      - AVIFは、AV1ビデオコーデックに基づく次世代の画像フォーマットです。
      - 優れた圧縮効率と画像品質を提供し、ウェブでの使用に最適です。AVIFは主要なブラウザでサポートされており、人気が高まっています。
  jxl:
    title: JPEG XL
    description:
      - JPEG XLは、JPEGの後継として設計されたモダンな画像フォーマットです。
      - 特に高解像度画像に対して、より優れた圧縮と品質を提供します。
      - JPEG XLは可逆圧縮と非可逆圧縮の両方をサポートしており、ウェブパフォーマンスに最適化されています。
ko:
  title: 포맷 가이드
  meta_description: Drop Compress Image에서 지원하는 이미지 포맷 가이드.
  webp:
    title: WebP
    description: WebP는 웹상의 이미지에 대해 우수한 무손실 및 손실 압축을 제공하는 현대적인 이미지 포맷입니다. Google에서 개발했으며 브라우저와 플랫폼에서 널리 지원됩니다.
  avif:
    title: AVIF (AV1 Image File Format)
    description:
      - AVIF는 AV1 비디오 코덱을 기반으로 하는 차세대 이미지 포맷입니다.
      - 우수한 압축 효율성과 이미지 품질을 제공하여 웹 사용에 이상적입니다. AVIF는 주요 브라우저에서 지원되며 인기를 얻고 있습니다.
  jxl:
    title: JPEG XL
    description:
      - JPEG XL은 JPEG의 후속으로 설계된 현대적인 이미지 포맷입니다.
      - 특히 고해상도 이미지에 대해 더 나은 압축 및 품질을 제공합니다.
      - JPEG XL은 무손실 및 손실 압축을 모두 지원하며 웹 성능에 최적화되어 있습니다.
zhHans:
  title: 格式指南
  meta_description: Drop Compress Image 支持的图像格式指南。
  webp:
    title: WebP
    description: WebP 是一种现代图像格式，为网络上的图像提供卓越的无损和有损压缩。它由 Google 开发，并在浏览器和平台上得到广泛支持。
  avif:
    title: AVIF (AV1 图像文件格式)
    description:
      - AVIF 是一种基于 AV1 视频编解码器的下一代图像格式。
      - 它提供出色的压缩效率和图像质量，非常适合网络使用。AVIF 得到主要浏览器的支持，且越来越受欢迎。
  jxl:
    title: JPEG XL
    description:
      - JPEG XL 是一种现代图像格式，设计为 JPEG 的继任者。
      - 它提供更好的压缩和质量，特别是对于高分辨率图像。
      - JPEG XL 支持有损和无损压缩，并针对网络性能进行了优化。
zhHant:
  title: 格式指南
  meta_description: Drop Compress Image 支援的圖像格式指南。
  webp:
    title: WebP
    description: WebP 是一種現代圖像格式，為網路上的圖像提供卓越的無損和有損壓縮。它由 Google 開發，並在瀏覽器和平台上得到廣泛支援。
  avif:
    title: AVIF (AV1 圖像檔案格式)
    description:
      - AVIF 是一種基於 AV1 視頻編解碼器的下一代圖像格式。
      - 它提供出色的壓縮效率和圖像質量，非常適合網路使用。AVIF 得到主要瀏覽器的支援，且越來越受歡迎。
  jxl:
    title: JPEG XL
    description:
      - JPEG XL 是一種現代圖像格式，設計為 JPEG 的繼任者。
      - 它提供更好的壓縮和質量，特別是對於高解析度圖像。
      - JPEG XL 支援有損和無損壓縮，並針對網路效能進行了優化。
</i18n>
