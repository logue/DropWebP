<script setup lang="ts">
import { unref } from 'vue';
import avif from '@/assets/Avif-logo-rgb.svg';
import jxl from '@/assets/JPEG_XL_logo.svg';
import jpeg from '@/assets/Mozjpeg_logotype.svg';
import webp from '@/assets/WebPLogo.svg';
import logo from '@/assets/logo.png';
import ogp from '@/assets/ogp.png';
import png from '@/assets/zopfli-logo.png';

const { locale, rt, t, tm } = useI18n();
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

const formats = [
  { key: 'webp', logo: webp },
  { key: 'avif', logo: avif },
  { key: 'jxl', logo: jxl },
  { key: 'png', logo: png },
  { key: 'jpeg', logo: jpeg }
];

// 言語リスト定義
const languages = [
  { code: 'en', name: '🇺🇸 English' },
  { code: 'ja', name: '🇯🇵 日本語' },
  { code: 'fr', name: '🇫🇷 Français' },
  { code: 'ko', name: '🇰🇷 한국어' },
  { code: 'zhHans', name: '🇨🇳 简体中文' },
  { code: 'zhHant', name: '🇹🇼 繁體中文' }
];

const urlPrefix = `https://github.com/logue/DropWebP/releases/download/${version}/drop-compress-image_${version}_`;

// サイトのベースURL - 本番環境ではhttps://logue.devを使用
const baseUrl = 'https://logue.dev';
const sitePath = '/DropWebP';

const currentUrl = computed(() => {
  const path = locale.value === 'en' ? '' : `/${locale.value}`;
  return `${baseUrl}${sitePath}${path}`;
});

// OGP画像（ロゴ）- 完全なURLを構築
const ogImage = computed(() => {
  // ogp変数が相対パスの場合、適切にベースURLと結合
  if (ogp.startsWith('/DropWebP/')) {
    return `${baseUrl}${ogp}`;
  } else if (ogp.startsWith('/')) {
    return `${baseUrl}${sitePath}${ogp}`;
  } else {
    return `${baseUrl}${sitePath}/${ogp}`;
  }
});

// hreflangタグを手動で生成（デプロイ環境での重複URL問題を回避）
const hreflangLinks = computed(() => {
  const links = [];

  // x-default（英語）
  links.push({
    rel: 'alternate',
    hreflang: 'x-default',
    href: `${baseUrl}${sitePath}/`
  });

  // 各言語
  languages.forEach(lang => {
    if (lang.code === 'en') {
      links.push({
        rel: 'alternate',
        hreflang: 'en',
        href: `${baseUrl}${sitePath}/`
      });
    } else {
      links.push({
        rel: 'alternate',
        hreflang: lang.code,
        href: `${baseUrl}${sitePath}/${lang.code}/`
      });
    }
  });

  return links;
});

useHead({
  link: hreflangLinks
});

// SEO メタデータ
useSeoMeta({
  title: computed(() => `Drop Compress Image - ${t('lead.subtitle')}`),
  ogSiteName: 'Drop Compress Image',
  description: computed(() => t('lead.description[0]')),
  ogTitle: 'Drop Compress Image',
  ogDescription: computed(() => t('lead.subtitle')),
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
  twitterDescription: computed(() => t('lead.subtitle')),
  twitterImage: ogImage
});

// JSON-LD 構造化データ
const jsonLdData = computed(() => {
  const languageMap: Record<string, string> = {
    ja: 'ja',
    en: 'en',
    fr: 'fr',
    ko: 'ko',
    zhHans: 'zh-CN',
    zhHant: 'zh-TW'
  };

  return {
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
    description: unref(t('lead.description[0]')),
    url: `${unref(baseUrl)}/DropWebP`,
    image: unref(ogImage),
    softwareVersion: unref(version),
    releaseNotes: `https://github.com/logue/DropWebP/releases/tag/${unref(version)}`,
    downloadUrl: `https://github.com/logue/DropWebP/releases/download/${unref(version)}/`,
    author: {
      '@type': 'Person',
      name: 'Logue',
      url: 'https://github.com/logue'
    },
    featureList: [
      unref(t('features.multiple_formats.title')),
      unref(t('features.high_speed.title')),
      unref(t('features.drag_drop.title')),
      unref(t('features.i18n.title')),
      unref(t('features.dark_mode.title')),
      unref(t('features.paste.title'))
    ],
    screenshot: unref(ogImage),
    inLanguage: languageMap[locale.value] || 'en'
  };
});

