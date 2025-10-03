"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var vue_i18n_1 = require("vue-i18n");
var locale_1 = require("vuetify/locale");
// ユーザーのブラウザ/OS言語を取得
var userLocale = navigator.language.slice(0, 2) || 'en'; // フォールバックとして'en'
exports.default = (0, vue_i18n_1.createI18n)({
    locale: userLocale, // 'en-US' -> 'en' など
    fallbackLocale: 'en',
    messages: { en: locale_1.en, ja: locale_1.ja, ko: locale_1.ko, zhHant: locale_1.zhHant },
    legacy: false,
    globalInjection: true
});
document.documentElement.lang = userLocale;
