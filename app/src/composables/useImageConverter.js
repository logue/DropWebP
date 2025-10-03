"use strict";
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
exports.useImageConverter = useImageConverter;
var store_1 = require("@/store");
var vue_1 = require("vue");
var core_1 = require("@tauri-apps/api/core");
var path_1 = require("@tauri-apps/api/path");
var useFileSystem_1 = require("./useFileSystem");
function useImageConverter() {
    var _this = this;
    var fileSystem = (0, useFileSystem_1.useFileSystem)();
    var settingsStore = (0, store_1.useSettingsStore)();
    /**利用可能な拡張子 */
    var extensions = [
        // Imageクレートのサポートする拡張子
        'avif',
        'bmp',
        'dds',
        'ff',
        'gif',
        'hdr',
        'ico',
        'jpg',
        'jpeg',
        'exr',
        'png',
        'pnm',
        'qoi',
        'tga',
        'tif',
        'tiff',
        'webp',
        // 追加対応の拡張子
        'heic',
        'heif',
        'jp2',
        'j2k',
        'jxl'
    ];
    /**
     * 単一ファイルの変換処理
     * @param input 入力ファイルのパス
     * @param options 変換パラメータ
     */
    var convert = function (input, output) { return __awaiter(_this, void 0, void 0, function () {
        var pathInfo, fileName, buffer, _a, outputFileName, savePath, _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0: return [4 /*yield*/, fileSystem.pathInfo(input)];
                case 1:
                    pathInfo = _c.sent();
                    if (!pathInfo.exists || !pathInfo.isFile) {
                        return [2 /*return*/];
                    }
                    fileName = pathInfo.fileName;
                    _a = compress;
                    return [4 /*yield*/, fileSystem.read(input)];
                case 2: return [4 /*yield*/, _a.apply(void 0, [_c.sent()])];
                case 3:
                    buffer = _c.sent();
                    outputFileName = "".concat(fileName.split('.').slice(0, -1).join('.'), ".").concat(settingsStore.commonOptions.format);
                    if (!output) return [3 /*break*/, 5];
                    return [4 /*yield*/, (0, path_1.join)(output, outputFileName)]; // 出力先を指定して保存
                case 4:
                    _b = _c.sent(); // 出力先を指定して保存
                    return [3 /*break*/, 7];
                case 5: return [4 /*yield*/, (0, path_1.join)(pathInfo.parentDir, outputFileName)];
                case 6:
                    _b = _c.sent();
                    _c.label = 7;
                case 7:
                    savePath = _b;
                    // 保存処理
                    return [4 /*yield*/, fileSystem.save(savePath, buffer)];
                case 8:
                    // 保存処理
                    _c.sent();
                    return [2 /*return*/];
            }
        });
    }); };
    /**
     * 圧縮処理
     * @param data 元バイナリデータ
     * @returns 圧縮済みバイナリデータ
     */
    var compress = function (data) { return __awaiter(_this, void 0, void 0, function () {
        var optionsMap, format, options, _a, e_1;
        var _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0:
                    optionsMap = {
                        avif: settingsStore.avifOptions,
                        jxl: settingsStore.jxlOptions,
                        webp: settingsStore.webpOptions
                    };
                    format = settingsStore.commonOptions.format;
                    // マップに指定されたフォーマットが存在するかチェック
                    if (!(format in optionsMap)) {
                        throw new Error('Unsupported format');
                    }
                    options = (_b = {},
                        _b[format] = (0, vue_1.toRaw)(optionsMap[format]),
                        _b);
                    console.log(options);
                    _c.label = 1;
                case 1:
                    _c.trys.push([1, 3, , 4]);
                    _a = Uint8Array.bind;
                    return [4 /*yield*/, (0, core_1.invoke)('convert', { data: data, options: options })];
                case 2: 
                // rust側のVec<8>はnumber[]型になるのでUint8Arrayに変換する
                return [2 /*return*/, new (_a.apply(Uint8Array, [void 0, _c.sent()]))()];
                case 3:
                    e_1 = _c.sent();
                    console.error(e_1);
                    throw e_1;
                case 4: return [2 /*return*/];
            }
        });
    }); };
    return { extensions: extensions, convert: convert, compress: compress };
}
