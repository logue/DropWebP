/**
 * Vuetify3 Plugin
 */
import 'vuetify/styles';
import '@mdi/font/css/materialdesignicons.css';

import { useI18n } from 'vue-i18n';

import { createVuetify, type VuetifyOptions } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { aliases, mdi } from 'vuetify/iconsets/mdi';
// Translations provided by Vuetify
import { createVueI18nAdapter } from 'vuetify/locale/adapters/vue-i18n';

import i18n from '@/plugins/i18n';
import { loadFonts } from '@/plugins/webfontloader';

await loadFonts();
/**
 * Vuetify Components
 *
 * @see {@link https://vuetifyjs.com/en/features/treeshaking/}
 */
let vuetifyConfig: VuetifyOptions = {
  // Global configuration
  // https://vuetifyjs.com/en/features/global-configuration/
  /*
  defaults: {
    global: {
      ripple: false,
    },
    VSheet: {
      elevation: 4,
    },
  },
  */
  // Icon Fonts
  // https://vuetifyjs.com/en/features/icon-fonts/
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi
    }
  },
  // Internationalization (i18n)
  // https://vuetifyjs.com/en/features/internationalization/#internationalization-i18n
  locale: {
    adapter: createVueI18nAdapter({ i18n, useI18n })
  },
  // Theme
  // https://vuetifyjs.com/en/features/theme/
  theme: {
    defaultTheme: 'light'
  }
};

if (import.meta.env.DEV) {
  // Disable treeshaking for DEV mode.
  vuetifyConfig = {
    components: { components },
    directives,
    ...vuetifyConfig
  };
}
export default createVuetify(vuetifyConfig);

// Export for test.
export { components, directives };
