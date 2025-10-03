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
var JxlTypes_1 = require("@/types/JxlTypes"); // Adjust the import path as needed
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
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.lossless),
    hint: (__VLS_ctx.t('lossless_hint')),
    label: (__VLS_ctx.t('lossless')),
    color: "primary",
    persistentHint: true,
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.lossless),
        hint: (__VLS_ctx.t('lossless_hint')),
        label: (__VLS_ctx.t('lossless')),
        color: "primary",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_1), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_5 = {}.VSlider;
/** @type {[typeof __VLS_components.VSlider, typeof __VLS_components.vSlider, ]} */ ;
// @ts-ignore
VSlider;
// @ts-ignore
var __VLS_6 = __VLS_asFunctionalComponent(__VLS_5, new __VLS_5({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.quality),
    disabled: (__VLS_ctx.settingsStore.jxlOptions.lossless),
    hint: (__VLS_ctx.t('quality_hint')),
    label: (__VLS_ctx.t('quality')),
    color: "primary",
    max: "15.0",
    min: "0.1",
    persistentHint: true,
    step: "0.1",
    thumbLabel: "always",
    type: "number",
}));
var __VLS_7 = __VLS_6.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.quality),
        disabled: (__VLS_ctx.settingsStore.jxlOptions.lossless),
        hint: (__VLS_ctx.t('quality_hint')),
        label: (__VLS_ctx.t('quality')),
        color: "primary",
        max: "15.0",
        min: "0.1",
        persistentHint: true,
        step: "0.1",
        thumbLabel: "always",
        type: "number",
    }], __VLS_functionalComponentArgsRest(__VLS_6), false));
// @ts-ignore
[settingsStore, settingsStore, t, t,];
var __VLS_10 = {}.VSelect;
/** @type {[typeof __VLS_components.VSelect, typeof __VLS_components.vSelect, ]} */ ;
// @ts-ignore
VSelect;
// @ts-ignore
var __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.speed),
    items: ([
        { text: '1, Lightning', value: __VLS_ctx.EncoderSpeed.Lightning },
        { text: '2, Thunder', value: __VLS_ctx.EncoderSpeed.Thunder },
        { text: '3, Falcon', value: __VLS_ctx.EncoderSpeed.Falcon },
        { text: '4, Cheetah', value: __VLS_ctx.EncoderSpeed.Cheetah },
        { text: '5, Hare', value: __VLS_ctx.EncoderSpeed.Hare },
        { text: '6, Wombat', value: __VLS_ctx.EncoderSpeed.Wombat },
        { text: '7, Squirrel', value: __VLS_ctx.EncoderSpeed.Squirrel },
        { text: '8, Tortoise', value: __VLS_ctx.EncoderSpeed.Tortoise },
        { text: '9, Kitten', value: __VLS_ctx.EncoderSpeed.Kitten },
        { text: '10, Glacier', value: __VLS_ctx.EncoderSpeed.Glacier }
    ]),
    hint: (__VLS_ctx.t('speed_hint')),
    label: (__VLS_ctx.t('speed')),
    itemTitle: "text",
    itemValue: "value",
    persistentHint: true,
}));
var __VLS_12 = __VLS_11.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.speed),
        items: ([
            { text: '1, Lightning', value: __VLS_ctx.EncoderSpeed.Lightning },
            { text: '2, Thunder', value: __VLS_ctx.EncoderSpeed.Thunder },
            { text: '3, Falcon', value: __VLS_ctx.EncoderSpeed.Falcon },
            { text: '4, Cheetah', value: __VLS_ctx.EncoderSpeed.Cheetah },
            { text: '5, Hare', value: __VLS_ctx.EncoderSpeed.Hare },
            { text: '6, Wombat', value: __VLS_ctx.EncoderSpeed.Wombat },
            { text: '7, Squirrel', value: __VLS_ctx.EncoderSpeed.Squirrel },
            { text: '8, Tortoise', value: __VLS_ctx.EncoderSpeed.Tortoise },
            { text: '9, Kitten', value: __VLS_ctx.EncoderSpeed.Kitten },
            { text: '10, Glacier', value: __VLS_ctx.EncoderSpeed.Glacier }
        ]),
        hint: (__VLS_ctx.t('speed_hint')),
        label: (__VLS_ctx.t('speed')),
        itemTitle: "text",
        itemValue: "value",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_11), false));
