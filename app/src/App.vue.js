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
var _a;
Object.defineProperty(exports, "__esModule", { value: true });
var store_1 = require("@/store");
var vue_1 = require("vue");
// Components
var AppBarMenuComponent_vue_1 = require("@/components/AppBarMenuComponent.vue");
var MainContent_vue_1 = require("@/components/MainContent.vue");
/** Global Store */
var globalStore = (0, store_1.useGlobalStore)();
/** Config Store */
var configStore = (0, store_1.useConfigStore)();
/** Title */
var title = (_a = import.meta.env.VITE_APP_TITLE) !== null && _a !== void 0 ? _a : 'Drop Compress Image';
/** loading overlay visibility */
var loading = (0, vue_1.computed)({
    get: function () { return globalStore.loading; },
    set: function (v) { return globalStore.setLoading(v); }
});
/** Appbar progressbar value */
var progress = (0, vue_1.computed)({
    get: function () { return globalStore.progress; },
    set: function (v) { return globalStore.setProgress(v); }
});
/** Snackbar visibility */
var snackbarVisibility = (0, vue_1.ref)(false);
/** Snackbar text */
var snackbarText = (0, vue_1.computed)(function () { return globalStore.message; });
/** Toggle Dark mode */
var isDark = (0, vue_1.computed)(function () { return (configStore.theme ? 'dark' : 'light'); });
// When snackbar text has been set, show snackbar.
(0, vue_1.watch)(function () { return globalStore.message; }, function (message) { return (snackbarVisibility.value = message !== ''); });
/** Clear store when snackbar hide */
var onSnackbarChanged = function () { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                globalStore.setMessage();
                return [4 /*yield*/, (0, vue_1.nextTick)()];
            case 1:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); };
(0, vue_1.onMounted)(function () { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        document.title = title;
        loading.value = false;
        return [2 /*return*/];
    });
}); });
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_ctx = __assign(__assign({}, {}), {});
var __VLS_elements;
var __VLS_components;
var __VLS_directives;
var __VLS_0 = {}.VApp;
/** @type {[typeof __VLS_components.VApp, typeof __VLS_components.vApp, typeof __VLS_components.VApp, typeof __VLS_components.vApp, ]} */ ;
// @ts-ignore
VApp;
// @ts-ignore
var __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({
    theme: (__VLS_ctx.isDark),
    dataTauriDragRegion: "true",
}));
var __VLS_2 = __VLS_1.apply(void 0, __spreadArray([{
        theme: (__VLS_ctx.isDark),
        dataTauriDragRegion: "true",
    }], __VLS_functionalComponentArgsRest(__VLS_1), false));
var __VLS_4 = {};
var __VLS_5 = __VLS_3.slots.default;
// @ts-ignore
[isDark,];
var __VLS_6 = {}.VAppBar;
/** @type {[typeof __VLS_components.VAppBar, typeof __VLS_components.vAppBar, typeof __VLS_components.VAppBar, typeof __VLS_components.vAppBar, ]} */ ;
// @ts-ignore
VAppBar;
// @ts-ignore
var __VLS_7 = __VLS_asFunctionalComponent(__VLS_6, new __VLS_6({
    color: "primary",
    density: "compact",
}));
var __VLS_8 = __VLS_7.apply(void 0, __spreadArray([{
        color: "primary",
        density: "compact",
    }], __VLS_functionalComponentArgsRest(__VLS_7), false));
var __VLS_10 = __VLS_9.slots.default;
var __VLS_11 = {}.VAppBarTitle;
/** @type {[typeof __VLS_components.VAppBarTitle, typeof __VLS_components.vAppBarTitle, typeof __VLS_components.VAppBarTitle, typeof __VLS_components.vAppBarTitle, ]} */ ;
// @ts-ignore
VAppBarTitle;
// @ts-ignore
var __VLS_12 = __VLS_asFunctionalComponent(__VLS_11, new __VLS_11({
    tag: "h1",
}));
var __VLS_13 = __VLS_12.apply(void 0, __spreadArray([{
        tag: "h1",
    }], __VLS_functionalComponentArgsRest(__VLS_12), false));
var __VLS_15 = __VLS_14.slots.default;
(__VLS_ctx.title);
// @ts-ignore
[title,];
var __VLS_14;
var __VLS_16 = {}.VSpacer;
/** @type {[typeof __VLS_components.VSpacer, typeof __VLS_components.vSpacer, ]} */ ;
// @ts-ignore
VSpacer;
// @ts-ignore
var __VLS_17 = __VLS_asFunctionalComponent(__VLS_16, new __VLS_16({}));
var __VLS_18 = __VLS_17.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_17), false));
/** @type {[typeof AppBarMenuComponent, ]} */ ;
// @ts-ignore
var __VLS_21 = __VLS_asFunctionalComponent(AppBarMenuComponent_vue_1.default, new AppBarMenuComponent_vue_1.default({}));
var __VLS_22 = __VLS_21.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_21), false));
var __VLS_25 = {}.VProgressLinear;
/** @type {[typeof __VLS_components.VProgressLinear, typeof __VLS_components.vProgressLinear, ]} */ ;
// @ts-ignore
VProgressLinear;
// @ts-ignore
var __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25({
    active: (__VLS_ctx.loading),
    indeterminate: (__VLS_ctx.progress === null),
    modelValue: (__VLS_ctx.progress !== null ? __VLS_ctx.progress : 0),
    color: "blue-accent-3",
}));
var __VLS_27 = __VLS_26.apply(void 0, __spreadArray([{
        active: (__VLS_ctx.loading),
        indeterminate: (__VLS_ctx.progress === null),
        modelValue: (__VLS_ctx.progress !== null ? __VLS_ctx.progress : 0),
        color: "blue-accent-3",
    }], __VLS_functionalComponentArgsRest(__VLS_26), false));
