import { fileURLToPath, URL } from 'node:url';

import VueI18nVitePlugin from '@intlify/unplugin-vue-i18n/vite';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

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
          href: 'https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Noto+Sans+JP:wght@100..900&family=Noto+Sans+KR:wght@100..900&family=Noto+Sans+Mono:wght@100..900&family=Noto+Sans+TC:wght@100..900&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap'
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
        '/ko',
        '/zh-tw',
        '/ja/getting-started',
        '/en/getting-started',
        '/ko/getting-started',
        '/zh-tw/getting-started',
        '/ja/test',
        '/en/test',
        '/ko/test',
        '/zh-tw/test'
      ]
    }
  },

  // 4. ルーティング設定
  router: {
    options: {
      strict: false
    }
  },

  modules: ['@nuxt/eslint', 'vuetify-nuxt-module', '@pinia/nuxt'],

  // CSS設定（Vuetifyのフォント設定を含む）
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
  },
  vite: {
    plugins: [
      VueI18nVitePlugin({
        include: fileURLToPath(new URL('./src/locales', import.meta.url))
      })
    ]
  }
});