useHead({
  script: [
    {
      type: 'application/ld+json',
      innerHTML: () => JSON.stringify(jsonLdData.value)
    }
  ]
});
</script>

<template>
  <v-card class="mb-6 bg-transparent mx-auto" flat tag="section" max-width="960">
    <v-img :src="logo" alt="Drop Compress Image Logo" max-width="256" class="mx-auto mb-4" />
    <v-card-title class="text-h4 text-center pa-3" tag="h2">Drop Compress Image</v-card-title>
    <v-card-subtitle class="text-center pb-4">{{ t('lead.subtitle') }}</v-card-subtitle>
    <v-card-text class="text-center">
      <!-- Language Links -->
      <v-chip-group class="flex justify-center mb-6">
        <v-chip
          v-for="lang in languages"
          :key="lang.code"
          :hreflang="lang.code"
          :to="localePath('/', lang.code as any)"
          :variant="locale === lang.code ? 'elevated' : 'outlined'"
          :color="locale === lang.code ? 'primary' : 'default'"
          class="block mx-auto"
          rel="alternate"
          size="small"
        >
          {{ lang.name }}
        </v-chip>
      </v-chip-group>
      <p v-for="description in tm(`lead.description`)" :key="description">
        {{ rt(description) }}
      </p>
    </v-card-text>
    <!--v-card-actions class="justify-center">
      <v-btn
        disabled
        :to="localePath('getting-started')"
        class="ma-4"
        color="primary"
        prepend-icon="mdi-rocket"
        size="large"
        variant="elevated"
      >
        {{ t('lead.start_button') }}
      </v-btn>
    </!v-card-actions-->
  </v-card>

  <v-card class="mb-6 bg-transparent" flat tag="section">
    <v-card-title class="text-h5 text-center" tag="h2">
      {{ t('download.download') }}
    </v-card-title>
    <v-card-subtitle class="text-center">
      <v-code>v.{{ version }}</v-code>
    </v-card-subtitle>
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

  <v-card class="mb-6 bg-transparent" flat tag="section">
    <v-card-title class="text-h5 text-center" tag="h2">{{ t('features.title') }}</v-card-title>
    <v-card-subtitle class="text-center">{{ t('features.subtitle') }}</v-card-subtitle>
    <v-card-text>
      <v-row class="mb-5">
        <v-col v-for="item in features" :key="item.key" cols="12" md="4">
          <v-card class="h-100">
            <v-card-text class="text-center">
              <v-icon :icon="item.icon" size="64" color="primary" class="mb-4" />
              <h3 class="text-h6 mb-2">{{ t(`features.${item.key}.title`) }}</h3>
              <p>
                {{ t(`features.${item.key}.description`) }}
              </p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>

  <v-card class="mb-6 bg-transparent" flat tag="section">
    <v-card-title class="text-h5 text-center" tag="h2">{{ t('format.title') }}</v-card-title>
    <v-card-subtitle class="text-center">{{ t('format.subtitle') }}</v-card-subtitle>
    <v-card-text>
      <v-row class="mb-5">
        <v-col v-for="item in formats" :key="item.key" cols="12" md="4">
          <v-card class="h-100">
            <v-img
              v-if="item.logo"
              :src="item.logo"
              :alt="t(`format.${item.key}.title`)"
              max-height="100"
              contain
              class="mx-auto mt-4"
            />
            <v-card-title class="text-h6 text-center mt-4" tag="h3">
              {{ t(`format.${item.key}.title`) }}
            </v-card-title>
            <v-card-text>
              <p v-for="description in tm(`format.${item.key}.description`)" :key="description">
                {{ rt(description) }}
              </p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-card-text>
    <v-card-actions class="justify-center">
      <v-btn
        :to="localePath('format-guide')"
        class="ma-4"
        color="primary"
        prepend-icon="mdi-book-open-page-variant"
        size="large"
        variant="elevated"
      >
        {{ t('format.more') }}
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<i18n lang="yaml">
en:
  lead:
    subtitle: The Modern Image Converter
    description:
      - Drop Compress Image is a powerful, fast, and versatile GUI tool for converting your images into next-generation formats. Built with performance in mind, it leverages modern codecs to bring you the best in speed, quality, and file size.
      - Say goodbye to outdated formats and hello to the future of web images!
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
    subtitle: Key Features of Drop Compress Image
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
  format:
    title: Supported Output Formats
    subtitle: These are the image formats supported by Drop Compress Image.
    more: Learn More
    webp:
      title: WebP
      description:
        - WebP is a modern image format that provides superior lossless and lossy compression for images on the web.
        - Developed by Google, it is widely supported across browsers and platforms.
    avif:
      title: AVIF
      description:
        - AVIF (AV1 Image File Format) is a next-generation image format based on the AV1 video codec.
        - It offers excellent compression efficiency and image quality, making it ideal for web use. AVIF is supported by major browsers and is gaining popularity.
    jxl:
      title: JPEG XL
      description:
        - JPEG XL is a modern image format designed as a successor to JPEG.
        - It provides better compression and quality, especially for high-resolution images.
        - JPEG XL supports both lossless and lossy compression. It is optimized for web performance.
    png:
      title: PNG (Zopfli Compression)
      description:
        - PNG (Portable Network Graphics) is a format that allows images to be saved "without any loss of quality."
        - Zopfli, used in this program, is a special compression technique created by Google to make PNGs "smaller."
        - It supports transparency and maintains high-quality images.
        - Zopfli is an algorithm for compressing PNG images more efficiently. It uses advanced compression techniques to reduce file size while preserving image quality.
    jpeg:
      title: JPEG (MozJPEG Compression)
      description:
        - JPEG is a widely used image format known for its lossy compression capabilities.
        - MozJPEG is an improved JPEG encoder developed by Mozilla that focuses on better compression and quality.
        - It achieves smaller file sizes while maintaining visual quality, making it ideal for web images.
        - MozJPEG incorporates advanced techniques to optimize JPEG images, resulting in faster loading times and reduced bandwidth usage.
