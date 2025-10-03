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
// import { WebPImageHint } from '@/types/WebpTypes';
var t = (0, vue_i18n_1.useI18n)().t;
var settingsStore = (0, store_1.useSettingsStore)();
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({
    modelValue: (__VLS_ctx.settingsStore.webpOptions.lossless),
    label: (__VLS_ctx.t('lossless')),
    color: "primary",
    inline: true,
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.webpOptions.lossless),
        label: (__VLS_ctx.t('lossless')),
        color: "primary",
        inline: true,
    }], __VLS_functionalComponentArgsRest(__VLS_1), false));
// @ts-ignore
[settingsStore, t,];
var __VLS_5 = {}.VSlider;
/** @type {[typeof __VLS_components.VSlider, typeof __VLS_components.vSlider, ]} */ ;
// @ts-ignore
VSlider;
// @ts-ignore
var __VLS_6 = __VLS_asFunctionalComponent(__VLS_5, new __VLS_5({
    modelValue: (__VLS_ctx.settingsStore.webpOptions.quality),
    disabled: (__VLS_ctx.settingsStore.webpOptions.lossless),
    label: (__VLS_ctx.t('quality')),
    max: (100),
    min: (0),
    color: "primary",
    step: "0.1",
    thumbLabel: "always",
    type: "number",
}));
var __VLS_7 = __VLS_6.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.webpOptions.quality),
        disabled: (__VLS_ctx.settingsStore.webpOptions.lossless),
        label: (__VLS_ctx.t('quality')),
        max: (100),
        min: (0),
        color: "primary",
        step: "0.1",
        thumbLabel: "always",
        type: "number",
    }], __VLS_functionalComponentArgsRest(__VLS_6), false));
// @ts-ignore
[settingsStore, settingsStore, t,];
var __VLS_10 = {}.VBtn;
/** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
// @ts-ignore
VBtn;
// @ts-ignore
var __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10(__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })));
var __VLS_12 = __VLS_11.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })], __VLS_functionalComponentArgsRest(__VLS_11), false));
var __VLS_14;
var __VLS_15;
var __VLS_16 = ({ click: {} },
    { onClick: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.settingsStore.resetWebpOptions();
            // @ts-ignore
            [settingsStore,];
        } });
var __VLS_17 = __VLS_13.slots.default;
(__VLS_ctx.t('reset_webp_options'));
// @ts-ignore
[t,];
var __VLS_13;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
