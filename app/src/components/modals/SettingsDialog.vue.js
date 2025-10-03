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
var vue_1 = require("vue");
var vue_i18n_1 = require("vue-i18n");
var AvifOptions_vue_1 = require("./SettingTabItems/AvifOptions.vue");
var CommonOptions_vue_1 = require("./SettingTabItems/CommonOptions.vue");
var JxlOptions_vue_1 = require("./SettingTabItems/JxlOptions.vue");
var WebpOptions_vue_1 = require("./SettingTabItems/WebpOptions.vue");
var t = (0, vue_i18n_1.useI18n)().t;
var tab = (0, vue_1.ref)('common');
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VDialog;
/** @type {[typeof __VLS_components.VDialog, typeof __VLS_components.vDialog, typeof __VLS_components.VDialog, typeof __VLS_components.vDialog, ]} */ ;
// @ts-ignore
VDialog;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({
    fullscreen: true,
    persistent: true,
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        fullscreen: true,
        persistent: true,
    }], __VLS_functionalComponentArgsRest(__VLS_1), false));
var __VLS_4 = {};
var __VLS_5 = __VLS_3.slots.default;
{
    var __VLS_6 = __VLS_3.slots.activator;
    var dialogProps = __VLS_getSlotParameters(__VLS_6)[0].props;
    var __VLS_7 = {}.VTooltip;
    /** @type {[typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, ]} */ ;
    // @ts-ignore
    VTooltip;
    // @ts-ignore
    var __VLS_8 = __VLS_asFunctionalComponent(__VLS_7, new __VLS_7({
        text: (__VLS_ctx.t('settings')),
        location: "bottom",
    }));
    var __VLS_9 = __VLS_8.apply(void 0, __spreadArray([{
            text: (__VLS_ctx.t('settings')),
            location: "bottom",
        }], __VLS_functionalComponentArgsRest(__VLS_8), false));
    var __VLS_11 = __VLS_10.slots.default;
    // @ts-ignore
    [t,];
    {
        var __VLS_12 = __VLS_10.slots.activator;
        var tooltipProps = __VLS_getSlotParameters(__VLS_12)[0].props;
        var __VLS_13 = {}.VBtn;
        /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
        // @ts-ignore
        VBtn;
        // @ts-ignore
        var __VLS_14 = __VLS_asFunctionalComponent(__VLS_13, new __VLS_13(__assign(__assign({}, (__assign(__assign({}, dialogProps), tooltipProps))), { icon: "mdi-cog-outline", variant: "plain" })));
        var __VLS_15 = __VLS_14.apply(void 0, __spreadArray([__assign(__assign({}, (__assign(__assign({}, dialogProps), tooltipProps))), { icon: "mdi-cog-outline", variant: "plain" })], __VLS_functionalComponentArgsRest(__VLS_14), false));
    }
    var __VLS_10;
}
{
    var __VLS_18 = __VLS_3.slots.default;
    var isActive_1 = __VLS_getSlotParameters(__VLS_18)[0].isActive;
    var __VLS_19 = {}.VCard;
    /** @type {[typeof __VLS_components.VCard, typeof __VLS_components.vCard, typeof __VLS_components.VCard, typeof __VLS_components.vCard, ]} */ ;
    // @ts-ignore
    VCard;
    // @ts-ignore
    var __VLS_20 = __VLS_asFunctionalComponent(__VLS_19, new __VLS_19({}));
    var __VLS_21 = __VLS_20.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_20), false));
    var __VLS_23 = __VLS_22.slots.default;
    var __VLS_24 = {}.VToolbar;
    /** @type {[typeof __VLS_components.VToolbar, typeof __VLS_components.vToolbar, typeof __VLS_components.VToolbar, typeof __VLS_components.vToolbar, ]} */ ;
    // @ts-ignore
    VToolbar;
    // @ts-ignore
    var __VLS_25 = __VLS_asFunctionalComponent(__VLS_24, new __VLS_24({}));
    var __VLS_26 = __VLS_25.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_25), false));
    var __VLS_28 = __VLS_27.slots.default;
    var __VLS_29 = {}.VToolbarTitle;
    /** @type {[typeof __VLS_components.VToolbarTitle, typeof __VLS_components.vToolbarTitle, typeof __VLS_components.VToolbarTitle, typeof __VLS_components.vToolbarTitle, ]} */ ;
    // @ts-ignore
    VToolbarTitle;
    // @ts-ignore
    var __VLS_30 = __VLS_asFunctionalComponent(__VLS_29, new __VLS_29({}));
    var __VLS_31 = __VLS_30.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_30), false));
    var __VLS_33 = __VLS_32.slots.default;
    (__VLS_ctx.t('settings'));
    // @ts-ignore
    [t,];
    var __VLS_32;
    var __VLS_34 = {}.VSpacer;
    /** @type {[typeof __VLS_components.VSpacer, typeof __VLS_components.vSpacer, ]} */ ;
    // @ts-ignore
    VSpacer;
    // @ts-ignore
    var __VLS_35 = __VLS_asFunctionalComponent(__VLS_34, new __VLS_34({}));
    var __VLS_36 = __VLS_35.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_35), false));
    var __VLS_39 = {}.VBtn;
    /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
    // @ts-ignore
    VBtn;
    // @ts-ignore
    var __VLS_40 = __VLS_asFunctionalComponent(__VLS_39, new __VLS_39(__assign({ 'onClick': {} }, { icon: "mdi-close" })));
    var __VLS_41 = __VLS_40.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { icon: "mdi-close" })], __VLS_functionalComponentArgsRest(__VLS_40), false));
    var __VLS_43 = void 0;
    var __VLS_44 = void 0;
    var __VLS_45 = ({ click: {} },
        { onClick: function () {
                var _a = [];
                for (var _i = 0; _i < arguments.length; _i++) {
                    _a[_i] = arguments[_i];
                }
                var $event = _a[0];
                isActive_1.value = false;
            } });
    var __VLS_42;
    var __VLS_27;
    var __VLS_47 = {}.VCardText;
    /** @type {[typeof __VLS_components.VCardText, typeof __VLS_components.vCardText, typeof __VLS_components.VCardText, typeof __VLS_components.vCardText, ]} */ ;
    // @ts-ignore
    VCardText;
    // @ts-ignore
    var __VLS_48 = __VLS_asFunctionalComponent(__VLS_47, new __VLS_47({}));
    var __VLS_49 = __VLS_48.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_48), false));
    var __VLS_51 = __VLS_50.slots.default;
    var __VLS_52 = {}.VTabs;
    /** @type {[typeof __VLS_components.VTabs, typeof __VLS_components.vTabs, typeof __VLS_components.VTabs, typeof __VLS_components.vTabs, ]} */ ;
    // @ts-ignore
    VTabs;
    // @ts-ignore
    var __VLS_53 = __VLS_asFunctionalComponent(__VLS_52, new __VLS_52({
        modelValue: (__VLS_ctx.tab),
        color: "primary",
    }));
    var __VLS_54 = __VLS_53.apply(void 0, __spreadArray([{
            modelValue: (__VLS_ctx.tab),
            color: "primary",
        }], __VLS_functionalComponentArgsRest(__VLS_53), false));
    var __VLS_56 = __VLS_55.slots.default;
    // @ts-ignore
    [tab,];
    var __VLS_57 = {}.VTab;
    /** @type {[typeof __VLS_components.VTab, typeof __VLS_components.vTab, typeof __VLS_components.VTab, typeof __VLS_components.vTab, ]} */ ;
    // @ts-ignore
    VTab;
    // @ts-ignore
    var __VLS_58 = __VLS_asFunctionalComponent(__VLS_57, new __VLS_57({
        value: "common",
    }));
    var __VLS_59 = __VLS_58.apply(void 0, __spreadArray([{
            value: "common",
        }], __VLS_functionalComponentArgsRest(__VLS_58), false));
    var __VLS_61 = __VLS_60.slots.default;
    (__VLS_ctx.t('common_options'));
    // @ts-ignore
    [t,];
    var __VLS_60;
    var __VLS_62 = {}.VTab;
    /** @type {[typeof __VLS_components.VTab, typeof __VLS_components.vTab, typeof __VLS_components.VTab, typeof __VLS_components.vTab, ]} */ ;
    // @ts-ignore
    VTab;
    // @ts-ignore
    var __VLS_63 = __VLS_asFunctionalComponent(__VLS_62, new __VLS_62({
        value: "webp",
    }));
    var __VLS_64 = __VLS_63.apply(void 0, __spreadArray([{
            value: "webp",
        }], __VLS_functionalComponentArgsRest(__VLS_63), false));
    var __VLS_66 = __VLS_65.slots.default;
    (__VLS_ctx.t('webp_options'));
    // @ts-ignore
    [t,];
    var __VLS_65;
    var __VLS_67 = {}.VTab;
    /** @type {[typeof __VLS_components.VTab, typeof __VLS_components.vTab, typeof __VLS_components.VTab, typeof __VLS_components.vTab, ]} */ ;
    // @ts-ignore
    VTab;
    // @ts-ignore
    var __VLS_68 = __VLS_asFunctionalComponent(__VLS_67, new __VLS_67({
        value: "avif",
    }));
    var __VLS_69 = __VLS_68.apply(void 0, __spreadArray([{
            value: "avif",
        }], __VLS_functionalComponentArgsRest(__VLS_68), false));
    var __VLS_71 = __VLS_70.slots.default;
    (__VLS_ctx.t('avif_options'));
    // @ts-ignore
    [t,];
    var __VLS_70;
    var __VLS_72 = {}.VTab;
    /** @type {[typeof __VLS_components.VTab, typeof __VLS_components.vTab, typeof __VLS_components.VTab, typeof __VLS_components.vTab, ]} */ ;
    // @ts-ignore
    VTab;
    // @ts-ignore
    var __VLS_73 = __VLS_asFunctionalComponent(__VLS_72, new __VLS_72({
        value: "jxl",
    }));
    var __VLS_74 = __VLS_73.apply(void 0, __spreadArray([{
            value: "jxl",
        }], __VLS_functionalComponentArgsRest(__VLS_73), false));
    var __VLS_76 = __VLS_75.slots.default;
    (__VLS_ctx.t('jxl_options'));
    // @ts-ignore
    [t,];
    var __VLS_75;
    var __VLS_55;
    var __VLS_77 = {}.VDivider;
    /** @type {[typeof __VLS_components.VDivider, typeof __VLS_components.vDivider, ]} */ ;
    // @ts-ignore
    VDivider;
    // @ts-ignore
    var __VLS_78 = __VLS_asFunctionalComponent(__VLS_77, new __VLS_77({}));
    var __VLS_79 = __VLS_78.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_78), false));
    var __VLS_82 = {}.VWindow;
    /** @type {[typeof __VLS_components.VWindow, typeof __VLS_components.vWindow, typeof __VLS_components.VWindow, typeof __VLS_components.vWindow, ]} */ ;
    // @ts-ignore
    VWindow;
    // @ts-ignore
    var __VLS_83 = __VLS_asFunctionalComponent(__VLS_82, new __VLS_82(__assign({ modelValue: (__VLS_ctx.tab) }, { class: "mt-4" })));
    var __VLS_84 = __VLS_83.apply(void 0, __spreadArray([__assign({ modelValue: (__VLS_ctx.tab) }, { class: "mt-4" })], __VLS_functionalComponentArgsRest(__VLS_83), false));
    var __VLS_86 = __VLS_85.slots.default;
    // @ts-ignore
    [tab,];
    var __VLS_87 = {}.VWindowItem;
    /** @type {[typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, ]} */ ;
    // @ts-ignore
    VWindowItem;
    // @ts-ignore
    var __VLS_88 = __VLS_asFunctionalComponent(__VLS_87, new __VLS_87({
        value: "common",
    }));
    var __VLS_89 = __VLS_88.apply(void 0, __spreadArray([{
            value: "common",
        }], __VLS_functionalComponentArgsRest(__VLS_88), false));
    var __VLS_91 = __VLS_90.slots.default;
    /** @type {[typeof CommonOptions, ]} */ ;
    // @ts-ignore
    var __VLS_92 = __VLS_asFunctionalComponent(CommonOptions_vue_1.default, new CommonOptions_vue_1.default({}));
    var __VLS_93 = __VLS_92.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_92), false));
    var __VLS_90;
    var __VLS_96 = {}.VWindowItem;
    /** @type {[typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, ]} */ ;
    // @ts-ignore
    VWindowItem;
    // @ts-ignore
    var __VLS_97 = __VLS_asFunctionalComponent(__VLS_96, new __VLS_96({
        value: "webp",
    }));
    var __VLS_98 = __VLS_97.apply(void 0, __spreadArray([{
            value: "webp",
        }], __VLS_functionalComponentArgsRest(__VLS_97), false));
    var __VLS_100 = __VLS_99.slots.default;
    /** @type {[typeof WebpOptions, ]} */ ;
    // @ts-ignore
    var __VLS_101 = __VLS_asFunctionalComponent(WebpOptions_vue_1.default, new WebpOptions_vue_1.default({}));
    var __VLS_102 = __VLS_101.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_101), false));
    var __VLS_99;
    var __VLS_105 = {}.VWindowItem;
    /** @type {[typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, ]} */ ;
    // @ts-ignore
    VWindowItem;
    // @ts-ignore
    var __VLS_106 = __VLS_asFunctionalComponent(__VLS_105, new __VLS_105({
        value: "avif",
    }));
    var __VLS_107 = __VLS_106.apply(void 0, __spreadArray([{
            value: "avif",
        }], __VLS_functionalComponentArgsRest(__VLS_106), false));
    var __VLS_109 = __VLS_108.slots.default;
    /** @type {[typeof AvifOptions, ]} */ ;
    // @ts-ignore
    var __VLS_110 = __VLS_asFunctionalComponent(AvifOptions_vue_1.default, new AvifOptions_vue_1.default({}));
    var __VLS_111 = __VLS_110.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_110), false));
    var __VLS_108;
    var __VLS_114 = {}.VWindowItem;
    /** @type {[typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, typeof __VLS_components.VWindowItem, typeof __VLS_components.vWindowItem, ]} */ ;
    // @ts-ignore
    VWindowItem;
    // @ts-ignore
    var __VLS_115 = __VLS_asFunctionalComponent(__VLS_114, new __VLS_114({
        value: "jxl",
    }));
    var __VLS_116 = __VLS_115.apply(void 0, __spreadArray([{
            value: "jxl",
        }], __VLS_functionalComponentArgsRest(__VLS_115), false));
    var __VLS_118 = __VLS_117.slots.default;
    /** @type {[typeof JxlOptions, ]} */ ;
    // @ts-ignore
    var __VLS_119 = __VLS_asFunctionalComponent(JxlOptions_vue_1.default, new JxlOptions_vue_1.default({}));
    var __VLS_120 = __VLS_119.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_119), false));
    var __VLS_117;
    var __VLS_85;
    var __VLS_50;
    var __VLS_22;
}
var __VLS_3;
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
