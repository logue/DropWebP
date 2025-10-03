"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.directives = exports.components = void 0;
/**
 * Vuetify3 Plugin
 */
require("vuetify/styles");
require("@mdi/font/css/materialdesignicons.css");
var vue_i18n_1 = require("vue-i18n");
var vuetify_1 = require("vuetify");
var components = require("vuetify/components");
exports.components = components;
var directives = require("vuetify/directives");
exports.directives = directives;
var mdi_1 = require("vuetify/iconsets/mdi");
// Translations provided by Vuetify
var vue_i18n_2 = require("vuetify/locale/adapters/vue-i18n");
var i18n_1 = require("@/plugins/i18n");
/**
 * Vuetify Components
 *
 * @see {@link https://vuetifyjs.com/en/features/treeshaking/}
 */
var vuetifyConfig = {
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
        aliases: mdi_1.aliases,
        sets: {
            mdi: mdi_1.mdi
        }
    },
    // Internationalization (i18n)
    // https://vuetifyjs.com/en/features/internationalization/#internationalization-i18n
    locale: {
        adapter: (0, vue_i18n_2.createVueI18nAdapter)({ i18n: i18n_1.default, useI18n: vue_i18n_1.useI18n })
    },
    // Theme
    // https://vuetifyjs.com/en/features/theme/
    theme: {
        defaultTheme: 'light'
    }
};
if (import.meta.env.DEV) {
    // Disable treeshaking for DEV mode.
    vuetifyConfig = __assign({ components: { components: components }, directives: directives }, vuetifyConfig);
}
exports.default = (0, vuetify_1.createVuetify)(vuetifyConfig);
