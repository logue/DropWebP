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
var vue_1 = require("vue");
var vue_i18n_1 = require("vue-i18n");
var ProgressDialog_vue_1 = require("./modals/ProgressDialog.vue");
var useImageConversionController_1 = require("@/composables/useImageConversionController");
var useLogger_1 = require("@/composables/useLogger");
var settingsStore = (0, store_1.useSettingsStore)();
var t = (0, vue_i18n_1.useI18n)().t;
(0, useLogger_1.useLogger)();
var _a = (0, useImageConversionController_1.useImageConversionController)(t), dialog = _a.dialog, inProgress = _a.inProgress, currentFile = _a.currentFile, progress = _a.progress, message = _a.message, convertByDialog = _a.convertByDialog;
var isEnter = (0, vue_1.ref)(false);
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VContainer;
/** @type {[typeof __VLS_components.VContainer, typeof __VLS_components.vContainer, typeof __VLS_components.VContainer, typeof __VLS_components.vContainer, ]} */ ;
// @ts-ignore
VContainer;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0(__assign(__assign(__assign({ 'onDragenter': {} }, { 'onDragleave': {} }), { 'onDrop': {} }), { class: "fill-height pa-0 d-flex flex-column justify-center" })));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([__assign(__assign(__assign({ 'onDragenter': {} }, { 'onDragleave': {} }), { 'onDrop': {} }), { class: "fill-height pa-0 d-flex flex-column justify-center" })], __VLS_functionalComponentArgsRest(__VLS_1), false));
var __VLS_4;
var __VLS_5;
var __VLS_6 = ({ dragenter: {} },
    { onDragenter: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.isEnter = true;
            // @ts-ignore
            [isEnter,];
        } });
var __VLS_7 = ({ dragleave: {} },
    { onDragleave: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.isEnter = false;
            // @ts-ignore
            [isEnter,];
        } });
var __VLS_8 = ({ drop: {} },
    { onDrop: function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.isEnter = false;
            // @ts-ignore
            [isEnter,];
        } });
var __VLS_9 = __VLS_3.slots.default;
var __VLS_10 = {}.VSheet;
/** @type {[typeof __VLS_components.VSheet, typeof __VLS_components.vSheet, typeof __VLS_components.VSheet, typeof __VLS_components.vSheet, ]} */ ;
// @ts-ignore
VSheet;
// @ts-ignore
var __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10(__assign(__assign({ class: (__VLS_ctx.isEnter ? 'bg-green-lighten-5' : '') }, { class: "d-flex flex-grow-1 align-center justify-center my-4 px-15" }), { rounded: "xl" })));
var __VLS_12 = __VLS_11.apply(void 0, __spreadArray([__assign(__assign({ class: (__VLS_ctx.isEnter ? 'bg-green-lighten-5' : '') }, { class: "d-flex flex-grow-1 align-center justify-center my-4 px-15" }), { rounded: "xl" })], __VLS_functionalComponentArgsRest(__VLS_11), false));
var __VLS_14 = __VLS_13.slots.default;
// @ts-ignore
[isEnter,];
__VLS_asFunctionalElement(__VLS_elements.h2, __VLS_elements.h2)(__assign({ class: "text-center text-medium-emphasis" }));
(__VLS_ctx.t('hero_text'));
// @ts-ignore
[t,];
var __VLS_13;
var __VLS_15 = {}.VSheet;
/** @type {[typeof __VLS_components.VSheet, typeof __VLS_components.vSheet, typeof __VLS_components.VSheet, typeof __VLS_components.vSheet, ]} */ ;
// @ts-ignore
VSheet;
// @ts-ignore
var __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15(__assign({ class: "d-flex bg-transparent" })));
var __VLS_17 = __VLS_16.apply(void 0, __spreadArray([__assign({ class: "d-flex bg-transparent" })], __VLS_functionalComponentArgsRest(__VLS_16), false));
var __VLS_19 = __VLS_18.slots.default;
var __VLS_20 = {}.VBtn;
/** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
// @ts-ignore
VBtn;
// @ts-ignore
var __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, new __VLS_20(__assign(__assign({ 'onClick': {} }, { prependIcon: "mdi-file-multiple" }), { class: "mr-2" })));
var __VLS_22 = __VLS_21.apply(void 0, __spreadArray([__assign(__assign({ 'onClick': {} }, { prependIcon: "mdi-file-multiple" }), { class: "mr-2" })], __VLS_functionalComponentArgsRest(__VLS_21), false));
var __VLS_24;
var __VLS_25;
var __VLS_26 = ({ click: {} },
    { onClick: (__VLS_ctx.convertByDialog) });
var __VLS_27 = __VLS_23.slots.default;
// @ts-ignore
[convertByDialog,];
(__VLS_ctx.t('select_files'));
// @ts-ignore
[t,];
var __VLS_23;
var __VLS_28 = {}.VRadioGroup;
/** @type {[typeof __VLS_components.VRadioGroup, typeof __VLS_components.vRadioGroup, typeof __VLS_components.VRadioGroup, typeof __VLS_components.vRadioGroup, ]} */ ;
// @ts-ignore
VRadioGroup;
// @ts-ignore
var __VLS_29 = __VLS_asFunctionalComponent(__VLS_28, new __VLS_28(__assign(__assign({ modelValue: (__VLS_ctx.settingsStore.commonOptions.format), label: (__VLS_ctx.t('convert_to')) }, { class: "d-flex justify-end" }), { inline: true })));
var __VLS_30 = __VLS_29.apply(void 0, __spreadArray([__assign(__assign({ modelValue: (__VLS_ctx.settingsStore.commonOptions.format), label: (__VLS_ctx.t('convert_to')) }, { class: "d-flex justify-end" }), { inline: true })], __VLS_functionalComponentArgsRest(__VLS_29), false));
var __VLS_32 = __VLS_31.slots.default;
// @ts-ignore
[t, settingsStore,];
var __VLS_33 = {}.VTooltip;
/** @type {[typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, ]} */ ;
// @ts-ignore
VTooltip;
// @ts-ignore
var __VLS_34 = __VLS_asFunctionalComponent(__VLS_33, new __VLS_33({
    text: (__VLS_ctx.t('type.webp_description')),
    location: "top",
}));
var __VLS_35 = __VLS_34.apply(void 0, __spreadArray([{
        text: (__VLS_ctx.t('type.webp_description')),
        location: "top",
    }], __VLS_functionalComponentArgsRest(__VLS_34), false));