fr:
  lead:
    subtitle: Le convertisseur d'images moderne
    start_button: Commencer
    description:
      - Drop Compress Image est un outil GUI puissant, rapide et polyvalent pour convertir vos images en formats de nouvelle génération. Conçu pour la performance, il utilise des codecs modernes pour vous offrir le meilleur en termes de vitesse, de qualité et de taille de fichier.
      - Dites adieu aux formats obsolètes et bonjour à l'avenir des images web !
      - Ce projet a été créé pour fournir une solution de conversion complète, prenant en charge une large gamme de formats d'entrée et exportant vers des formats très efficaces comme AVIF, JPEG XL et WebP.
  download:
    download: Télécharger
    windows: Télécharger pour Windows
    window_requirement: Windows 11 ou version ultérieure
    macos: Télécharger pour MacOS
    macos_requirement: M1 ou version ultérieure
  features:
    title: Fonctionnalités
    subtitle: Fonctionnalités clés de Drop Compress Image
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
  format:
    title: Formats de sortie pris en charge
    subtitle: Voici les formats d'image pris en charge par Drop Compress Image.
    more: En savoir plus
    webp:
      title: WebP
      description:
        - WebP est un format d'image moderne qui offre une compression sans perte et avec perte supérieure pour les images sur le web.
        - Développé par Google, il est largement pris en charge par les navigateurs et les plateformes.
    avif:
      title: AVIF
      description:
        - AVIF (AV1 Image File Format) est un format d'image de nouvelle génération basé sur le codec vidéo AV1.
        - Il offre une excellente efficacité de compression et une qualité d'image, ce qui le rend idéal pour une utilisation sur le web. AVIF est pris en charge par les principaux navigateurs et gagne en popularité.
    jxl:
      title: JPEG XL
      description:
        - JPEG XL est un format d'image moderne conçu comme successeur du JPEG.
        - Il offre une meilleure compression et qualité, en particulier pour les images haute résolution.
        - JPEG XL prend en charge à la fois la compression sans perte et avec perte. Il est optimisé pour les performances web.
    png:
      title: PNG (Compression Zopfli)
      description:
        - PNG (Portable Network Graphics) est un format qui permet de sauvegarder les images "sans aucune perte de qualité".
        - Zopfli, utilisé dans ce programme, est une technique de compression spéciale créée par Google pour rendre les PNG "plus petits".
        - Il prend en charge la transparence et maintient des images de haute qualité.
        - Zopfli est un algorithme de compression des images PNG plus efficace. Il utilise des techniques de compression avancées pour réduire la taille des fichiers tout en préservant la qualité de l'image.
    jpeg:
      title: JPEG (Compression MozJPEG)
      description:
        - JPEG est un format d'image largement utilisé, connu pour ses capacités de compression avec perte.
        - MozJPEG est un encodeur JPEG amélioré développé par Mozilla qui se concentre sur une meilleure compression et qualité.
        - Il permet d'obtenir des tailles de fichiers plus petites tout en maintenant une qualité visuelle, ce qui le rend idéal pour les images web.
        - MozJPEG intègre des techniques avancées pour optimiser les images JPEG, ce qui se traduit par des temps de chargement plus rapides et une réduction de l'utilisation de la bande passante.
