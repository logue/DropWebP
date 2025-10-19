<script setup lang="ts">
import logo from '@/assets/logo.png';
import ogp from '@/assets/ogp.png';

const { t, locale } = useI18n();
const localePath = useLocalePath();

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
  },
  {
    icon: 'mdi-earth',
    key: 'i18n'
  },
  {
    icon: 'mdi-theme-light-dark',
    key: 'dark_mode'
  },
  {
    icon: 'mdi-clipboard-outline',
    key: 'paste'
  }
];

const urlPrefix = `https://github.com/logue/DropWebP/releases/download/${version}/drop-compress-image_${version}_`;

// サイトのベースURL
const baseUrl = 'https://logue.dev';
const sitePath = '/DropWebP';
const currentUrl = computed(() => {
  const path = locale.value === 'ja' ? '' : `/${locale.value}`;
  return `${baseUrl}${sitePath}${path}`;
});

// OGP画像（ロゴ）- ogpはすでにbasePathを含むので、ドメインのみ追加
const ogImage = `${baseUrl}${ogp}`;

// hreflangタグを生成
useHead(useLocaleHead());

// SEO メタデータ
useSeoMeta({
  title: computed(() => `Drop Compress Image - ${t('subtitle')}`),
  ogSiteName: 'Drop Compress Image',
  description: computed(() => t('information[0]')),
  ogTitle: 'Drop Compress Image',
  ogDescription: computed(() => t('subtitle')),
  ogImage: ogImage,
  ogUrl: currentUrl,
  ogType: 'website',
  ogLocale: computed(() => {
    const localeMap: Record<string, string> = {
      ja: 'ja_JP',
      en: 'en_US',
      fr: 'fr_FR',
      ko: 'ko_KR',
      zhHans: 'zh_CN',
      zhHant: 'zh_TW'
    };
    return localeMap[locale.value] || 'en_US';
  }),
  twitterCard: 'summary_large_image',
  twitterTitle: 'Drop Compress Image',
  twitterDescription: computed(() => t('subtitle')),
  twitterImage: ogImage
});

// JSON-LD 構造化データ
useHead({
  script: [
    {
      type: 'application/ld+json',
      innerHTML: computed(() => {
        const languageMap: Record<string, string> = {
          ja: 'ja',
          en: 'en',
          fr: 'fr',
          ko: 'ko',
          zhHans: 'zh-CN',
          zhHant: 'zh-TW'
        };

        return JSON.stringify({
          '@context': 'https://schema.org',
          '@type': 'SoftwareApplication',
          name: 'Drop Compress Image',
          applicationCategory: 'DesignApplication',
          operatingSystem: 'Windows 11, macOS',
          offers: {
            '@type': 'Offer',
            price: '0',
            priceCurrency: 'USD'
          },
          description: t('information[0]'),
          url: `${baseUrl}${sitePath}`,
          image: ogImage,
          softwareVersion: version,
          releaseNotes: `https://github.com/logue/DropWebP/releases/tag/${version}`,
          downloadUrl: `https://github.com/logue/DropWebP/releases/download/${version}/`,
          author: {
            '@type': 'Person',
            name: 'Logue',
            url: 'https://github.com/logue'
          },
          featureList: [
            t('features.multiple_formats.title'),
            t('features.high_speed.title'),
            t('features.drag_drop.title'),
            t('features.i18n.title'),
            t('features.dark_mode.title'),
            t('features.paste.title')
          ],
          screenshot: ogImage,
          inLanguage: languageMap[locale.value] || 'en'
        });
      })
    }
  ]
});
</script>

<template>
  <v-card class="mb-6 bg-transparent" tag="section" elevation="0">
    <v-img :src="logo" alt="Drop Compress Image Logo" max-width="256" class="mx-auto mb-4" />
    <v-card-title class="text-h4 text-center pa-3" tag="h2">Drop Compress Image</v-card-title>
    <v-card-text class="text-center">
      <p class="text-h6 mb-4">{{ t('subtitle') }}</p>
      <p class="mb-6">{{ t('information[0]') }}</p>
      <p class="mb-6">{{ t('information[1]') }}</p>
    </v-card-text>
    <v-card-actions class="justify-center">
      <v-btn
        disabled
        :to="localePath('getting-started')"
        class="ma-4"
        color="primary"
        prepend-icon="mdi-rocket"
        size="large"
        variant="elevated"
      >
        {{ t('start_button') }}
      </v-btn>
    </v-card-actions>
  </v-card>

  <v-card class="mb-6 bg-transparent" tag="section" elevation="0">
    <v-card-title class="text-h5 text-center pa-6" tag="h2">
      {{ t('download.download') }}
    </v-card-title>
    <!-- Download Buttons -->
    <v-card-actions class="justify-center">
      <v-btn
        :href="`${urlPrefix}x64_en-US.msi`"
        class="mr-1"
        prepend-icon-color="blue"
        prepend-icon="mdi-microsoft-windows"
        size="large"
        stacked
        variant="elevated"
      >
        {{ t('download.windows') }}
        <br />
        <small class="text-secondary">({{ t('download.window_requirement') }})</small>
      </v-btn>
      <v-btn
        :href="`${urlPrefix}aarch64.dmg`"
        class="ml-1"
        prepend-icon="mdi-apple"
        size="large"
        stacked
        variant="elevated"
      >
        {{ t('download.macos') }}
        <br />
        <small class="text-secondary">({{ t('download.macos_requirement') }})</small>
      </v-btn>
    </v-card-actions>
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
    download: Download
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
    dark_mode:
      title: Dark Mode
      description: Enjoy a comfortable viewing experience with dark mode support.
    i18n:
      title: Internationalization
      description: Supports multiple languages for a global user experience.
    paste:
      title: Paste from Clipboard
      description: Directly paste images from clipboard for quick conversion. (Ctrl (⌘) + V)
