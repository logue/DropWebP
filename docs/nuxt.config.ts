import { fileURLToPath, URL } from 'node:url';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  // SSG設定
  ssr: true,

  // 1. ソースコードディレクトリの変更
  srcDir: './src/',

  // 2. アプリ設定
  app: {
    baseURL: process.env.NODE_ENV === 'production' ? '/DropWebP/' : '/',
    buildAssetsDir: '_nuxt/',
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

  // 3. SSG設定（静的サイト生成）
  nitro: {
    prerender: {
      routes: [
        '/', // ルートはミドルウェアでリダイレクト
        '/ja',
        '/en',
        '/fr',
        '/ko',
        '/zhHant',
        '/zhHans',
        '/ja/format-guide',
        '/en/format-guide',
        '/fr/format-guide',
        '/ko/format-guide',
        '/zhHant/format-guide',
        '/zhHans/format-guide',
        '/ja/getting-started',
        '/en/getting-started',
        '/fr/getting-started',
        '/ko/getting-started',
        '/zhHant/getting-started',
        '/zhHans/getting-started'
      ]
    }
  },

  // i18n設定（<i18n>ブロック使用）
  i18n: {
    locales: [
      { code: 'ja', language: 'ja-JP', name: '🇯🇵 日本語' },
      { code: 'en', language: 'en-US', name: '🇺🇸 English' },
      { code: 'fr', language: 'fr-FR', name: '🇫🇷 Français' },
      { code: 'ko', language: 'ko-KR', name: '🇰🇷 한국어' },
      { code: 'zhHans', language: 'zh-CN', name: '🇨🇳 简体中文' },
      { code: 'zhHant', language: 'zh-TW', name: '🇹🇼 繁體中文' }
    ],
    defaultLocale: 'ja',
    strategy: 'prefix_and_default',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    },
    vueI18n: './i18n.config.ts',
    compilation: {
      strictMessage: false,
      escapeHtml: false
    }
  },

  // モジュール
  modules: ['@pinia/nuxt', 'vuetify-nuxt-module', '@nuxtjs/i18n'],

  // CSS設定
  css: ['@/styles/settings.scss'],
  // Vuetify設定
  vuetify: {
    vuetifyOptions: {
      theme: {
        defaultTheme: 'light',
        themes: {
          light: {
            colors: {
              primary: '#1976d2',
              secondary: '#424242',
              accent: '#82b1ff',
              error: '#ff5252',
              info: '#2196f3',
              success: '#4caf50',
              warning: '#ffc107'
            }
          },
          dark: {
            colors: {
              primary: '#2196f3',
              secondary: '#424242',
              accent: '#ff4081',
              error: '#ff5252',
              info: '#2196f3',
              success: '#4caf50',
              warning: '#fb8c00'
            }
          }
        }
      }
    }
  },

  // TypeScript パスエイリアス設定
  alias: {
    '@': fileURLToPath(new URL('./src', import.meta.url))
  },

  build: {
    transpile: ['vue-i18n']
  }
});
