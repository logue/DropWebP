<script setup lang="ts">
const { locale, rt, t } = useI18n();

// Composables
const { version, downloads, primaryDownload, detectPlatform } = useDownloads();
const { features, leadDescriptions } = useContentData();
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
    <v-card-title class="text-h4 text-center pa-3" tag="h2">Tauri Vue3 App</v-card-title>
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
</template>

<i18n lang="yaml">
en:
  lead:
    subtitle: Modern Cross-Platform Desktop Application
    description:
      - This is a modern desktop application built with Tauri v2 and Vue 3, combining the power of Rust with the flexibility of web technologies.
      - Built for performance, security, and a great user experience across Windows, macOS, and Linux.
      - This template provides a solid foundation for building your own cross-platform desktop applications with modern tools and best practices.
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
    subtitle: Key Features of This Application
    multiple_formats:
      title: Cross-Platform
      description: Works seamlessly on Windows, macOS, and Linux with native performance.
    high_speed:
      title: High Performance
      description: Built with Rust backend for blazing fast performance and low resource usage.
    drag_drop:
      title: Modern UI
      description: Beautiful, responsive interface built with Vue 3 and Vuetify.
    dark_mode:
      title: Dark Mode
      description: Enjoy a comfortable viewing experience with dark mode support.
    i18n:
      title: Internationalization
      description: Supports multiple languages for a global user experience.
    paste:
      title: Secure & Safe
      description: Built with security in mind, leveraging Tauri's security features.
fr:
  lead:
    subtitle: Application de bureau multiplateforme moderne
    start_button: Commencer
    description:
      - Il s'agit d'une application de bureau moderne construite avec Tauri v2 et Vue 3, combinant la puissance de Rust avec la flexibilité des technologies web.
      - Conçue pour la performance, la sécurité et une excellente expérience utilisateur sur Windows, macOS et Linux.
      - Ce modèle fournit une base solide pour créer vos propres applications de bureau multiplateformes avec des outils et des pratiques modernes.
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
    subtitle: Fonctionnalités clés de cette application
    multiple_formats:
      title: Multiplateforme
      description: Fonctionne de manière transparente sur Windows, macOS et Linux avec des performances natives.
    high_speed:
      title: Hautes performances
      description: Construit avec un backend Rust pour des performances ultra-rapides et une faible utilisation des ressources.
    drag_drop:
      title: Interface moderne
      description: Interface belle et réactive construite avec Vue 3 et Vuetify.
    dark_mode:
      title: Mode Sombre
      description: Profitez d'une expérience visuelle confortable avec la prise en charge du mode sombre.
    i18n:
      title: Internationalisation
      description: Prend en charge plusieurs langues pour une expérience utilisateur mondiale.
    paste:
      title: Sécurisé et sûr
      description: Conçu avec la sécurité à l'esprit, tirant parti des fonctionnalités de sécurité de Tauri.
ja:
  lead:
    subtitle: モダンなクロスプラットフォームデスクトップアプリケーション
    start_button: はじめに
    description:
      - Tauri v2とVue 3で構築されたモダンなデスクトップアプリケーション。RustのパワーとWeb技術の柔軟性を組み合わせています。
      - Windows、macOS、Linuxで優れたパフォーマンス、セキュリティ、ユーザー体験を提供します。
      - このテンプレートは、最新のツールとベストプラクティスを使用して独自のクロスプラットフォームデスクトップアプリケーションを構築するための強固な基盤を提供します。
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
    subtitle: このアプリケーションの主な機能
    multiple_formats:
      title: クロスプラットフォーム
      description: Windows、macOS、Linuxでネイティブパフォーマンスをシームレスに実現。
    high_speed:
      title: 高性能
      description: Rustバックエンドによる超高速なパフォーマンスと低リソース使用率。
    dark_mode:
      title: ダークモード
      description: ダークモード対応で快適な閲覧体験を提供。
    drag_drop:
      title: モダンUI
      description: Vue 3とVuetifyで構築された美しくレスポンシブなインターフェース。
    i18n:
      title: 多言語対応
      description: 複数言語に対応し、グローバルなユーザー体験を提供。
    paste:
      title: セキュア＆セーフ
      description: Tauriのセキュリティ機能を活用したセキュリティ重視の設計。
