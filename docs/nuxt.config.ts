import { fileURLToPath, URL } from 'node:url';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  // SSG設定（CSS外部化対応）
  ssr: true, // SSRで翻訳済みHTMLを生成

  // CSSファイル（Vuetifyスタイル確保 + GitHub Markdown CSS）
  css: [
    '~/styles/settings.scss',
    'github-markdown-css/github-markdown-light.css',
    'github-markdown-css/github-markdown-dark.css'
  ],

  // SSRスタイル設定（CSS最適化）
  features: {
    inlineStyles: false // CSS外部化
  },

  plugins: ['~/plugins/prism.client.ts'],

  // 1. ソースコードディレクトリの変更
  srcDir: './src/',

  // 2. アプリ設定
  app: {
    baseURL: process.env.NUXT_APP_BASE_URL || (process.env.GITHUB_PAGES ? '/DropWebP/' : '/'),
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

  // i18n設定（<i18n>ブロック使用）
  i18n: {
    locales: [
      { code: 'ja', language: 'ja-JP', name: '🇯🇵 日本語', iso: 'ja-JP' },
      { code: 'en', language: 'en-US', name: '🇺🇸 English', iso: 'en-US' },
      { code: 'fr', language: 'fr-FR', name: '🇫🇷 Français', iso: 'fr-FR' },
      { code: 'ko', language: 'ko-KR', name: '🇰🇷 한국어', iso: 'ko-KR' },
      { code: 'zhHans', language: 'zh-CN', name: '🇨🇳 简体中文', iso: 'zh-CN' },
      { code: 'zhHant', language: 'zh-TW', name: '🇹🇼 繁體中文', iso: 'zh-TW' }
    ],
    defaultLocale: 'en',
    strategy: 'prefix_and_default',
    baseUrl: 'https://logue.dev/DropWebP',
    detectBrowserLanguage: false,
    compilation: {
      // HTMLを含むメッセージの警告を無効化
      strictMessage: false
    }
  },

  // モジュール
  modules: ['@pinia/nuxt', 'vuetify-nuxt-module', '@nuxtjs/i18n', '@nuxt/eslint'],

  // Vuetify設定（CSS完全外部化）
  vuetify: {
    moduleOptions: {
      /* CSS外部化を強制 */
      styles: true
    },
    vuetifyOptions: {
      theme: {
        // テーマCSS外部化
        variations: false
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

  // Vite設定（VuetifyCSS外部化強制 + Markdownローダー）
  vite: {
    css: {
      postcss: {}
    },
    ssr: {
      // VuetifyをSSR時に外部化
      noExternal: ['vuetify']
    },
    optimizeDeps: {
      include: ['prismjs', 'prismjs/components/prism-bash', 'prismjs/components/prism-powershell']
    },
    assetsInclude: ['**/*.md'],
    plugins: [
      {
        name: 'markdown-loader',
        transform(code, id) {
          if (id.endsWith('.md')) {
            return `export default ${JSON.stringify(code)};`;
          }
        }
      }
    ],
    build: {
      rollupOptions: {
        output: {
          // CSS外部化設定（_nuxtディレクトリに統一）
          assetFileNames: assetInfo => {
            if (assetInfo.name?.endsWith('.css')) {
              // VuetifyのCSSを_nuxtディレクトリに出力
              return '_nuxt/vuetify-[hash].css';
            }
            return '_nuxt/[name]-[hash][extname]';
          },
          // CSSチャンクを単一ファイルに統合
          manualChunks: undefined
        }
      },
      // CSS分割を無効化してVuetify CSSを統合
      cssCodeSplit: false,
      // CSS最適化を有効化
      cssMinify: true
    }
  },

  // Nitro設定（SSG + CSS最適化）
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
        '/zhHans/getting-started',
        '/ja/build-windows',
        '/en/build-windows',
        '/fr/build-windows',
        '/ko/build-windows',
        '/zhHant/build-windows',
        '/zhHans/build-windows',
        '/ja/build-macos',
        '/en/build-macos',
        '/fr/build-macos',
        '/ko/build-macos',
        '/zhHant/build-macos',
        '/zhHans/build-macos'
      ]
    },
    inlineDynamicImports: false,
    minify: true,
    // CSS外部化を強制（_nuxtディレクトリに統一）
    rollupConfig: {
      output: {
        assetFileNames: assetInfo => {
          if (assetInfo.name?.endsWith('.css')) {
            return '_nuxt/[name]-[hash].css';
          }
          return '_nuxt/[name]-[hash][extname]';
        }
      }
    }
  }
});
