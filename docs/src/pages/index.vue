<script setup lang="ts">
import logo from '@/assets/logo.png';

const { locale, rt, t } = useI18n();

// Composables
const { version, downloads, primaryDownload, detectPlatform } = useDownloads();
const { features, formats, leadDescriptions, getFormatDescriptions } = useContentData();
const { languages, setupSeoMeta } = useSeoMetadata();

// OS検出
onMounted(() => {
  detectPlatform();
});

// SEOメタデータの設定
setupSeoMeta();
</script>

<template>
  <v-card class="mb-6 bg-transparent mx-auto" flat tag="section" max-width="960">
    <v-img :src="logo" alt="Drop Compress Image Logo" max-width="256" class="mx-auto mb-4" />
    <v-card-title class="text-h4 text-center pa-3" tag="h2">Drop Compress Image</v-card-title>
    <v-card-subtitle class="text-center pb-4">{{ t('lead.subtitle') }}</v-card-subtitle>
    <v-card-text class="text-center">
      <!-- Language Links -->
      <v-chip-group class="flex justify-center mb-6" tag="nav">
        <v-chip
          v-for="lang in languages"
          :key="lang.code"
          :hreflang="lang.code"
          :to="lang.code === 'en' ? '/' : `/${lang.code}`"
          :variant="locale === lang.code ? 'elevated' : 'outlined'"
          :color="locale === lang.code ? 'primary' : 'default'"
          :text="lang.name"
          class="block mx-auto"
          rel="alternate"
          size="small"
        />
      </v-chip-group>
      <p v-for="(description, index) in leadDescriptions" :key="`lead-${index}`">
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

    <!-- Primary Download Button (Auto-detected) -->
    <v-card-actions class="justify-center mb-4 d-flex flex-column">
      <v-btn
        :href="primaryDownload.url"
        :prepend-icon-color="primaryDownload.iconColor"
        :prepend-icon="primaryDownload.icon"
        class="px-8 py-4"
        download
        height="100"
        size="x-large"
        spaced="both"
        stacked
        variant="elevated"
      >
        <span class="text-center">
          <div class="text-h6 mb-1 text-primary">{{ primaryDownload.label }}</div>
          <small class="text-medium-emphasis">{{ primaryDownload.subtitle }}</small>
        </span>
      </v-btn>
      <br />
      <!-- Alternative Downloads Expansion Panel -->
      <v-expansion-panels elevation="2">
        <v-expansion-panel>
          <v-expansion-panel-title>
            <v-icon start>mdi-package-variant</v-icon>
            {{ t('download.other_platforms') }}
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-row>
              <!-- Windows -->
              <v-col cols="12" md="6">
                <v-list density="compact">
                  <v-list-subheader>
                    <v-icon start color="blue">mdi-microsoft-windows</v-icon>
                    {{ t('download.windows') }}
                  </v-list-subheader>
                  <v-list-item
                    :href="downloads.windows.x64"
                    download
                    prepend-icon="mdi-download"
                    subtitle=".msi installer"
                    title="Windows 10/11 (x64)"
                  />
                </v-list>
              </v-col>
              <!-- macOS -->
              <v-col cols="12" md="6">
                <v-list density="compact">
                  <v-list-subheader>
                    <v-icon start color="red">mdi-apple</v-icon>
                    {{ t('download.macos') }}
                  </v-list-subheader>
                  <v-list-item
                    :href="downloads.macos.universal"
                    :subtitle="t('download.recommended')"
                    :title="t('download.macos_universal')"
                    download
                    prepend-icon="mdi-download"
                  />
                </v-list>
              </v-col>
              <!-- Linux x86_64 -->
              <v-col cols="12" md="6">
                <v-list density="compact">
                  <v-list-subheader>
                    <v-icon start color="orange">mdi-linux</v-icon>
                    {{ t('download.linux') }} - x86_64
                  </v-list-subheader>
                  <!--v-list-item
                    :href="downloads.linux.x64.appimage"
                    :subtitle="t('download.linux_appimage_desc')"
                    download
                    prepend-icon="mdi-download"
                    title="AppImage (x86_64)"
                  /-->
                  <v-list-item
                    :href="downloads.linux.x64.deb"
                    download
                    prepend-icon="mdi-download"
                    subtitle="Debian / Ubuntu"
                    title=".deb (x86_64)"
                  />
                  <v-list-item
                    :href="downloads.linux.x64.rpm"
                    prepend-icon="mdi-download"
                    download
                    title=".rpm (x86_64)"
                    subtitle="Fedora / RHEL / openSUSE"
                  />
                </v-list>
              </v-col>
              <!-- Linux ARM64 -->
              <v-col cols="12" md="6">
                <v-list density="compact">
                  <v-list-subheader class="px-0">
                    <v-icon start color="orange">mdi-linux</v-icon>
                    {{ t('download.linux') }} - ARM64
                  </v-list-subheader>
                  <!--v-list-item
                    :href="downloads.linux.arm64.appimage"
                    :subtitle="t('download.linux_appimage_desc')"
                    download
                    prepend-icon="mdi-download"
                    title="AppImage (ARM64)"
                  /-->
                  <v-list-item
                    :href="downloads.linux.arm64.deb"
                    prepend-icon="mdi-download"
                    download
                    title=".deb (ARM64)"
                    subtitle="Debian / Ubuntu"
                  />
                  <v-list-item
                    :href="downloads.linux.arm64.rpm"
                    prepend-icon="mdi-download"
                    download
                    title=".rpm (ARM64)"
                    subtitle="Fedora / RHEL / openSUSE"
                  />
                </v-list>
              </v-col>
            </v-row>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
    </v-card-actions>
  </v-card>

  <v-card class="mb-6 bg-transparent" flat tag="section">
    <v-card-title class="text-h5 text-center" tag="h2">{{ t('features.title') }}</v-card-title>
    <v-card-subtitle class="text-center">{{ t('features.subtitle') }}</v-card-subtitle>
    <v-card-text>
      <v-row class="mb-5">
        <v-col v-for="item in features" :key="item.key" cols="12" md="4">
          <v-card class="h-100">
            <v-icon :icon="item.icon" size="64" color="primary" class="ma-4 mx-auto w-100" />
            <v-card-title class="text-h6 text-center mt-2" tag="h3">
              {{ t(`features.${item.key}.title`) }}
            </v-card-title>
            <v-card-text class="text-center">
              {{ t(`features.${item.key}.description`) }}
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
              :src="item.logo"
              :alt="t(`format.${item.key}.title`)"
              max-height="128"
              contain
              class="mx-auto my-4"
            />
            <v-card-title class="text-h6 text-center mt-4" tag="h3">
              {{ t(`format.${item.key}.title`) }}
            </v-card-title>
            <v-card-text>
              <p
                v-for="(description, index) in getFormatDescriptions(item.key)"
                :key="`${item.key}-${index}`"
              >
                {{ rt(description) }}
              </p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-card-text>
    <v-card-actions class="justify-center">
      <NuxtLink to="/format-guide" custom #="{ navigate }">
        <v-btn
          :text="t('format.more')"
          class="ma-4"
          color="primary"
          prepend-icon="mdi-book-open-page-variant"
          size="large"
          variant="elevated"
          @click="navigate"
        />
      </NuxtLink>
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
    macos: Download for macOS
    macos_universal: Universal Binary (Recommended)
    linux: Download for Linux
    linux_x64: x86_64
    linux_arm64: ARM64
    other_platforms: Other Platforms & Formats
    recommended: Recommended
    select_platform: Select your platform
    linux_appimage_desc: Portable, distribution-independent
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
      title: PNG (Oxipng)
      description:
        - PNG (Portable Network Graphics) is a format that allows images to be saved "without any loss of quality."
        - Zopfli, used in this program, is a special compression technique created by Google to make PNGs "smaller."
        - It supports transparency and maintains high-quality images.
    jpeg:
      title: JPEG (jpegli Compression)
      description:
        - JPEG (Joint Photographic Experts Group) is an image format widely used for photographs and realistic images.
        - JPEG, the format used in this program, is a special compression technology developed by Google to make JPEGs "smaller," achieving a compression rate that is approximately 35% better than standard JPEG compression.
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
    window_requirement: Windows 11 ou ultérieur
    macos: Télécharger pour macOS
    macos_universal: Binaire Universel
    linux: Télécharger pour Linux
    linux_x64: x86_64
    linux_arm64: ARM64
    other_platforms: Autres Plateformes et Formats
    recommended: Recommandé
    select_platform: Sélectionnez votre plateforme
    linux_appimage_desc: Portable, indépendant de la distribution
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
      title: PNG (Oxipng)
      description:
        - PNG (Portable Network Graphics) est un format qui permet de sauvegarder les images "sans aucune perte de qualité".
        - Zopfli, utilisé dans ce programme, est une technique de compression spéciale créée par Google pour rendre les PNG "plus petits".
        - Il prend en charge la transparence et maintient des images de haute qualité.
    jpeg:
      title: JPEG (Compression jpegli)
      description:
        - JPEG (Joint Photographic Experts Group) est un format d'image largement utilisé pour les photographies et les images réalistes.
        - jpegli est un encodeur JPEG amélioré développé par Google, axé sur une meilleure compression et qualité.
        - Il atteint une taille de fichier plus petite tout en maintenant la qualité visuelle, ce qui le rend idéal pour les images web.
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
    macos: macOS版をダウンロード
    macos_universal: ユニバーサルバイナリ
    linux: Linux版をダウンロード
    linux_x64: x86_64
    linux_arm64: ARM64
    other_platforms: その他のプラットフォームと形式
    recommended: 推奨
    select_platform: プラットフォームを選択
    linux_appimage_desc: ポータブル、ディストリビューション非依存
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
      title: PNG (Oxipng)
      description:
        - PNG（Portable Network Graphics）は、画像を「画質を全く落とさずに」保存できる形式です。
        - 本プログラムで使用されているZopfli（ゾップフリ）とは、このPNGを「より小さく」するための、Googleが作った特別な圧縮技術です。
        - 透明度をサポートし、高品質の画像を保持します。
    jpeg:
      title: JPEG (jpegli圧縮)
      description:
        - JPEG（Joint Photographic Experts Group）は、写真やリアルな画像に広く使用されている画像フォーマットです。
        - 本プログラムで使用されているjpegli（ジェイペグエルアイ）とは、このJPEGを「より小さく」するための、Googleが開発した特別な圧縮技術で通常のJPEG圧縮と比較して約35 ％の圧縮率向上を実現しています。
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
    macos: macOS용 다운로드
    macos_universal: 유니버설 바이너리
    linux: Linux용 다운로드
    linux_x64: x86_64
    linux_arm64: ARM64
    other_platforms: 기타 플랫폼 및 형식
    recommended: 권장
    select_platform: 플랫폼 선택
    linux_appimage_desc: 휴대 가능, 배포판 독립적
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
      title: PNG (Oxipng)
      description:
        - PNG(Portable Network Graphics)는 이미지를 "품질 손실 없이" 저장할 수 있는 형식입니다.
        - 이 프로그램에서 사용되는 Zopfli는 PNG를 "더 작게" 만들기 위해 Google이 만든 특별한 압축 기술입니다.
        - 투명도를 지원하며 고품질 이미지를 유지합니다.
    jpeg:
      title: JPEG (jpegli 압축)
      description:
        - JPEG (Joint Photographic Experts Group)는 사진과 현실적인 이미지에 널리 사용되는 이미지 형식입니다.
        - 본 프로그램에서 사용되고 있는 jpegli(제이페구엘아이)란, 이 JPEG를 「보다 작게」 하기 위한, 구글이 개발한 특별한 압축 기술로 통상의 JPEG 압축에 비해 약 35%의 압축률 향상을 실현하고 있습니다.
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
    macos_universal: 通用二進位檔
    linux: 下載 Linux 版
    linux_x64: x64
    linux_arm64: ARM64
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
      title: PNG (Oxipng)
      description:
        - PNG（Portable Network Graphics）是一種允許將圖像「無任何質量損失」保存的格式。
        - 本程式中使用的 Zopfli 是 Google 創建的一種特殊壓縮技術，用於使 PNG「更小」。
        - 它支援透明度並保持高質量圖像。
    jpeg:
      title: JPEG (jpegli 壓縮)
      description:
        - JPEG（聯合影像專家小組）是一種廣泛用於照片和逼真影像的影像格式。
        - 本程式使用的 jpegli 是由 Google 開發的一種特殊壓縮技術，旨在減小 JPEG 檔案的大小，其壓縮率比標準 JPEG 壓縮高出約 35%。
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
    macos_universal: 通用二进制文件
    linux: 下载安装 Linux 版
    linux_x64: x64
    linux_arm64: ARM64
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
      title: PNG (Oxipng)
      description:
        - PNG（Portable Network Graphics）是一种允许将图像「无任何质量损失」保存的格式。
        - 本程序中使用的 Zopfli 是 Google 创建的一种特殊压缩技术，用于使 PNG「更小」。
        - 它支持透明度并保持高质量图像。
    jpeg:
      title: JPEG (jpegli 压缩)
      description:
        - JPEG（联合图像专家组）是一种广泛用于照片和逼真图像的图像格式。
        - 本程序使用的 jpegli 是由 Google 开发的一种特殊压缩技术，旨在减小 JPEG 文件的大小，其压缩率比标准 JPEG 压缩高出约 35%。
</i18n>