var __VLS_37 = __VLS_36.slots.default;
// @ts-ignore
[t,];
{
    var __VLS_38 = __VLS_36.slots.activator;
    var props = __VLS_getSlotParameters(__VLS_38)[0].props;
    var __VLS_39 = {}.VRadio;
    /** @type {[typeof __VLS_components.VRadio, typeof __VLS_components.vRadio, ]} */ ;
    // @ts-ignore
    VRadio;
    // @ts-ignore
    var __VLS_40 = __VLS_asFunctionalComponent(__VLS_39, new __VLS_39(__assign(__assign({}, (props)), { label: (__VLS_ctx.t('type.webp')), value: "webp", color: "green" })));
    var __VLS_41 = __VLS_40.apply(void 0, __spreadArray([__assign(__assign({}, (props)), { label: (__VLS_ctx.t('type.webp')), value: "webp", color: "green" })], __VLS_functionalComponentArgsRest(__VLS_40), false));
    // @ts-ignore
    [t,];
}
var __VLS_36;
var __VLS_44 = {}.VTooltip;
/** @type {[typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, ]} */ ;
// @ts-ignore
VTooltip;
// @ts-ignore
var __VLS_45 = __VLS_asFunctionalComponent(__VLS_44, new __VLS_44({
    text: (__VLS_ctx.t('type.avif_description')),
    location: "top",
}));
var __VLS_46 = __VLS_45.apply(void 0, __spreadArray([{
        text: (__VLS_ctx.t('type.avif_description')),
        location: "top",
    }], __VLS_functionalComponentArgsRest(__VLS_45), false));