ko:
  lead:
    subtitle: 현대적인 크로스 플랫폼 데스크톱 애플리케이션
    start_button: 시작하기
    description:
      - Tauri v2와 Vue 3로 구축된 현대적인 데스크톱 애플리케이션으로, Rust의 강력함과 웹 기술의 유연성을 결합합니다.
      - Windows, macOS 및 Linux에서 뛰어난 성능, 보안 및 사용자 경험을 제공하도록 설계되었습니다.
      - 이 템플릿은 현대적인 도구와 모범 사례를 사용하여 크로스 플랫폼 데스크톱 애플리케이션을 구축하기 위한 견고한 기반을 제공합니다.
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
    subtitle: 이 애플리케이션의 주요 기능
    multiple_formats:
      title: 크로스 플랫폼
      description: Windows, macOS 및 Linux에서 원활하게 작동하며 네이티브 성능을 제공합니다.
    high_speed:
      title: 고성능
      description: Rust 백엔드로 구축되어 초고속 성능과 낮은 리소스 사용량을 제공합니다.
    drag_drop:
      title: 모던 UI
      description: Vue 3와 Vuetify로 구축된 아름답고 반응형 인터페이스.
    dark_mode:
      title: 다크 모드
      description: 다크 모드 지원으로 편안한 시청 경험 제공.
    i18n:
      title: 다국어 지원
      description: 글로벌 사용자 경험을 위한 다국어 지원.
    paste:
      title: 안전하고 보안적
      description: Tauri의 보안 기능을 활용하여 보안을 염두에 두고 구축되었습니다.
zhHant:
  lead:
    subtitle: 現代跨平台桌面應用程式
    start_button: 入門
    description:
      - 這是一個使用 Tauri v2 和 Vue 3 構建的現代桌面應用程式，結合了 Rust 的強大功能和 Web 技術的靈活性。
      - 專為在 Windows、macOS 和 Linux 上提供性能、安全性和出色的用戶體驗而構建。
      - 此範本為使用現代工具和最佳實踐構建您自己的跨平台桌面應用程式提供了堅實的基礎。
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
    subtitle: 此應用程式的主要功能
    multiple_formats:
      title: 跨平台
      description: 在 Windows、macOS 和 Linux 上無縫運行，具有原生性能。
    high_speed:
      title: 高性能
      description: 使用 Rust 後端構建，提供極快的性能和低資源使用率。
    drag_drop:
      title: 現代介面
      description: 使用 Vue 3 和 Vuetify 構建的美觀、響應式介面。
    dark_mode:
      title: 暗黑模式
      description: 暗黑模式支援，享受舒適的瀏覽體驗。
    i18n:
      title: 多語言支援
      description: 支援多種語言以提供全球用戶體驗。
    paste:
      title: 安全可靠
      description: 以安全為設計理念，利用 Tauri 的安全功能。
zhHans:
  lead:
    subtitle: 现代跨平台桌面应用程序
    start_button: 入门
    description:
      - 这是一个使用 Tauri v2 和 Vue 3 构建的现代桌面应用程序，结合了 Rust 的强大功能和 Web 技术的灵活性。
      - 专为在 Windows、macOS 和 Linux 上提供性能、安全性和出色的用户体验而构建。
      - 此模板为使用现代工具和最佳实践构建您自己的跨平台桌面应用程序提供了坚实的基础。
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
    subtitle: 此应用程序的主要功能
    multiple_formats:
      title: 跨平台
      description: 在 Windows、macOS 和 Linux 上无缝运行，具有原生性能。
    high_speed:
      title: 高性能
      description: 使用 Rust 后端构建，提供极快的性能和低资源使用率。
    drag_drop:
      title: 现代界面
      description: 使用 Vue 3 和 Vuetify 构建的美观、响应式界面。
    dark_mode:
      title: 暗黑模式
      description: 暗黑模式支持，享受舒适的浏览体验。
    i18n:
      title: 多语言支持
      description: 支持多种语言以提供全球用户体验。
    paste:
      title: 安全可靠
      description: 以安全为设计理念，利用 Tauri 的安全功能。
</i18n>
