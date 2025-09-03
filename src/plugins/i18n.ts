import { createI18n } from 'vue-i18n';

import { en, ja } from 'vuetify/locale';

export default createI18n({
  locale: 'ja',
  fallbackLocale: 'en',
  messages: { en, ja },
  legacy: false,
  globalInjection: true
});

document.documentElement.lang = navigator.language || 'en';
