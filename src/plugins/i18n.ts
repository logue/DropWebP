import { createI18n } from 'vue-i18n';

import { en, ja, ko, zhHant } from 'vuetify/locale';

// ユーザーのブラウザ/OS言語を取得
const userLocale = navigator.language.slice(0, 2) || 'en'; // フォールバックとして'en'

export default createI18n({
  locale: userLocale, // 'en-US' -> 'en' など
  fallbackLocale: 'en',
  messages: { en, ja, ko, zhHant },
  legacy: false,
  globalInjection: true
});

document.documentElement.lang = userLocale;
