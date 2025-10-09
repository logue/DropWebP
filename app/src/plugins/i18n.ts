import { createI18n } from 'vue-i18n';

import { en, fr, ja, ko, zhHant, zhHans } from 'vuetify/locale';

// ユーザーのブラウザ/OS言語を取得
let locale = navigator.language.slice(0, 2) || 'en'; // フォールバックとして'en'

if (locale === 'zh') {
  // 中国語の詳細なロケールを確認
  const fullLocale = navigator.language.toLowerCase();
  if (fullLocale === 'zh-cn' || fullLocale === 'zh-sg') {
    locale = 'zhHans'; // 簡体字中国語
  } else {
    locale = 'zhHant'; // 繁体字中国語
  }
}

export default createI18n({
  locale, // 'en-US' -> 'en' など
  fallbackLocale: 'en',
  messages: {
    en: { $vuetify: { ...en } }, // 英語
    fr: { $vuetify: { ...fr } }, // フランス語
    ja: { $vuetify: { ...ja } }, // 日本語
    ko: { $vuetify: { ...ko } }, // 韓国語
    zhHant: { $vuetify: { ...zhHant } }, // 繁体字中国語
    zhHans: { $vuetify: { ...zhHans } } // 簡体字中国語
  },
  legacy: false,
  globalInjection: true
});

document.documentElement.lang = locale;