// @ts-ignore
[settingsStore, t, t, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed, JxlTypes_1.EncoderSpeed,];
var __VLS_15 = {}.VRow;
/** @type {[typeof __VLS_components.VRow, typeof __VLS_components.vRow, typeof __VLS_components.VRow, typeof __VLS_components.vRow, ]} */ ;
// @ts-ignore
VRow;
// @ts-ignore
var __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15({}));
var __VLS_17 = __VLS_16.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_16), false));
var __VLS_19 = __VLS_18.slots.default;
var __VLS_20 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, new __VLS_20({}));
var __VLS_22 = __VLS_21.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_21), false));
var __VLS_24 = __VLS_23.slots.default;
var __VLS_25 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.useContainer),
    hint: (__VLS_ctx.t('use_container_hint')),
    label: (__VLS_ctx.t('use_container')),
    color: "primary",
    persistentHint: true,
}));
var __VLS_27 = __VLS_26.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.useContainer),
        hint: (__VLS_ctx.t('use_container_hint')),
        label: (__VLS_ctx.t('use_container')),
        color: "primary",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_26), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_23;
var __VLS_30 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_31 = __VLS_asFunctionalComponent(__VLS_30, new __VLS_30({}));
var __VLS_32 = __VLS_31.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_31), false));
var __VLS_34 = __VLS_33.slots.default;
var __VLS_35 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_36 = __VLS_asFunctionalComponent(__VLS_35, new __VLS_35({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.usesOriginalProfile),
    disabled: (__VLS_ctx.settingsStore.jxlOptions.lossless),
    hint: (__VLS_ctx.t('uses_original_profile_hint')),
    label: (__VLS_ctx.t('uses_original_profile')),
    color: "primary",
    persistentHint: true,
}));
var __VLS_37 = __VLS_36.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.usesOriginalProfile),
        disabled: (__VLS_ctx.settingsStore.jxlOptions.lossless),
        hint: (__VLS_ctx.t('uses_original_profile_hint')),
        label: (__VLS_ctx.t('uses_original_profile')),
        color: "primary",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_36), false));
// @ts-ignore
[settingsStore, settingsStore, t, t,];
var __VLS_33;
var __VLS_18;
var __VLS_40 = {}.VSlider;
/** @type {[typeof __VLS_components.VSlider, typeof __VLS_components.vSlider, ]} */ ;
// @ts-ignore
VSlider;
// @ts-ignore
var __VLS_41 = __VLS_asFunctionalComponent(__VLS_40, new __VLS_40({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.decodingSpeed),
    hint: (__VLS_ctx.t('decoding_speed_hint')),
    label: (__VLS_ctx.t('decoding_speed')),
    color: "primary",
    max: "4",
    min: "0",
    persistentHint: true,
    step: "1",
    thumbLabel: "always",
    type: "number",
}));
var __VLS_42 = __VLS_41.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.decodingSpeed),
        hint: (__VLS_ctx.t('decoding_speed_hint')),
        label: (__VLS_ctx.t('decoding_speed')),
        color: "primary",
        max: "4",
        min: "0",
        persistentHint: true,
        step: "1",
        thumbLabel: "always",
        type: "number",
    }], __VLS_functionalComponentArgsRest(__VLS_41), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_45 = {}.VRow;
