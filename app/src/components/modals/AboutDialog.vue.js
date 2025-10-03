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
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
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
var plugin_opener_1 = require("@tauri-apps/plugin-opener");
var Meta_1 = require("@/Meta");
var t = (0, vue_i18n_1.useI18n)().t;
var openGitHub = function () { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0: return [4 /*yield*/, (0, plugin_opener_1.openUrl)('https://github.com/logue/DropWebP')];
            case 1:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); };
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
    width: "auto",
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        width: "auto",
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
        text: (__VLS_ctx.t('about_title')),
        location: "bottom",
    }));
    var __VLS_9 = __VLS_8.apply(void 0, __spreadArray([{
            text: (__VLS_ctx.t('about_title')),
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
        var __VLS_14 = __VLS_asFunctionalComponent(__VLS_13, new __VLS_13(__assign(__assign({}, (__assign(__assign({}, dialogProps), tooltipProps))), { icon: "mdi-information-outline", variant: "plain" })));
        var __VLS_15 = __VLS_14.apply(void 0, __spreadArray([__assign(__assign({}, (__assign(__assign({}, dialogProps), tooltipProps))), { icon: "mdi-information-outline", variant: "plain" })], __VLS_functionalComponentArgsRest(__VLS_14), false));
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
    var __VLS_20 = __VLS_asFunctionalComponent(__VLS_19, new __VLS_19({
        width: "360",
        title: (__VLS_ctx.t('about_title')),
    }));
    var __VLS_21 = __VLS_20.apply(void 0, __spreadArray([{
            width: "360",
            title: (__VLS_ctx.t('about_title')),
        }], __VLS_functionalComponentArgsRest(__VLS_20), false));
    var __VLS_23 = __VLS_22.slots.default;
    // @ts-ignore
    [t,];
    {
        var __VLS_24 = __VLS_22.slots.actions;
        var __VLS_25 = {}.VBtn;
        /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
        // @ts-ignore
        VBtn;
        // @ts-ignore
        var __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25(__assign(__assign(__assign({ 'onClick': {} }, { color: "primary" }), { class: "ms-auto" }), { text: "OK" })));
        var __VLS_27 = __VLS_26.apply(void 0, __spreadArray([__assign(__assign(__assign({ 'onClick': {} }, { color: "primary" }), { class: "ms-auto" }), { text: "OK" })], __VLS_functionalComponentArgsRest(__VLS_26), false));
        var __VLS_29 = void 0;
        var __VLS_30 = void 0;
        var __VLS_31 = ({ click: {} },
            { onClick: function () {
                    var _a = [];
                    for (var _i = 0; _i < arguments.length; _i++) {
                        _a[_i] = arguments[_i];
                    }
                    var $event = _a[0];
                    isActive_1.value = false;
                } });
        var __VLS_28;
    }
    var __VLS_33 = {}.VCardText;
    /** @type {[typeof __VLS_components.VCardText, typeof __VLS_components.vCardText, typeof __VLS_components.VCardText, typeof __VLS_components.vCardText, ]} */ ;
    // @ts-ignore
    VCardText;
    // @ts-ignore
    var __VLS_34 = __VLS_asFunctionalComponent(__VLS_33, new __VLS_33(__assign({ class: "text-center" })));
    var __VLS_35 = __VLS_34.apply(void 0, __spreadArray([__assign({ class: "text-center" })], __VLS_functionalComponentArgsRest(__VLS_34), false));
    var __VLS_37 = __VLS_36.slots.default;
    __VLS_asFunctionalElement(__VLS_elements.h2, __VLS_elements.h2)({});
    __VLS_asFunctionalElement(__VLS_elements.p, __VLS_elements.p)({});
    (__VLS_ctx.Meta.version);
    // @ts-ignore
    [Meta_1.default,];
    __VLS_asFunctionalElement(__VLS_elements.br)({});
    __VLS_asFunctionalElement(__VLS_elements.small, __VLS_elements.small)({});
    (__VLS_ctx.Meta.date);
    // @ts-ignore
    [Meta_1.default,];
    __VLS_asFunctionalElement(__VLS_elements.p, __VLS_elements.p)({});
    __VLS_asFunctionalElement(__VLS_elements.a, __VLS_elements.a)(__assign({ onClick: (__VLS_ctx.openGitHub) }, { href: "https://github.com/logue/DropWebP", target: "_blank" }));
    // @ts-ignore
    [openGitHub,];
    var __VLS_36;
    var __VLS_22;
}
var __VLS_3;
/** @type {__VLS_StyleScopedClasses['ms-auto']} */ ;
/** @type {__VLS_StyleScopedClasses['text-center']} */ ;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