ja:
  lead:
    subtitle: モダンな画像変換ツール
    start_button: はじめに
    description:
      - Drop Compress Imageは、次世代フォーマットへの画像変換を強力かつ高速に行う多機能なGUIツールです。パフォーマンスを重視して設計されており、最新のコーデックを活用して、速度、品質、ファイルサイズのすべてにおいて最高の体験を提供します。
      - 古いフォーマットに別れを告げ、ウェブ画像の未来へようこそ！
      - このプロジェクトは、幅広い入力フォーマットに対応し、AVIF、JPEG XL、WebPなどの高効率フォーマットへのエクスポートをサポートする包括的な変換ソリューションを提供するために作成されました。
  download:
    download: ダウンロード
    windows: Windows版をダウンロード
    window_requirement: Windows 11以降
    macos: MacOS版をダウンロード
    macos_requirement: M1以降
  features:
    title: 機能
    subtitle: Drop Compress Imageの主な機能
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
  format:
    title: 対応出力フォーマット
    subtitle: Drop Compress Imageでサポートされている画像フォーマットです。
    more: 更に詳しく見る
    webp:
      title: WebP
      description:
        - WebP（ウェッピー）は、ウェブ上の画像に対して優れた可逆圧縮と非可逆圧縮を提供するモダンな画像フォーマットです。
        - Googleによって開発され、ブラウザやプラットフォームで広くサポートされています。
    avif:
      title: AVIF
      description:
        - AVIF（AV1 Image File Format）は、AV1ビデオコーデックに基づく次世代の画像フォーマットです。
        - 優れた圧縮効率と画像品質を提供し、ウェブでの使用に最適です。AVIFは主要なブラウザでサポートされており、人気が高まっています。
    jxl:
      title: JPEG XL
      description:
        - JPEG XL（ジェイペグエクセル）は、JPEGの後継として設計されたモダンな画像フォーマットです。
        - 特に高解像度画像に対して、より優れた圧縮と品質を提供します。
        - JPEG XLは可逆圧縮と非可逆圧縮の両方をサポートしており、ウェブパフォーマンスに最適化されています。
    png:
      title: PNG (Zopfli圧縮)
      description:
        - PNG（Portable Network Graphics）は、画像を「画質を全く落とさずに」保存できる形式です。
        - 本プログラムで使用されているZopfli（ゾップフリ）とは、このPNGを「より小さく」するための、Googleが作った特別な圧縮技術です。
        - 透明度をサポートし、高品質の画像を保持します。
        - Zopfliは、PNG画像の圧縮を更に高効率で圧縮するためのアルゴリズムです。
    jpeg:
      title: JPEG (MozJPEG圧縮)
      description:
        - JPEG（Joint Photographic Experts Group）は、写真やリアルな画像に広く使用されている画像フォーマットです。
        - 本プログラムで使用されているMozJPEG（モズジェイペグ）とは、このJPEGを「より小さく」するための、Mozillaが開発した特別な圧縮技術です。