__VLS_asFunctionalDirective(__VLS_directives.vShow)(null, __assign(__assign({}, __VLS_directiveBindingRestFields), { value: (__VLS_ctx.loading) }), null, null);
// @ts-ignore
[loading, loading, progress, progress, progress,];
var __VLS_9;
var __VLS_30 = {}.VMain;
/** @type {[typeof __VLS_components.VMain, typeof __VLS_components.vMain, typeof __VLS_components.VMain, typeof __VLS_components.vMain, ]} */ ;
// @ts-ignore
VMain;
// @ts-ignore
var __VLS_31 = __VLS_asFunctionalComponent(__VLS_30, new __VLS_30({}));
var __VLS_32 = __VLS_31.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_31), false));
var __VLS_34 = __VLS_33.slots.default;
/** @type {[typeof MainContent, ]} */ ;
// @ts-ignore
var __VLS_35 = __VLS_asFunctionalComponent(MainContent_vue_1.default, new MainContent_vue_1.default({}));
var __VLS_36 = __VLS_35.apply(void 0, __spreadArray([{}], __VLS_functionalComponentArgsRest(__VLS_35), false));
var __VLS_33;
var __VLS_39 = {}.VOverlay;
/** @type {[typeof __VLS_components.VOverlay, typeof __VLS_components.vOverlay, typeof __VLS_components.VOverlay, typeof __VLS_components.vOverlay, ]} */ ;
// @ts-ignore
VOverlay;
// @ts-ignore
var __VLS_40 = __VLS_asFunctionalComponent(__VLS_39, new __VLS_39(__assign(__assign({ modelValue: (__VLS_ctx.loading), app: true }, { class: "justify-center align-center" }), { persistent: true })));
var __VLS_41 = __VLS_40.apply(void 0, __spreadArray([__assign(__assign({ modelValue: (__VLS_ctx.loading), app: true }, { class: "justify-center align-center" }), { persistent: true })], __VLS_functionalComponentArgsRest(__VLS_40), false));
var __VLS_43 = __VLS_42.slots.default;
// @ts-ignore
[loading,];
var __VLS_44 = {}.VProgressCircular;
/** @type {[typeof __VLS_components.VProgressCircular, typeof __VLS_components.vProgressCircular, ]} */ ;
// @ts-ignore
VProgressCircular;
// @ts-ignore
var __VLS_45 = __VLS_asFunctionalComponent(__VLS_44, new __VLS_44({
    indeterminate: true,
    size: "64",
}));
var __VLS_46 = __VLS_45.apply(void 0, __spreadArray([{
        indeterminate: true,
        size: "64",
    }], __VLS_functionalComponentArgsRest(__VLS_45), false));
var __VLS_42;
var __VLS_49 = {}.VSnackbar;
/** @type {[typeof __VLS_components.VSnackbar, typeof __VLS_components.vSnackbar, typeof __VLS_components.VSnackbar, typeof __VLS_components.vSnackbar, ]} */ ;
// @ts-ignore
VSnackbar;
// @ts-ignore
var __VLS_50 = __VLS_asFunctionalComponent(__VLS_49, new __VLS_49(__assign({ 'onUpdate:modelValue': {} }, { modelValue: (__VLS_ctx.snackbarVisibility), color: (__VLS_ctx.globalStore.snackbarColor) })));
var __VLS_51 = __VLS_50.apply(void 0, __spreadArray([__assign({ 'onUpdate:modelValue': {} }, { modelValue: (__VLS_ctx.snackbarVisibility), color: (__VLS_ctx.globalStore.snackbarColor) })], __VLS_functionalComponentArgsRest(__VLS_50), false));
var __VLS_53;
var __VLS_54;
var __VLS_55 = ({ 'update:modelValue': {} },
    { 'onUpdate:modelValue': (__VLS_ctx.onSnackbarChanged) });
var __VLS_56 = __VLS_52.slots.default;
// @ts-ignore
[snackbarVisibility, globalStore, onSnackbarChanged,];
(__VLS_ctx.snackbarText);
// @ts-ignore
[snackbarText,];
{
    var __VLS_57 = __VLS_52.slots.actions;
    var __VLS_58 = {}.VBtn;
    /** @type {[typeof __VLS_components.VBtn, typeof __VLS_components.vBtn, ]} */ ;
    // @ts-ignore
    VBtn;
    // @ts-ignore
    var __VLS_59 = __VLS_asFunctionalComponent(__VLS_58, new __VLS_58(__assign({ 'onClick': {} }, { icon: "mdi-close" })));
    var __VLS_60 = __VLS_59.apply(void 0, __spreadArray([__assign({ 'onClick': {} }, { icon: "mdi-close" })], __VLS_functionalComponentArgsRest(__VLS_59), false));
    var __VLS_62 = void 0;
    var __VLS_63 = void 0;
    var __VLS_64 = ({ click: {} },
        { onClick: (__VLS_ctx.onSnackbarChanged) });
    // @ts-ignore
    [onSnackbarChanged,];
    var __VLS_61;
}
var __VLS_52;
var __VLS_3;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['align-center']} */ ;
var __VLS_export = (await Promise.resolve().then(function () { return require('vue'); })).defineComponent({});
exports.default = {};