/** @type {[typeof __VLS_components.VRow, typeof __VLS_components.vRow, typeof __VLS_components.VRow, typeof __VLS_components.vRow, ]} */ ;
// @ts-ignore
VRow;
// @ts-ignore
var __VLS_46 = __VLS_asFunctionalComponent(__VLS_45, new __VLS_45({}));
var __VLS_47 = __VLS_46.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_46), false));
var __VLS_49 = __VLS_48.slots.default;
var __VLS_50 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_51 = __VLS_asFunctionalComponent(__VLS_50, new __VLS_50({}));
var __VLS_52 = __VLS_51.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_51), false));
var __VLS_54 = __VLS_53.slots.default;
var __VLS_55 = {}.VNumberInput;
/** @type {[typeof __VLS_components.VNumberInput, typeof __VLS_components.vNumberInput, ]} */ ;
// @ts-ignore
VNumberInput;
// @ts-ignore
var __VLS_56 = __VLS_asFunctionalComponent(__VLS_55, new __VLS_55({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.initBufferSize),
    hint: (__VLS_ctx.t('init_buffer_size_hint')),
    label: (__VLS_ctx.t('init_buffer_size')),
    min: (32),
    step: (32),
    clearable: true,
    persistentHint: true,
    type: "number",
}));
var __VLS_57 = __VLS_56.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.initBufferSize),
        hint: (__VLS_ctx.t('init_buffer_size_hint')),
        label: (__VLS_ctx.t('init_buffer_size')),
        min: (32),
        step: (32),
        clearable: true,
        persistentHint: true,
        type: "number",
    }], __VLS_functionalComponentArgsRest(__VLS_56), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_53;
var __VLS_60 = {}.VCol;
/** @type {[typeof __VLS_components.VCol, typeof __VLS_components.vCol, typeof __VLS_components.VCol, typeof __VLS_components.vCol, ]} */ ;
// @ts-ignore
VCol;
// @ts-ignore
var __VLS_61 = __VLS_asFunctionalComponent(__VLS_60, new __VLS_60({}));
var __VLS_62 = __VLS_61.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_61), false));
var __VLS_64 = __VLS_63.slots.default;
var __VLS_65 = {}.VSelect;
/** @type {[typeof __VLS_components.VSelect, typeof __VLS_components.vSelect, ]} */ ;
// @ts-ignore
VSelect;
// @ts-ignore
var __VLS_66 = __VLS_asFunctionalComponent(__VLS_65, new __VLS_65({
    modelValue: (__VLS_ctx.settingsStore.jxlOptions.colorEncoding),
    items: ([
        { text: 'Srgb', value: __VLS_ctx.ColorEncoding.Srgb },
        { text: 'LinearSrgb', value: __VLS_ctx.ColorEncoding.LinearSrgb },
        { text: 'SrgbLuma', value: __VLS_ctx.ColorEncoding.SrgbLuma },
        { text: 'LinearSrgbLuma', value: __VLS_ctx.ColorEncoding.LinearSrgbLuma }
    ]),
    hint: (__VLS_ctx.t('color_encoding_hint')),
    label: (__VLS_ctx.t('color_encoding')),
    itemTitle: "text",
    itemValue: "value",
    persistentHint: true,
}));
var __VLS_67 = __VLS_66.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.jxlOptions.colorEncoding),
        items: ([
            { text: 'Srgb', value: __VLS_ctx.ColorEncoding.Srgb },
            { text: 'LinearSrgb', value: __VLS_ctx.ColorEncoding.LinearSrgb },
            { text: 'SrgbLuma', value: __VLS_ctx.ColorEncoding.SrgbLuma },
            { text: 'LinearSrgbLuma', value: __VLS_ctx.ColorEncoding.LinearSrgbLuma }
        ]),
        hint: (__VLS_ctx.t('color_encoding_hint')),
        label: (__VLS_ctx.t('color_encoding')),
        itemTitle: "text",
        itemValue: "value",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_66), false));
// @ts-ignore
[settingsStore, t, t, JxlTypes_1.ColorEncoding, JxlTypes_1.ColorEncoding, JxlTypes_1.ColorEncoding, JxlTypes_1.ColorEncoding,];
var __VLS_63;
var __VLS_48;
var __VLS_70 = {}.VBtn;
/** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
// @ts-ignore
VBtn;
// @ts-ignore
var __VLS_71 = __VLS_asFunctionalComponent(__VLS_70, new __VLS_70(__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })));
var __VLS_72 = __VLS_71.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })], __VLS_functionalComponentArgsRest(__VLS_71), false));
var __VLS_74;
var __VLS_75;
var __VLS_76 = ({ click: {} },
    { onClick: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.settingsStore.resetJxlOptions();
            // @ts-ignore
            [settingsStore,];
        } });
var __VLS_77 = __VLS_73.slots.default;
(__VLS_ctx.t('reset_jxl_options'));
// @ts-ignore
[t,];
var __VLS_73;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