ko:
  lead:
    subtitle: 모던 이미지 변환기
    start_button: 시작하기
    description:
      - Drop Compress Image는 차세대 포맷으로 이미지를 변환하는 강력하고 빠르며 다재다능한 GUI 도구입니다. 성능을 염두에 두고 설계되었으며 최신 코덱을 활용하여 속도, 품질 및 파일 크기 측면에서 최고의 경험을 제공합니다.
      - 구식 포맷과 작별하고 웹 이미지의 미래에 오신 것을 환영합니다!
      - 이 프로젝트는 광범위한 입력 포맷을 지원하고 AVIF, JPEG XL 및 WebP와 같은 고효율 포맷으로 내보내는 포괄적인 변환 솔루션을 제공하기 위해 만들어졌습니다.
  download:
    download: 다운로드
    windows: Windows용 다운로드
    window_requirement: Windows 11 이상
    macos: MacOS용 다운로드
    macos_requirement: M1 이상
  features:
    title: 기능
    subtitle: Drop Compress Image의 주요 기능
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
  format:
    title: 지원되는 출력 형식
    subtitle: Drop Compress Image에서 지원하는 이미지 형식입니다.
    more: 자세히 알아보기
    webp:
      title: WebP
      description:
        - WebP는 웹의 이미지를 위한 우수한 무손실 및 손실 압축을 제공하는 최신 이미지 형식입니다.
        - Google에서 개발했으며 브라우저와 플랫폼에서 널리 지원됩니다.
    avif:
      title: AVIF
      description:
        - AVIF(AV1 Image File Format)는 AV1 비디오 코덱을 기반으로 하는 차세대 이미지 형식입니다.
        - 우수한 압축 효율성과 이미지 품질을 제공하여 웹 사용에 이상적입니다. AVIF는 주요 브라우저에서 지원되며 인기를 얻고 있습니다.
    jxl:
      title: JPEG XL
      description:
        - JPEG XL은 JPEG의 후속으로 설계된 최신 이미지 형식입니다.
        - 특히 고해상도 이미지에 대해 더 나은 압축 및 품질을 제공합니다.
        - JPEG XL은 무손실 및 손실 압축을 모두 지원합니다. 웹 성능에 최적화되어 있습니다.
    png:
      title: PNG (Zopfli 압축)
      description:
        - PNG(Portable Network Graphics)는 이미지를 "품질 손실 없이" 저장할 수 있는 형식입니다.
        - 이 프로그램에서 사용되는 Zopfli는 PNG를 "더 작게" 만들기 위해 Google이 만든 특별한 압축 기술입니다.
        - 투명도를 지원하며 고품질 이미지를 유지합니다.
        - Zopfli는 PNG 이미지를 보다 효율적으로 압축하기 위한 알고리즘입니다. 고급 압축 기술을 사용하여 파일 크기를 줄이면서 이미지 품질을 유지합니다.
    jpeg:
      title: JPEG (MozJPEG 압축)
      description:
        - JPEG는 손실 압축 기능으로 잘 알려진 널리 사용되는 이미지 형식입니다.
        - MozJPEG는 Mozilla에서 개발한 향상된 JPEG 인코더로, 더 나은 압축 및 품질에 중점을 둡니다.
        - 시각적 품질을 유지하면서 더 작은 파일 크기를 달성하여 웹 이미지에 이상적입니다.
        - MozJPEG는 JPEG 이미지를 최적화하기 위한 고급
