"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var pinia_1 = require("pinia");
var vue_1 = require("vue");
var vue_i18n_1 = require("vue-i18n");
/** Config Store */
exports.default = (0, pinia_1.defineStore)('config', function () {
    // 1. i18nインスタンスからlocaleを取得
    var locale = (0, vue_i18n_1.useI18n)({ useScope: 'global' }).locale;
    // 2. Piniaのstateとして言語を定義（デフォルト値やlocalStorageからの復元など）
    var currentLocale = (0, vue_1.ref)(locale.value); // 初期値をi18nから拝借
    // 3. stateが変更されたら、i18nのlocaleにも反映させる watchを設置
    (0, vue_1.watch)(currentLocale, function (newLocale) {
        locale.value = newLocale;
        // 必要ならlocalStorageに保存する処理もここに追加
        // localStorage.setItem('locale', newLocale)
    });
    /** Dark Theme mode */
    var theme = (0, vue_1.ref)(window.matchMedia('(prefers-color-scheme: dark)').matches);
    /** Toggle Dark/Light mode */
    var toggleTheme = function () { return (theme.value = !theme.value); };
    /**
     * Set Locale.
     *
     * @param locale - Locale
     */
    var setLocale = function (l) { return (locale.value = l); };
    return { theme: theme, locale: locale, toggleTheme: toggleTheme, setLocale: setLocale };
});
