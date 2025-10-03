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
var AvifTypes_1 = require("@/types/AvifTypes");
var t = (0, vue_i18n_1.useI18n)().t;
var settingsStore = (0, store_1.useSettingsStore)();
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VRow;
/** @type {[typeof __VLS_components.VRow, typeof __VLS_components.vRow, typeof __VLS_components.VRow, typeof __VLS_components.vRow, ]} */ ;
// @ts-ignore
VRow;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_1), false));
var __VLS_4 = __VLS_3.slots.default;
var __VLS_5 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_6 = __VLS_asFunctionalComponent(__VLS_5, new __VLS_5({}));
var __VLS_7 = __VLS_6.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_6), false));
var __VLS_9 = __VLS_8.slots.default;
var __VLS_10 = {}.VSelect;
/** @type {[typeof __VLS_components.VSelect, typeof __VLS_components.vSelect, ]} */ ;
// @ts-ignore
VSelect;
// @ts-ignore
var __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.bitDepth),
    items: ([
        { text: __VLS_ctx.t('bit_depth_8'), value: __VLS_ctx.BitDepth.Eight },
        { text: __VLS_ctx.t('bit_depth_10'), value: __VLS_ctx.BitDepth.Ten },
        { text: __VLS_ctx.t('bit_depth_auto'), value: __VLS_ctx.BitDepth.Auto }
    ]),
    hint: (__VLS_ctx.t('bit_depth_hint')),
    label: (__VLS_ctx.t('bit_depth')),
    itemTitle: "text",
    itemValue: "value",
    persistentHint: true,
}));
var __VLS_12 = __VLS_11.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.bitDepth),
        items: ([
            { text: __VLS_ctx.t('bit_depth_8'), value: __VLS_ctx.BitDepth.Eight },
            { text: __VLS_ctx.t('bit_depth_10'), value: __VLS_ctx.BitDepth.Ten },
            { text: __VLS_ctx.t('bit_depth_auto'), value: __VLS_ctx.BitDepth.Auto }
        ]),
        hint: (__VLS_ctx.t('bit_depth_hint')),
        label: (__VLS_ctx.t('bit_depth')),
        itemTitle: "text",
        itemValue: "value",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_11), false));
// @ts-ignore
[settingsStore, t, t, t, t, t, AvifTypes_1.BitDepth, AvifTypes_1.BitDepth, AvifTypes_1.BitDepth,];
var __VLS_8;
var __VLS_15 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15({}));
var __VLS_17 = __VLS_16.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_16), false));
var __VLS_19 = __VLS_18.slots.default;
var __VLS_20 = {}.VSelect;
/** @type {[typeof __VLS_components.VSelect, typeof __VLS_components.vSelect, ]} */ ;
// @ts-ignore
VSelect;
// @ts-ignore
var __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, new __VLS_20({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.alphaColorMode),
    items: ([
        { text: 'UnassociatedDirty', value: __VLS_ctx.AlphaColorMode.UnassociatedDirty },
        { text: 'UnassociatedClean', value: __VLS_ctx.AlphaColorMode.UnassociatedClean },
        { text: 'Premultiplied', value: __VLS_ctx.AlphaColorMode.Premultiplied }
    ]),
    hint: (__VLS_ctx.t('alpha_color_mode_hint')),
    label: (__VLS_ctx.t('alpha_color_mode')),
    itemTitle: "text",
    itemValue: "value",
    persistentHint: true,
}));
var __VLS_22 = __VLS_21.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.alphaColorMode),
        items: ([
            { text: 'UnassociatedDirty', value: __VLS_ctx.AlphaColorMode.UnassociatedDirty },
            { text: 'UnassociatedClean', value: __VLS_ctx.AlphaColorMode.UnassociatedClean },
            { text: 'Premultiplied', value: __VLS_ctx.AlphaColorMode.Premultiplied }
        ]),
        hint: (__VLS_ctx.t('alpha_color_mode_hint')),
        label: (__VLS_ctx.t('alpha_color_mode')),
        itemTitle: "text",
        itemValue: "value",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_21), false));
// @ts-ignore
[settingsStore, t, t, AvifTypes_1.AlphaColorMode, AvifTypes_1.AlphaColorMode, AvifTypes_1.AlphaColorMode,];
var __VLS_18;
var __VLS_3;
var __VLS_25 = {}.VSlider;
/** @type {[typeof __VLS_components.VSlider, typeof __VLS_components.vSlider, ]} */ ;
// @ts-ignore
VSlider;
// @ts-ignore
var __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.quality),
    label: (__VLS_ctx.t('quality')),
    max: (100),
    min: (1),
    color: "primary",
    persistentHint: true,
    step: "0.1",
    thumbLabel: "always",
    type: "number",
}));
var __VLS_27 = __VLS_26.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.quality),
        label: (__VLS_ctx.t('quality')),
        max: (100),
        min: (1),
        color: "primary",
        persistentHint: true,
        step: "0.1",
        thumbLabel: "always",
        type: "number",
    }], __VLS_functionalComponentArgsRest(__VLS_26), false));