zhHant:
  lead:
    subtitle: 現代圖像轉換器
    start_button: 入門
    description:
      - Drop Compress Image 是一款強大、快速且多功能的 GUI 工具，可將您的圖像轉換為新一代格式。它以性能為設計理念，利用現代編解碼器為您帶來速度、質量和文件大小方面的最佳體驗。
      - 告別過時的格式，迎接網絡圖像的未來！
      - 該項目旨在提供全面的轉換解決方案，支持廣泛的輸入格式，並導出高效的格式，如 AVIF、JPEG XL 和 WebP。
  download:
    download: 下載
    windows: 下載 Windows 版
    window_requirement: Windows 11 或更新版本
    macos: 下載 MacOS 版
    macos_requirement: M1 或更新版本
  features:
    title: 功能
    subtitle: Drop Compress Image 的主要功能
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
  format:
    title: 支援的輸出格式
    subtitle: 這些是 Drop Compress Image 支援的圖像格式。
    more: 瞭解更多
    webp:
      title: WebP
      description:
        - WebP 是一種現代圖像格式，為網絡上的圖像提供優越的無損和有損壓縮。
        - 由 Google 開發，在瀏覽器和平台上得到廣泛支援。
    avif:
      title: AVIF
      description:
        - AVIF（AV1 Image File Format）是一種基於 AV1 視頻編解碼器的下一代圖像格式。
        - 它提供出色的壓縮效率和圖像質量，非常適合網絡使用。主要瀏覽器均支援 AVIF，且其受歡迎程度正在提升。
    jxl:
      title: JPEG XL
      description:
        - JPEG XL 是一種現代圖像格式，設計為 JPEG 的繼任者。
        - 它提供更好的壓縮和質量，特別是對於高分辨率圖像。
        - JPEG XL 支援無損和有損壓縮。它針對網絡性能進行了優化。
    png:
      title: PNG (Zopfli 壓縮)
      description:
        - PNG（Portable Network Graphics）是一種允許將圖像「無任何質量損失」保存的格式。
        - 本程式中使用的 Zopfli 是 Google 創建的一種特殊壓縮技術，用於使 PNG「更小」。
        - 它支援透明度並保持高質量圖像。
        - Zopfli 是一種更高效地壓縮 PNG 圖像的算法。它使用先進的壓縮技術來減少文件大小，同時保留圖像質量。
    jpeg:
      title: JPEG (MozJPEG 壓縮)
      description:
        - JPEG 是一種廣泛使用的圖像格式，以其有損壓縮能力而聞名。
        - MozJPEG 是 Mozilla 開發的一種改進型 JPEG 編碼器，專注於更好的壓縮和質量。
        - 它在保持視覺質量的同時實現更小的文件大小，非常適合網絡圖像。
        - MozJPEG 採用先進技術來優化 JPEG 圖像，從而實現更快的加載時間和減少帶寬使用。
zhHans:
  lead:
    subtitle: 现代图像转换器
    start_button: 入门
    description:
      - Drop Compress Image 是一款强大、快速且多功能的 GUI 工具，可将您的图像转换为新一代格式。它以性能为设计理念，利用现代编解码器为您带来速度、质量和文件大小方面的最佳体验。
      - 告别过时的格式，迎接网络图像的未来！
      - 该项目旨在提供全面的转换解决方案，支持广泛的输入格式，并导出高效的格式，如 AVIF、JPEG XL 和 WebP。
  download:
    download: 下载安装
    windows: 下载安装 Windows 版
    window_requirement: Windows 11 或更新版本
    macos: 下载安装 MacOS 版
    macos_requirement: M1 或更新版本
  features:
    title: 功能
    subtitle: Drop Compress Image 的主要功能
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
  format:
    title: 支持的输出格式
    subtitle: 这些是 Drop Compress Image 支持的图像格式。
    more: 了解更多
    webp:
      title: WebP
      description:
        - WebP 是一种现代图像格式，为网络上的图像提供优越的无损和有损压缩。
        - 由 Google 开发，在浏览器和平台上得到广泛支持。
    avif:
      title: AVIF
      description:
        - AVIF（AV1 Image File Format）是一种基于 AV1 视频编解码器的下一代图像格式。
        - 它提供出色的压缩效率和图像质量，非常适合网络使用。主要浏览器均支持 AVIF，且其受欢迎程度正在提升。
    jxl:
      title: JPEG XL
      description:
        - JPEG XL 是一种现代图像格式，设计为 JPEG 的继任者。
        - 它提供更好的压缩和质量，特别是对于高分辨率图像。
        - JPEG XL 支持无损和有损压缩。它针对网络性能进行了优化。
    png:
      title: PNG (Zopfli 压缩)
      description:
        - PNG（Portable Network Graphics）是一种允许将图像「无任何质量损失」保存的格式。
        - 本程序中使用的 Zopfli 是 Google 创建的一种特殊压缩技术，用于使 PNG「更小」。
        - 它支持透明度并保持高质量图像。
        - Zopfli 是一种更高效地压缩 PNG 图像的算法。它使用先进的压缩技术来减少文件大小，同时保留图像质量。
    jpeg:
      title: JPEG (MozJPEG 压缩)
      description:
        - JPEG 是一种广泛使用的图像格式，以其有损压缩能力而闻名。
        - MozJPEG 是 Mozilla 开发的一种改进型 JPEG 编码器，专注于更好的压缩和质量。
        - 它在保持视觉质量的同时实现更小的文件大小，非常适合网络图像。
        - MozJPEG 采用先进技术来优化 JPEG 图像，从而实现更快的加载时间和减少带宽使用。
</i18n>
