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
var settingsStore = (0, store_1.useSettingsStore)();
var t = (0, vue_i18n_1.useI18n)().t;
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
    modelValue: (__VLS_ctx.settingsStore.commonOptions.ignoreJpeg),
    hint: (__VLS_ctx.t('ignore_jpeg_hint')),
    label: (__VLS_ctx.t('ignore_jpeg')),
    color: "primary",
    persistentHint: true,
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.commonOptions.ignoreJpeg),
        hint: (__VLS_ctx.t('ignore_jpeg_hint')),
        label: (__VLS_ctx.t('ignore_jpeg')),
        color: "primary",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_1), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_5 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_6 = __VLS_asFunctionalComponent(__VLS_5, new __VLS_5({
    modelValue: (__VLS_ctx.settingsStore.commonOptions.overwrite),
    label: (__VLS_ctx.t('overwrite')),
    color: "primary",
    hideDetails: true,
}));
var __VLS_7 = __VLS_6.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.commonOptions.overwrite),
        label: (__VLS_ctx.t('overwrite')),
        color: "primary",
        hideDetails: true,
    }], __VLS_functionalComponentArgsRest(__VLS_6), false));
// @ts-ignore
[settingsStore, t,];
var __VLS_10 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10({
    modelValue: (__VLS_ctx.settingsStore.commonOptions.deleteOriginal),
    hint: (__VLS_ctx.t('delete_original_hint')),
    label: (__VLS_ctx.t('delete_original')),
    color: "red",
    persistentHint: true,
}));
var __VLS_12 = __VLS_11.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.commonOptions.deleteOriginal),
        hint: (__VLS_ctx.t('delete_original_hint')),
        label: (__VLS_ctx.t('delete_original')),
        color: "red",
        persistentHint: true,
    }], __VLS_functionalComponentArgsRest(__VLS_11), false));
// @ts-ignore
[settingsStore, t, t,];
var __VLS_15 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15({
    modelValue: (__VLS_ctx.settingsStore.commonOptions.recursive),
    label: (__VLS_ctx.t('recursive')),
    color: "primary",
    hideDetails: true,
}));
var __VLS_17 = __VLS_16.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.commonOptions.recursive),
        label: (__VLS_ctx.t('recursive')),
        color: "primary",
        hideDetails: true,
    }], __VLS_functionalComponentArgsRest(__VLS_16), false));
// @ts-ignore
[settingsStore, t,];
var __VLS_20 = {}.VSwitch;
/** @type {[typeof __VLS_components.VSwitch, typeof __VLS_components.vSwitch, ]} */ ;
// @ts-ignore
VSwitch;
// @ts-ignore
var __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, new __VLS_20({
    modelValue: (__VLS_ctx.settingsStore.commonOptions.sameDirectory),
    label: (__VLS_ctx.t('same_directory')),
    color: "primary",
    hideDetails: true,
}));
var __VLS_22 = __VLS_21.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.commonOptions.sameDirectory),
        label: (__VLS_ctx.t('same_directory')),
        color: "primary",
        hideDetails: true,
    }], __VLS_functionalComponentArgsRest(__VLS_21), false));
// @ts-ignore
[settingsStore, t,];
var __VLS_25 = {}.VTextField;
/** @type {[typeof __VLS_components.VTextField, typeof __VLS_components.vTextField, typeof __VLS_components.VTextField, typeof __VLS_components.vTextField, ]} */ ;
// @ts-ignore
VTextField;
// @ts-ignore
var __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25({
    modelValue: (__VLS_ctx.settingsStore.commonOptions.outputPath),
    disabled: (__VLS_ctx.settingsStore.commonOptions.sameDirectory),
    label: (__VLS_ctx.t('output_path')),
    readonly: true,
}));
var __VLS_27 = __VLS_26.apply(void 0, __spreadArray([{
        modelValue: (__VLS_ctx.settingsStore.commonOptions.outputPath),
        disabled: (__VLS_ctx.settingsStore.commonOptions.sameDirectory),
        label: (__VLS_ctx.t('output_path')),
        readonly: true,
    }], __VLS_functionalComponentArgsRest(__VLS_26), false));
var __VLS_29 = __VLS_28.slots.default;
// @ts-ignore
[settingsStore, settingsStore, t,];
{
    var __VLS_30 = __VLS_28.slots.append;
    var __VLS_31 = {}.VBtn;
    /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
    // @ts-ignore
    VBtn;
    // @ts-ignore
    var __VLS_32 = __VLS_asFunctionalComponent(__VLS_31, new __VLS_31(__assign({ 'onClick': {} }, { icon: "mdi-folder-open", variant: "plain" })));
    var __VLS_33 = __VLS_32.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { icon: "mdi-folder-open", variant: "plain" })], __VLS_functionalComponentArgsRest(__VLS_32), false));
    var __VLS_35 = void 0;
    var __VLS_36 = void 0;
    var __VLS_37 = ({ click: {} },
        { onClick: function () {
                var _a = [];
                for (var _i = 0; _i < arguments.length; _i++) {
                    _a[_i] = arguments[_i];
                }
                var $event = _a[0];
                __VLS_ctx.settingsStore.browseOutputPath();
                // @ts-ignore
                [settingsStore,];
            } });
    var __VLS_34;
}
var __VLS_28;
var __VLS_39 = {}.VBtn;
/** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
// @ts-ignore
VBtn;
// @ts-ignore
var __VLS_40 = __VLS_asFunctionalComponent(__VLS_39, new __VLS_39(__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })));
var __VLS_41 = __VLS_40.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { color: "warning", prependIcon: "mdi-rotate-left", variant: "text" })], __VLS_functionalComponentArgsRest(__VLS_40), false));
var __VLS_43;
var __VLS_44;
var __VLS_45 = ({ click: {} },
    { onClick: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.settingsStore.resetCommonOptions();
            // @ts-ignore
            [settingsStore,];
        } });
var __VLS_46 = __VLS_42.slots.default;
(__VLS_ctx.t('reset_common'));
// @ts-ignore
[t,];
var __VLS_42;
var __VLS_47 = {}.VBtn;
/** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
// @ts-ignore
VBtn;
// @ts-ignore
var __VLS_48 = __VLS_asFunctionalComponent(__VLS_47, new __VLS_47(__assign({ 'onClick': {} }, { color: "red", prependIcon: "mdi-rotate-left", variant: "text" })));
var __VLS_49 = __VLS_48.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { color: "red", prependIcon: "mdi-rotate-left", variant: "text" })], __VLS_functionalComponentArgsRest(__VLS_48), false));
var __VLS_51;
var __VLS_52;
var __VLS_53 = ({ click: {} },
    { onClick: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.settingsStore.reset();
            // @ts-ignore
            [settingsStore,];
        } });
var __VLS_54 = __VLS_50.slots.default;
(__VLS_ctx.t('reset_all'));
// @ts-ignore
[t,];
var __VLS_50;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