// @ts-ignore
[settingsStore, t,];
var __VLS_30 = {}.VSlider;
/** @type {[typeof __VLS_components.VSlider, typeof __VLS_components.vSlider, ]} */ ;
// @ts-ignore
VSlider;
// @ts-ignore
var __VLS_31 = __VLS_asFunctionalComponent(__VLS_30, new __VLS_30({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.alphaQuality),
    label: (__VLS_ctx.t('alpha_quality')),
    max: (100),
    min: (1),
    color: "primary",
    step: "0.1",
    thumbLabel: "always",
    persistentHint: true,
}));
var __VLS_32 = __VLS_31.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.alphaQuality),
        label: (__VLS_ctx.t('alpha_quality')),
        max: (100),
        min: (1),
        color: "primary",
        step: "0.1",
        thumbLabel: "always",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_31), false));
// @ts-ignore
[settingsStore, t,];
var __VLS_35 = {}.VSlider;
/** @type {[typeof __VLS_components.VSlider, typeof __VLS_components.vSlider, ]} */ ;
// @ts-ignore
VSlider;
// @ts-ignore
var __VLS_36 = __VLS_asFunctionalComponent(__VLS_35, new __VLS_35({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.speed),
    hint: (__VLS_ctx.t('speed_hint')),
    label: (__VLS_ctx.t('speed')),
    max: (10),
    min: (1),
    color: "primary",
    persistentHint: true,
    step: "1",
    thumbLabel: "always",
    type: "number",
}));
var __VLS_37 = __VLS_36.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.speed),
        hint: (__VLS_ctx.t('speed_hint')),
        label: (__VLS_ctx.t('speed')),
        max: (10),
        min: (1),
        color: "primary",
        persistentHint: true,
        step: "1",
        thumbLabel: "always",
        type: "number",
    }], __VLS_functionalComponentArgsRest(__VLS_36), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_40 = {}.VRow;
/** @type {[typeof __VLS_components.VRow, typeof __VLS_components.vRow, typeof __VLS_components.VRow, typeof __VLS_components.vRow, ]} */ ;
// @ts-ignore
VRow;
// @ts-ignore
var __VLS_41 = __VLS_asFunctionalComponent(__VLS_40, new __VLS_40({}));
var __VLS_42 = __VLS_41.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_41), false));
var __VLS_44 = __VLS_43.slots.default;
var __VLS_45 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_46 = __VLS_asFunctionalComponent(__VLS_45, new __VLS_45({}));
var __VLS_47 = __VLS_46.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_46), false));
var __VLS_49 = __VLS_48.slots.default;
var __VLS_50 = {}.VSelect;
/** @type {[typeof __VLS_components.VSelect, typeof __VLS_components.vSelect, ]} */ ;
// @ts-ignore
VSelect;
// @ts-ignore
var __VLS_51 = __VLS_asFunctionalComponent(__VLS_50, new __VLS_50({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.colorModel),
    items: ([
        { text: 'YCbCr', value: __VLS_ctx.ColorModel.YCbCr },
        { text: 'RGB', value: __VLS_ctx.ColorModel.RGB }
    ]),
    hint: (__VLS_ctx.t('color_model_hint')),
    label: (__VLS_ctx.t('color_model')),
    itemTitle: "text",
    itemValue: "value",
    persistentHint: true,
}));
var __VLS_52 = __VLS_51.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.colorModel),
        items: ([
            { text: 'YCbCr', value: __VLS_ctx.ColorModel.YCbCr },
            { text: 'RGB', value: __VLS_ctx.ColorModel.RGB }
        ]),
        hint: (__VLS_ctx.t('color_model_hint')),
        label: (__VLS_ctx.t('color_model')),
        itemTitle: "text",
        itemValue: "value",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_51), false));
// @ts-ignore
[settingsStore, t, t, AvifTypes_1.ColorModel, AvifTypes_1.ColorModel,];
var __VLS_48;
var __VLS_55 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_56 = __VLS_asFunctionalComponent(__VLS_55, new __VLS_55({}));
var __VLS_57 = __VLS_56.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_56), false));
var __VLS_59 = __VLS_58.slots.default;
var __VLS_60 = {}.VNumberInput;
/** @type {[typeof __VLS_components.VNumberInput, typeof __VLS_components.vNumberInput, ]} */ ;
// @ts-ignore
VNumberInput;
// @ts-ignore
var __VLS_61 = __VLS_asFunctionalComponent(__VLS_60, new __VLS_60({
    modelValue: (__VLS_ctx.settingsStore.avifOptions.threads),
    hint: (__VLS_ctx.t('threads_hint')),
    label: (__VLS_ctx.t('threads')),
    max: (10),
    min: (1),
    clearable: true,
    type: "number",
    persistentHint: true,
}));
var __VLS_62 = __VLS_61.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.avifOptions.threads),
        hint: (__VLS_ctx.t('threads_hint')),
        label: (__VLS_ctx.t('threads')),
        max: (10),
        min: (1),
        clearable: true,
        type: "number",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_61), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_58;
var __VLS_43;
var __VLS_65 = {}.VBtn;
/** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
// @ts-ignore
VBtn;
// @ts-ignore
var __VLS_66 = __VLS_asFunctionalComponent(__VLS_65, new __VLS_65(__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })));
var __VLS_67 = __VLS_66.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })], __VLS_functionalComponentArgsRest(__VLS_66), false));
var __VLS_69;
var __VLS_70;
var __VLS_71 = ({ click: {} },
    { onClick: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.settingsStore.resetAvifOptions();
            // @ts-ignore
            [settingsStore,];
        } });
var __VLS_72 = __VLS_68.slots.default;
(__VLS_ctx.t('reset_avif_options'));
// @ts-ignore
[t,];
var __VLS_68;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