var __VLS_48 = __VLS_47.slots.default;
// @ts-ignore
[t,];
{
    var __VLS_49 = __VLS_47.slots.activator;
    var props = __VLS_getSlotParameters(__VLS_49)[0].props;
    var __VLS_50 = {}.VRadio;
    /** @type {[typeof __VLS_components.VRadio, typeof __VLS_components.vRadio, ]} */ ;
    // @ts-ignore
    VRadio;
    // @ts-ignore
    var __VLS_51 = __VLS_asFunctionalComponent(__VLS_50, new __VLS_50(__assign(__assign({}, (props)), { label: (__VLS_ctx.t('type.avif')), value: "avif", color: "red" })));
    var __VLS_52 = __VLS_51.apply(void 0, __spreadArray([__assign(__assign({}, (props)), { label: (__VLS_ctx.t('type.avif')), value: "avif", color: "red" })], __VLS_functionalComponentArgsRest(__VLS_51), false));
    // @ts-ignore
    [t,];
}
var __VLS_47;
var __VLS_55 = {}.VTooltip;
/** @type {[typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, ]} */ ;
// @ts-ignore
VTooltip;
// @ts-ignore
var __VLS_56 = __VLS_asFunctionalComponent(__VLS_55, new __VLS_55({
    text: (__VLS_ctx.t('type.jxl_description')),
    location: "top",
}));
var __VLS_57 = __VLS_56.apply(void 0, __spreadArray([{
        text: (__VLS_ctx.t('type.jxl_description')),
        location: "top",
    }], __VLS_functionalComponentArgsRest(__VLS_56), false));
var __VLS_59 = __VLS_58.slots.default;
// @ts-ignore
[t,];
{
    var __VLS_60 = __VLS_58.slots.activator;
    var props = __VLS_getSlotParameters(__VLS_60)[0].props;
    var __VLS_61 = {}.VRadio;
    /** @type {[typeof __VLS_components.VRadio, typeof __VLS_components.vRadio, typeof __VLS_components.VRadio, typeof __VLS_components.vRadio, ]} */ ;
    // @ts-ignore
    VRadio;
    // @ts-ignore
    var __VLS_62 = __VLS_asFunctionalComponent(__VLS_61, new __VLS_61(__assign(__assign({}, (props)), { label: (__VLS_ctx.t('type.jxl')), value: "jxl", color: "blue" })));
    var __VLS_63 = __VLS_62.apply(void 0, __spreadArray([__assign(__assign({}, (props)), { label: (__VLS_ctx.t('type.jxl')), value: "jxl", color: "blue" })], __VLS_functionalComponentArgsRest(__VLS_62), false));
    var __VLS_65 = __VLS_64.slots.default;
    // @ts-ignore
    [t,];
    {
        var __VLS_66 = __VLS_64.slots.label;
        (__VLS_ctx.t('type.jxl'));
        // @ts-ignore
        [t,];
        __VLS_asFunctionalElement(__VLS_elements.small, __VLS_elements.small)(__assign({ class: "text-grey" }));
        (__VLS_ctx.t('experimental'));
        // @ts-ignore
        [t,];
    }
    var __VLS_64;
}
var __VLS_58;
var __VLS_31;
var __VLS_18;
var __VLS_3;
/** @type {[typeof ProgressDialog, ]} */ ;
// @ts-ignore
var __VLS_67 = __VLS_asFunctionalComponent(ProgressDialog_vue_1.default, new ProgressDialog_vue_1.default({
    currentFile: (__VLS_ctx.currentFile),
    dialog: (__VLS_ctx.dialog),
    inProgress: (__VLS_ctx.inProgress),
    progress: (__VLS_ctx.progress),
    title: (__VLS_ctx.message),
}));
var __VLS_68 = __VLS_67.apply(void 0, __spreadArray([{
        currentFile: (__VLS_ctx.currentFile),
        dialog: (__VLS_ctx.dialog),
        inProgress: (__VLS_ctx.inProgress),
        progress: (__VLS_ctx.progress),
        title: (__VLS_ctx.message),
    }], __VLS_functionalComponentArgsRest(__VLS_67), false));
// @ts-ignore
[currentFile, dialog, inProgress, progress, message,];
/** @type {__VLS_StyleScopedClasses['fill-height']} */ ;
/** @type {__VLS_StyleScopedClasses['pa-0']} */ ;
/** @type {__VLS_StyleScopedClasses['d-flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-column']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['d-flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-grow-1']} */ ;
/** @type {__VLS_StyleScopedClasses['align-center']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['my-4']} */ ;
/** @type {__VLS_StyleScopedClasses['px-15']} */ ;
/** @type {__VLS_StyleScopedClasses['text-center']} */ ;
/** @type {__VLS_StyleScopedClasses['text-medium-emphasis']} */ ;
/** @type {__VLS_StyleScopedClasses['d-flex']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-transparent']} */ ;
/** @type {__VLS_StyleScopedClasses['mr-2']} */ ;
/** @type {__VLS_StyleScopedClasses['d-flex']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-end']} */ ;
/** @type {__VLS_StyleScopedClasses['text-grey']} */ ;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
