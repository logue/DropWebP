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
var LocaleSelector_vue_1 = require("./LocaleSelector.vue");
var AboutDialog_vue_1 = require("./modals/AboutDialog.vue");
var SettingsDialog_vue_1 = require("./modals/SettingsDialog.vue");
var t = (0, vue_i18n_1.useI18n)().t;
/** Config Store */
var configStore = (0, store_1.useConfigStore)();
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
/** @type {[typeof AboutDialog, ]} */ ;
// @ts-ignore
var __VLS_0 = __VLS_asFunctionalComponent(AboutDialog_vue_1.default, new AboutDialog_vue_1.default({}));
var __VLS_1 = __VLS_0.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_0), false));
var __VLS_4 = {}.VMenu;
/** @type {[typeof __VLS_components.VMenu, typeof __VLS_components.vMenu, typeof __VLS_components.VMenu, typeof __VLS_components.vMenu, ]} */ ;
// @ts-ignore
VMenu;
// @ts-ignore
var __VLS_5 = __VLS_asFunctionalComponent(__VLS_4, new __VLS_4({
    location: "bottom",
}));
var __VLS_6 = __VLS_5.apply(void 0, __spreadArray([{
        location: "bottom",
    }], __VLS_functionalComponentArgsRest(__VLS_5), false));
var __VLS_8 = __VLS_7.slots.default;
{
    var __VLS_9 = __VLS_7.slots.activator;
    var props = __VLS_getSlotParameters(__VLS_9)[0].props;
    var __VLS_10 = {}.VBtn;
    /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
    // @ts-ignore
    VBtn;
    // @ts-ignore
    var __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10(__assign(__assign({}, (props)), { icon: true, variant: "plain" })));
    var __VLS_12 = __VLS_11.apply(void 0, __spreadArray([__assign(__assign({}, (props)), { icon: true, variant: "plain" })], __VLS_functionalComponentArgsRest(__VLS_11), false));
    var __VLS_14 = __VLS_13.slots.default;
    var __VLS_15 = {}.VIcon;
    /** @type {[typeof __VLS_components.VIcon, typeof __VLS_components.vIcon, typeof __VLS_components.VIcon, typeof __VLS_components.vIcon, ]} */ ;
    // @ts-ignore
    VIcon;
    // @ts-ignore
    var __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15({}));
    var __VLS_17 = __VLS_16.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_16), false));
    var __VLS_19 = __VLS_18.slots.default;
    var __VLS_18;
    var __VLS_20 = {}.VTooltip;
    /** @type {[typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, ]} */ ;
    // @ts-ignore
    VTooltip;
    // @ts-ignore
    var __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, new __VLS_20({
        text: (__VLS_ctx.t('locale_btn')),
        activator: "parent",
        location: "bottom",
    }));
    var __VLS_22 = __VLS_21.apply(void 0, __spreadArray([{
            text: (__VLS_ctx.t('locale_btn')),
            activator: "parent",
            location: "bottom",
        }], __VLS_functionalComponentArgsRest(__VLS_21), false));
    // @ts-ignore
    [t,];
    var __VLS_13;
}
var __VLS_25 = {}.VList;
/** @type {[typeof __VLS_components.VList, typeof __VLS_components.vList, typeof __VLS_components.VList, typeof __VLS_components.vList, ]} */ ;
// @ts-ignore
VList;
// @ts-ignore
var __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25({
    density: "compact",
}));
var __VLS_27 = __VLS_26.apply(void 0, __spreadArray([{
        density: "compact",
    }], __VLS_functionalComponentArgsRest(__VLS_26), false));
var __VLS_29 = __VLS_28.slots.default;
/** @type {[typeof LocaleSelector, ]} */ ;
// @ts-ignore
var __VLS_30 = __VLS_asFunctionalComponent(LocaleSelector_vue_1.default, new LocaleSelector_vue_1.default({
    tooltip: (__VLS_ctx.t('locale_btn')),
}));
var __VLS_31 = __VLS_30.apply(void 0, __spreadArray([{
        tooltip: (__VLS_ctx.t('locale_btn')),
    }], __VLS_functionalComponentArgsRest(__VLS_30), false));
// @ts-ignore
[t,];
var __VLS_28;
var __VLS_7;
var __VLS_34 = {}.VTooltip;
/** @type {[typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, typeof __VLS_components.VTooltip, typeof __VLS_components.vTooltip, ]} */ ;
// @ts-ignore
VTooltip;
// @ts-ignore
var __VLS_35 = __VLS_asFunctionalComponent(__VLS_34, new __VLS_34({
    text: (__VLS_ctx.t('toggle_dark_mode_btn')),
    location: "bottom",
}));
var __VLS_36 = __VLS_35.apply(void 0, __spreadArray([{
        text: (__VLS_ctx.t('toggle_dark_mode_btn')),
        location: "bottom",
    }], __VLS_functionalComponentArgsRest(__VLS_35), false));
var __VLS_38 = __VLS_37.slots.default;
// @ts-ignore
[t,];
{
    var __VLS_39 = __VLS_37.slots.activator;
    var props = __VLS_getSlotParameters(__VLS_39)[0].props;
    var __VLS_40 = {}.VBtn;
    /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
    // @ts-ignore
    VBtn;
    // @ts-ignore
    var __VLS_41 = __VLS_asFunctionalComponent(__VLS_40, new __VLS_40(__assign(__assign({ 'onClick': {} }, (props)), { icon: "mdi-theme-light-dark", variant: "plain" })));
    var __VLS_42 = __VLS_41.apply(void 0, __spreadArray([__assign(__assign({ 'onClick': {} }, (props)), { icon: "mdi-theme-light-dark", variant: "plain" })], __VLS_functionalComponentArgsRest(__VLS_41), false));
    var __VLS_44 = void 0;
    var __VLS_45 = void 0;
    var __VLS_46 = ({ click: {} },
        { onClick: (__VLS_ctx.configStore.toggleTheme) });
    // @ts-ignore
    [configStore,];
    var __VLS_43;
}
var __VLS_37;
/** @type {[typeof SettingsDialog, ]} */ ;
// @ts-ignore
var __VLS_48 = __VLS_asFunctionalComponent(SettingsDialog_vue_1.default, new SettingsDialog_vue_1.default({
    tooltip: (__VLS_ctx.t('settings_btn')),
}));
var __VLS_49 = __VLS_48.apply(void 0, __spreadArray([{
        tooltip: (__VLS_ctx.t('settings_btn')),
    }], __VLS_functionalComponentArgsRest(__VLS_48), false));
// @ts-ignore
[t,];
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