fr:
  subtitle: Le convertisseur d'images moderne
  start_button: Commencer
  information:
    - Drop Compress Image est un outil GUI puissant, rapide et polyvalent pour convertir vos images en formats de nouvelle génération. Conçu pour la performance, il utilise des codecs modernes pour vous offrir le meilleur en termes de vitesse, de qualité et de taille de fichier. Dites adieu aux formats obsolètes et bonjour à l'avenir des images web !
    - Ce projet a été créé pour fournir une solution de conversion complète, prenant en charge une large gamme de formats d'entrée et exportant vers des formats très efficaces comme AVIF, JPEG XL et WebP.
  download:
    download: Télécharger
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
    dark_mode:
      title: Mode Sombre
      description: Profitez d'une expérience visuelle confortable avec la prise en charge du mode sombre.
    i18n:
      title: Internationalisation
      description: Prend en charge plusieurs langues pour une expérience utilisateur mondiale.
    paste:
      title: Coller depuis le presse-papiers
      description: Collez directement des images depuis le presse-papiers pour une conversion rapide. (Ctrl (⌘) + V)
ja:
  subtitle: モダンな画像変換ツール
  start_button: はじめに
  information:
    - Drop Compress Imageは、次世代フォーマットへの画像変換を強力かつ高速に行う多機能なGUIツールです。パフォーマンスを重視して設計されており、最新のコーデックを活用して、速度、品質、ファイルサイズのすべてにおいて最高の体験を提供します。古いフォーマットに別れを告げ、ウェブ画像の未来へようこそ！
    - このプロジェクトは、幅広い入力フォーマットに対応し、AVIF、JPEG XL、WebPなどの高効率フォーマットへのエクスポートをサポートする包括的な変換ソリューションを提供するために作成されました。
  download:
    download: ダウンロード
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
    dark_mode:
      title: ダークモード
      description: ダークモード対応で快適な閲覧体験を提供。
    drag_drop:
      title: ドラッグ&ドロップ
      description: 簡単な操作で画像を一括変換可能。
    i18n:
      title: 多言語対応
      description: 複数言語に対応し、グローバルなユーザー体験を提供。
    paste:
      title: クリップボードから貼り付け
      description: クリップボードから直接画像を貼り付けて素早く変換。(Ctrl (⌘) + V)
ko:
  subtitle: 모던 이미지 변환기
  start_button: 시작하기
  information:
    - Drop Compress Image는 차세대 포맷으로 이미지를 변환하는 강력하고 빠르며 다재다능한 GUI 도구입니다. 성능을 염두에 두고 설계되었으며 최신 코덱을 활용하여 속도, 품질 및 파일 크기 측면에서 최고의 경험을 제공합니다. 구식 포맷과 작별하고 웹 이미지의 미래에 오신 것을 환영합니다!
    - 이 프로젝트는 광범위한 입력 포맷을 지원하고 AVIF, JPEG XL 및 WebP와 같은 고효율 포맷으로 내보내는 포괄적인 변환 솔루션을 제공하기 위해 만들어졌습니다.
  download:
    download: 다운로드
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
    dark_mode:
      title: 다크 모드
      description: 다크 모드 지원으로 편안한 시청 경험 제공.
    i18n:
      title: 다국어 지원
      description: 글로벌 사용자 경험을 위한 다국어 지원.
    paste:
      title: 클립보드에서 붙여넣기
      description: 클립보드에서 직접 이미지를 붙여넣어 빠르게 변환. (Ctrl (⌘) + V)
zhHant:
  subtitle: 現代圖像轉換器
  start_button: 入門
  information:
    - Drop Compress Image 是一款強大、快速且多功能的 GUI 工具，可將您的圖像轉換為新一代格式。它以性能為設計理念，利用現代編解碼器為您帶來速度、質量和文件大小方面的最佳體驗。告別過時的格式，迎接網絡圖像的未來！
    - 該項目旨在提供全面的轉換解決方案，支持廣泛的輸入格式，並導出高效的格式，如 AVIF、JPEG XL 和 WebP。
  download:
    download: 下載
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
    dark_mode:
      title: 暗黑模式
      description: 暗黑模式支援，享受舒適的瀏覽體驗。
    i18n:
      title: 多語言支援
      description: 支援多種語言以提供全球用戶體驗。
    paste:
      title: 從剪貼簿貼上
      description: 直接從剪貼簿貼上圖像以快速轉換。(Ctrl (⌘) + V)
zhHans:
  subtitle: 现代图像转换器
  start_button: 入门
  information:
    - Drop Compress Image 是一款强大、快速且多功能的 GUI 工具，可将您的图像转换为新一代格式。它以性能为设计理念，利用现代编解码器为您带来速度、质量和文件大小方面的最佳体验。告别过时的格式，迎接网络图像的未来！
    - 该项目旨在提供全面的转换解决方案，支持广泛的输入格式，并导出高效的格式，如 AVIF、JPEG XL 和 WebP。
  download:
    download: 下载安装
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
    dark_mode:
      title: 暗黑模式
      description: 暗黑模式支持，享受舒适的浏览体验。
    i18n:
      title: 多语言支持
      description: 支持多种语言以提供全球用户体验。
    paste:
      title: 从剪贴板粘贴
      description: 直接从剪贴板粘贴图像以快速转换。(Ctrl (⌘) + V)
</i18n>
