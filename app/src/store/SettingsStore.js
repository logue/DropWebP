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
Object.defineProperty(exports, "__esModule", { value: true });
var pinia_1 = require("pinia");
var vue_1 = require("vue");
var path_1 = require("@tauri-apps/api/path");
var plugin_dialog_1 = require("@tauri-apps/plugin-dialog");
// デフォルト設定を定義
var defaultAvifOptions = {
    quality: 80.0,
    bitDepth: 'Auto',
    alphaQuality: 80.0,
    speed: 5,
    colorModel: 'YCbCr',
    threads: undefined,
    alphaColorMode: 'Premultiplied'
};
var defaultWebpOptions = {
    quality: 80,
    lossless: true,
    hint: 'Default',
    method: 6,
    autofilter: false
};
var defaultJxlOptions = {
    lossless: true,
    speed: 'Squirrel',
    quality: 1,
    useContainer: false,
    usesOriginalProfile: false,
    decodingSpeed: 0,
    initBufferSize: 512,
    colorEncoding: 'Srgb'
};
var defaultCommonOptions = {
    format: 'webp',
    overwrite: true,
    deleteOriginal: false,
    recursive: false,
    sameDirectory: true,
    ignoreJpeg: false,
    outputPath: await (0, path_1.documentDir)()
};
/** Global Store */
exports.default = (0, pinia_1.defineStore)('settings', function () {
    /** 全般オプション */
    var commonOptions = (0, vue_1.ref)(__assign({}, defaultCommonOptions));
    /** AVIFオプション */
    var avifOptions = (0, vue_1.ref)(__assign({}, defaultAvifOptions));
    /** WebPオプション */
    var webpOptions = (0, vue_1.ref)(__assign({}, defaultWebpOptions));
    /** JPEG XLオプション */
    var jxlOptions = (0, vue_1.ref)(__assign({}, defaultJxlOptions));
    /** 設定を初期化 */
    var reset = function () {
        commonOptions.value = __assign({}, defaultCommonOptions);
        avifOptions.value = __assign({}, defaultAvifOptions);
        webpOptions.value = __assign({}, defaultWebpOptions);
        jxlOptions.value = __assign({}, defaultJxlOptions);
    };
    var resetCommonOptions = function () { return (commonOptions.value = __assign({}, defaultCommonOptions)); };
    var resetAvifOptions = function () { return (avifOptions.value = __assign({}, defaultAvifOptions)); };
    var resetWebpOptions = function () { return (webpOptions.value = __assign({}, defaultWebpOptions)); };
    var resetJxlOptions = function () { return (jxlOptions.value = __assign({}, defaultJxlOptions)); };
    /** 出力先ディレクトリ選択ダイアログ */
    var browseOutputPath = function () { return __awaiter(void 0, void 0, void 0, function () {
        var path;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, (0, plugin_dialog_1.open)({
                        multiple: false,
                        directory: true
                    })];
                case 1:
                    path = _a.sent();
                    if (path === null || path === void 0 ? void 0 : path.length) {
                        commonOptions.value.outputPath = path;
                    }
                    return [2 /*return*/];
            }
        });
    }); };
    return {
        avifOptions: avifOptions,
        webpOptions: webpOptions,
        jxlOptions: jxlOptions,
        commonOptions: commonOptions,
        reset: reset,
        resetAvifOptions: resetAvifOptions,
        resetWebpOptions: resetWebpOptions,
        resetCommonOptions: resetCommonOptions,
        resetJxlOptions: resetJxlOptions,
        browseOutputPath: browseOutputPath
    };
}, {
    persist: {
        storage: window.localStorage
    }
});
