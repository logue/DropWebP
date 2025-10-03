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
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
var store_1 = require("@/store");
var vue_i18n_1 = require("vue-i18n");
/** vue i18n */
var _a = (0, vue_i18n_1.useI18n)(), t = _a.t, availableLocales = _a.availableLocales;
/** グローバルストア */
var configStore = (0, store_1.useConfigStore)();
// localeの変更はストアのアクション経由で行う
function changeLocale(newLocale) {
    configStore.setLocale(newLocale);
}
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VList;
/** @type {[typeof __VLS_components.VList, typeof __VLS_components.vList, typeof __VLS_components.VList, typeof __VLS_components.vList, ]} */ ;
// @ts-ignore
VList;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({
    mandatory: true,
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        mandatory: true,
    }], __VLS_functionalComponentArgsRest(__VLS_1), false));
var __VLS_4 = {};
var __VLS_5 = __VLS_3.slots.default;
var _loop_1 = function (lang) {
    // @ts-ignore
    [availableLocales,];
    var __VLS_6 = {}.VListItem;
    /** @type {[typeof __VLS_components.VListItem, typeof __VLS_components.vListItem, typeof __VLS_components.VListItem, typeof __VLS_components.vListItem, ]} */ ;
    // @ts-ignore
    VListItem;
    // @ts-ignore
    var __VLS_7 = __VLS_asFunctionalComponent(__VLS_6, new __VLS_6(__assign({ 'onClick': {} }, { key: (lang), active: (__VLS_ctx.configStore.locale === lang) })));
    var __VLS_8 = __VLS_7.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { key: (lang), active: (__VLS_ctx.configStore.locale === lang) })], __VLS_functionalComponentArgsRest(__VLS_7), false));
    var __VLS_10 = void 0;
    var __VLS_11 = void 0;
    var __VLS_12 = ({ click: {} },
        { onClick: function () {
                var _a = [];
                for (var _i = 0; _i < arguments.length; _i++) {
                    _a[_i] = arguments[_i];
                }
                var $event = _a[0];
                __VLS_ctx.changeLocale(lang);
                // @ts-ignore
                [configStore, changeLocale,];
            } });
    var __VLS_13 = __VLS_9.slots.default;
    var __VLS_14 = {}.VListItemTitle;
    /** @type {[typeof __VLS_components.VListItemTitle, typeof __VLS_components.vListItemTitle, typeof __VLS_components.VListItemTitle, typeof __VLS_components.vListItemTitle, ]} */ ;
    // @ts-ignore
    VListItemTitle;
    // @ts-ignore
    var __VLS_15 = __VLS_asFunctionalComponent(__VLS_14, new __VLS_14({}));
    var __VLS_16 = __VLS_15.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_15), false));
    var __VLS_18 = __VLS_17.slots.default;
    (__VLS_ctx.t("".concat(lang)));
    // @ts-ignore
    [t,];
};
var __VLS_17, __VLS_9;
for (var _i = 0, _b = __VLS_getVForSourceType((__VLS_ctx.availableLocales)); _i < _b.length; _i++) {
    var lang = _b[_i][0];
    _loop_1(lang);
}
var __VLS_3;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
