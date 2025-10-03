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
    buildAssetsDir: '_nuxt/'
  },

  // 3. SSG設定（静的サイト生成）
  nitro: {
    prerender: {
      routes: [
        '/',
        '/ja',
        '/en',
        '/ko',
        '/zh-tw',
        '/getting-started',
        '/ja/getting-started',
        '/en/getting-started',
        '/ko/getting-started',
        '/zh-tw/getting-started',
        '/test',
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

  modules: ['@nuxt/eslint', 'vuetify-nuxt-module'],

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
