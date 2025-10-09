<script setup lang="ts">
import { useI18n } from 'vue-i18n';

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
en:
  title: Getting Started
  meta_description: Guide to get started with Drop Compress Image
  subtitle: Start using Drop Compress Image easily
  installation:
    title: Installation
    download: Download the latest version from the official website
    install: Install the application by following the installer instructions
    launch: Launch Drop Compress Image
  download: Download
  usage:
    title: How to Use
    drag_drop:
      title: Drag & Drop
      description: Simply drag image files into the application window
    file_menu:
      title: File Menu
      description: Use the File menu to open image files
    batch_convert:
      title: Batch Conversion
      description: Select multiple files for batch processing
    settings:
      title: Settings
      description: Configure output format and quality settings
  features:
    title: Key Features
    list:
      webp:
        title: WebP Support
        description: Convert to modern WebP format with excellent compression
      avif:
        title: AVIF Support
        description: Support for next-generation AVIF format
      jxl:
        title: JPEG XL Support
        description: Convert to JPEG XL with superior compression
      lossless:
        title: Lossless Compression
        description: Maintain image quality with lossless compression options
ja:
  title: はじめに
  meta_description: Drop Compress Image の使い方ガイド
  subtitle: Drop Compress Image を簡単に始めましょう
  installation:
    title: インストール
    download: 公式サイトから最新版をダウンロード
    install: インストーラーの指示に従ってアプリケーションをインストール
    launch: Drop Compress Image を起動
  download: ダウンロード
  usage:
    title: 使用方法
    drag_drop:
      title: ドラッグ&ドロップ
      description: 画像ファイルをアプリケーションウィンドウにドラッグするだけ
    file_menu:
      title: ファイルメニュー
      description: ファイルメニューを使用して画像ファイルを開く
    batch_convert:
      title: 一括変換
      description: 複数のファイルを選択してバッチ処理
    settings:
      title: 設定
      description: 出力形式と品質設定を構成
  features:
    title: 主な機能
    list:
      webp:
        title: WebP サポート
        description: 優れた圧縮率の現代的な WebP 形式に変換
      avif:
        title: AVIF サポート
        description: 次世代 AVIF 形式をサポート
      jxl:
        title: JPEG XL サポート
        description: 優秀な圧縮率の JPEG XL に変換
      lossless:
        title: ロスレス圧縮
        description: ロスレス圧縮オプションで画像品質を維持
ko:
  title: 시작하기
  meta_description: Drop Compress Image 사용 가이드
  subtitle: Drop Compress Image를 쉽게 시작하세요
  installation:
    title: 설치
    download: 공식 웹사이트에서 최신 버전 다운로드
    install: 설치 프로그램 지침에 따라 애플리케이션 설치
    launch: Drop Compress Image 실행
  download: 다운로드
  usage:
    title: 사용 방법
    drag_drop:
      title: 드래그 & 드롭
      description: 이미지 파일을 애플리케이션 창에 드래그하기만 하면 됩니다
    file_menu:
      title: 파일 메뉴
      description: 파일 메뉴를 사용하여 이미지 파일 열기
    batch_convert:
      title: 일괄 변환
      description: 여러 파일을 선택하여 배치 처리
    settings:
      title: 설정
      description: 출력 형식 및 품질 설정 구성
  features:
    title: 주요 기능
    list:
      webp:
        title: WebP 지원
        description: 뛰어난 압축률의 현대적인 WebP 형식으로 변환
      avif:
        title: AVIF 지원
        description: 차세대 AVIF 형식 지원
      jxl:
        title: JPEG XL 지원
        description: 우수한 압축률의 JPEG XL으로 변환
      lossless:
        title: 무손실 압축
        description: 무손실 압축 옵션으로 이미지 품질 유지
zhHant:
  title: 入門指南
  meta_description: Drop Compress Image 使用指南
  subtitle: 輕鬆開始使用 Drop Compress Image
  installation:
    title: 安裝
    download: 從官方網站下載最新版本
    install: 按照安裝程式指示安裝應用程式
    launch: 啟動 Drop Compress Image
  download: 下載
  usage:
    title: 使用方法
    drag_drop:
      title: 拖放操作
      description: 只需將圖像文件拖到應用程式視窗中
    file_menu:
      title: 文件選單
      description: 使用文件選單開啟圖像文件
    batch_convert:
      title: 批次轉換
      description: 選擇多個文件進行批次處理
    settings:
      title: 設定
      description: 配置輸出格式和品質設定
  features:
    title: 主要功能
    list:
      webp:
        title: WebP 支援
        description: 轉換為具有優秀壓縮率的現代 WebP 格式
      avif:
        title: AVIF 支援
        description: 支援次世代 AVIF 格式
      jxl:
        title: JPEG XL 支援
        description: 轉換為具有卓越壓縮率的 JPEG XL
      lossless:
        title: 無損壓縮
        description: 使用無損壓縮選項保持圖像品質
zhHans:
  title: 入门指南
  meta_description: Drop Compress Image 使用指南
  subtitle: 轻松开始使用 Drop Compress Image
  installation:
    title: 安装
    download: 从官方网站下载最新版本
    install: 按照安装程序指示安装应用程序
    launch: 启动 Drop Compress Image
  download: 下载
  usage:
    title: 使用方法
    drag_drop:
      title: 拖放操作
      description: 只需将图像文件拖到应用程序窗口中
    file_menu:
      title: 文件菜单
      description: 使用文件菜单打开图像文件
    batch_convert:
      title: 批量转换
      description: 选择多个文件进行批量处理
    settings:
      title: 设置
      description: 配置输出格式和质量设置
  features:
    title: 主要功能
    list:
      webp:
        title: WebP 支持
        description: 转换为具有优秀压缩率的现代 WebP 格式
      avif:
        title: AVIF 支持
        description: 支持次世代 AVIF 格式
      jxl:
        title: JPEG XL 支持
        description: 转换为具有卓越压缩率的 JPEG XL
      lossless:
        title: 无损压缩
        description: 使用无损压缩选项保持图像质量
</i18n>
