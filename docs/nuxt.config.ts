import { fileURLToPath, URL } from 'node:url';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // SSG設定（CSS外部化対応）
  ssr: true, // SSRで翻訳済みHTMLを生成
  // ソースコードディレクトリの変更
  srcDir: './src/',
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  // CSSファイル（Vuetifyスタイル確保 + GitHub Markdown CSS）
  css: ['~/styles/settings.scss'],
  // SSRスタイル設定（CSS最適化）
  features: {
    inlineStyles: false // CSS外部化
  },

  // アプリ設定
  app: {
    baseURL: process.env.NUXT_APP_BASE_URL || '/DropWebP/',
    head: {
      link: [
        // app側と同じGoogle Fontsを読み込み
        {
          rel: 'preconnect',
          href: 'https://fonts.googleapis.com'
        },
        {
          rel: 'preconnect',
          href: 'https://fonts.gstatic.com',
          crossorigin: 'anonymous'
        },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Noto+Sans+JP:wght@100..900&family=Noto+Sans+KR:wght@100..900&family=Noto+Sans+Mono:wght@100..900&family=Noto+Sans+SC:wght@100..900&family=Noto+Sans+TC:wght@100..900&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap'
        }
      ]
    }
  },
  // モジュール
  modules: [
    '@nuxt/content',
    '@nuxt/eslint',
    '@nuxtjs/i18n',
    '@pinia/nuxt',
    'nuxt-gtag',
    'vuetify-nuxt-module'
  ],

  // i18n設定（<i18n>ブロック使用）
  i18n: {
    locales: [
      { code: 'en', language: 'en-US', name: '🇺🇸 English', iso: 'en-US' },
      { code: 'fr', language: 'fr-FR', name: '🇫🇷 Français', iso: 'fr-FR' },
      { code: 'ja', language: 'ja-JP', name: '🇯🇵 日本語', iso: 'ja-JP' },
      { code: 'ko', language: 'ko-KR', name: '🇰🇷 한국어', iso: 'ko-KR' },
      { code: 'zhHans', language: 'zh-CN', name: '🇨🇳 简体中文', iso: 'zh-CN' },
      { code: 'zhHant', language: 'zh-TW', name: '🇹🇼 繁體中文', iso: 'zh-TW' }
    ],
    strategy: 'prefix_except_default',
    defaultLocale: 'en',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root' // recommended
    }
    /*
    baseUrl:
      process.env.NUXT_PUBLIC_SITE_URL ||
      (process.env.GITHUB_PAGES
        ? "https://logue.dev"
        : "http://localhost:3000"),
    compilation: {
      // HTMLを含むメッセージの警告を無効化
      strictMessage: false,
    },
    */
  },
  // Google Analytics設定
  gtag: {
    id: 'G-2Y2FW3QEG4'
  }
});
