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
var vue_i18n_1 = require("vue-i18n");
var t = (0, vue_i18n_1.useI18n)().t;
var __VLS_props = defineProps({
    dialog: { type: Boolean, default: false },
    title: { type: String, required: true },
    currentFile: { type: String, default: '' },
    progress: {
        type: Number,
        required: false,
        default: 0
    },
    inProgress: { type: Boolean, default: false }
});
var emit = defineEmits();
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign(__assign(__assign(__assign({}, {}), {}), {}), {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VDialog;
/** @type {[typeof __VLS_components.VDialog, typeof __VLS_components.vDialog, typeof __VLS_components.VDialog, typeof __VLS_components.vDialog, ]} */ ;
// @ts-ignore
VDialog;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0(__assign(__assign(__assign({ 'onUpdate:modelValue': {} }, { modelValue: (__VLS_ctx.dialog), persistent: true }), { style: {} }), { width: "auto" })));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([__assign(__assign(__assign({ 'onUpdate:modelValue': {} }, { modelValue: (__VLS_ctx.dialog), persistent: true }), { style: {} }), { width: "auto" })], __VLS_functionalComponentArgsRest(__VLS_1), false));
var __VLS_4;
var __VLS_5;
var __VLS_6 = ({ 'update:modelValue': {} },
    { 'onUpdate:modelValue': function () {
            var _a = [];
            for (var _i = 0; _i < arguments.length; _i++) {
                _a[_i] = arguments[_i];
            }
            var $event = _a[0];
            __VLS_ctx.emit('update:dialog', $event);
            // @ts-ignore
            [dialog, emit,];
        } });
var __VLS_7 = {};
var __VLS_8 = __VLS_3.slots.default;
var __VLS_9 = {}.VCard;
/** @type {[typeof __VLS_components.VCard, typeof __VLS_components.vCard, typeof __VLS_components.VCard, typeof __VLS_components.vCard, ]} */ ;
// @ts-ignore
VCard;
// @ts-ignore
var __VLS_10 = __VLS_asFunctionalComponent(__VLS_9, new __VLS_9({
    width: "512",
    prependIcon: "mdi-arrow-collapse-vertical",
    title: (__VLS_ctx.title),
}));
var __VLS_11 = __VLS_10.apply(void 0, __spreadArray([{
        width: "512",
        prependIcon: "mdi-arrow-collapse-vertical",
        title: (__VLS_ctx.title),
    }], __VLS_functionalComponentArgsRest(__VLS_10), false));
var __VLS_13 = __VLS_12.slots.default;
// @ts-ignore
[title,];
{
    var __VLS_14 = __VLS_12.slots.actions;
    var __VLS_15 = {}.VBtn;
    /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
    // @ts-ignore
    VBtn;
    // @ts-ignore
    var __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15(__assign(__assign({ 'onClick': {} }, { class: "ms-auto" }), { text: (__VLS_ctx.t('cancel')) })));
    var __VLS_17 = __VLS_16.apply(void 0, __spreadArray([__assign(__assign({ 'onClick': {} }, { class: "ms-auto" }), { text: (__VLS_ctx.t('cancel')) })], __VLS_functionalComponentArgsRest(__VLS_16), false));
    var __VLS_19 = void 0;
    var __VLS_20 = void 0;
    var __VLS_21 = ({ click: {} },
        { onClick: function () {
                var _a = [];
                for (var _i = 0; _i < arguments.length; _i++) {
                    _a[_i] = arguments[_i];
                }
                var $event = _a[0];
                __VLS_ctx.emit('update:inProgress', false);
                // @ts-ignore
                [emit, t,];
            } });
    var __VLS_18;
}
var __VLS_23 = {}.VCardText;
/** @type {[typeof __VLS_components.VCardText, typeof __VLS_components.vCardText, typeof __VLS_components.VCardText, typeof __VLS_components.vCardText, ]} */ ;
// @ts-ignore
VCardText;
// @ts-ignore
var __VLS_24 = __VLS_asFunctionalComponent(__VLS_23, new __VLS_23({}));
var __VLS_25 = __VLS_24.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_24), false));
var __VLS_27 = __VLS_26.slots.default;
(__VLS_ctx.currentFile);
// @ts-ignore
[currentFile,];
var __VLS_28 = {}.VProgressLinear;
/** @type {[typeof __VLS_components.VProgressLinear, typeof __VLS_components.vProgressLinear, typeof __VLS_components.VProgressLinear, typeof __VLS_components.vProgressLinear, ]} */ ;
// @ts-ignore
VProgressLinear;
// @ts-ignore
var __VLS_29 = __VLS_asFunctionalComponent(__VLS_28, new __VLS_28({
    indeterminate: (__VLS_ctx.progress === 0),
    modelValue: (__VLS_ctx.progress),
    color: "primary",
    height: "25",
}));
var __VLS_30 = __VLS_29.apply(void 0, __spreadArray([{
        indeterminate: (__VLS_ctx.progress === 0),
        modelValue: (__VLS_ctx.progress),
        color: "primary",
        height: "25",
    }], __VLS_functionalComponentArgsRest(__VLS_29), false));
var __VLS_32 = __VLS_31.slots.default;
// @ts-ignore
[progress, progress,];
{
    var __VLS_33 = __VLS_31.slots.default;
    var value = __VLS_getSlotParameters(__VLS_33)[0].value;
    if (__VLS_ctx.progress) {
        // @ts-ignore
        [progress,];
        __VLS_asFunctionalElement(__VLS_elements.strong, __VLS_elements.strong)({});
        (Math.ceil(value));
    }
}
var __VLS_31;
var __VLS_26;
var __VLS_12;
var __VLS_3;
/** @type {__VLS_StyleScopedClasses['ms-auto']} */ ;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({
    __typeEmits: {},
    props: {
        dialog: { type: Boolean, default: false },
        title: { type: String, required: true },
        currentFile: { type: String, default: '' },
        progress: {
            type: Number,
            required: false,
            default: 0
        },
        inProgress: { type: Boolean, default: false }
    },
});
exports.default = {};
