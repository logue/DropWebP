import { readFileSync } from 'node:fs';
import { fileURLToPath, URL } from 'node:url';

// Load version from .env file
const loadVersionFromEnv = (): string => {
  try {
    const envPath = fileURLToPath(new URL('../.env', import.meta.url));
    const envContent = readFileSync(envPath, 'utf-8');
    const versionRegex = /^VERSION=(.+)$/m;
    const versionMatch = versionRegex.exec(envContent);
    return versionMatch ? versionMatch[1].trim() : '0.0.0';
  } catch {
    console.warn('Failed to load version from .env, using default version');
    return '0.0.0';
  }
};

const version = loadVersionFromEnv();

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

  // Runtime config to expose version
  runtimeConfig: {
    public: {
      appVersion: version
    }
  },

  // サイト設定
  site: {
    url: process.env.NUXT_PUBLIC_SITE_URL || 'https://logue.dev',
    name: 'Drop Compress Image'
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
    '@nuxtjs/sitemap',
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
  },
  // Google Analytics設定
  gtag: {
    id: 'G-2Y2FW3QEG4'
  },

  // TypeScript パスエイリアス設定
  alias: {
    '@': fileURLToPath(new URL('./src', import.meta.url))
  },

  build: {
    transpile: ['vue-i18n']
  },

  // Nitro設定（SSG用プリレンダリング - 自動生成を利用）
  nitro: {
    prerender: {
      crawlLinks: true,
      routes: ['/']
    }
  }
});
