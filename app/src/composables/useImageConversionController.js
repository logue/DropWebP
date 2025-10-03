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
exports.useImageConversionController = useImageConversionController;
var store_1 = require("@/store");
var vue_1 = require("vue");
var event_1 = require("@tauri-apps/api/event");
var path_1 = require("@tauri-apps/api/path");
var plugin_dialog_1 = require("@tauri-apps/plugin-dialog");
var sound_1 = require("@vueuse/sound");
var complete_mp3_1 = require("../assets/sounds/complete.mp3");
var error_mp3_1 = require("../assets/sounds/error.mp3");
var useFileSystem_1 = require("./useFileSystem");
var useImageConverter_1 = require("./useImageConverter"); // 汎用コンバーターをインポート
var usePaste_1 = require("./usePaste");
function useImageConversionController(t) {
    var _this = this;
    var globalStore = (0, store_1.useGlobalStore)();
    var fileSystem = (0, useFileSystem_1.useFileSystem)();
    var settingsStore = (0, store_1.useSettingsStore)();
    var playCompleteSound = (0, sound_1.useSound)(complete_mp3_1.default).play;
    var playErrorSound = (0, sound_1.useSound)(error_mp3_1.default).play;
    var _a = (0, useImageConverter_1.useImageConverter)(), convert = _a.convert, compress = _a.compress, extensions = _a.extensions; // コアロジックを取得
    // --- UIの状態管理 ---
    var dialog = (0, vue_1.ref)(false); // 進捗ダイアログ表示制御
    var currentFile = (0, vue_1.ref)(); // 現在のファイル
    var inProgress = (0, vue_1.ref)(false); // 処理中フラグ
    var progress = (0, vue_1.ref)(0); // 進捗
    var message = (0, vue_1.ref)(''); // ダイアログのメッセージ
    /**
     * 変換処理
     * @param files 変換対象のファイルパスのリスト
     */
    var processFiles = function (files) { return __awaiter(_this, void 0, void 0, function () {
        var i, file, pathInfo, e_1;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    dialog.value = true;
                    inProgress.value = true;
                    progress.value = 0;
                    return [4 /*yield*/, (0, vue_1.nextTick)()];
                case 1:
                    _a.sent();
                    i = 0;
                    _a.label = 2;
                case 2:
                    if (!(i < files.length)) return [3 /*break*/, 11];
                    file = files[i];
                    if (!file) {
                        return [3 /*break*/, 10];
                    }
                    return [4 /*yield*/, fileSystem.pathInfo(file)];
                case 3:
                    pathInfo = _a.sent();
                    message.value = t('progress', {
                        file: pathInfo.fileName,
                        type: t("type.".concat(settingsStore.commonOptions.format))
                    });
                    if (!settingsStore.commonOptions.overwrite && pathInfo.exists) {
                        // 上書き禁止オプションが有効で、出力先に同名ファイルが存在する場合はスキップ
                        console.info("Skipping ".concat(file, " as it already exists and overwrite is disabled."));
                        return [3 /*break*/, 10];
                    }
                    if (!extensions.includes(pathInfo.extension)) {
                        // 拡張子がマッチしない場合はスキップ
                        return [3 /*break*/, 10];
                    }
                    currentFile.value = file;
                    _a.label = 4;
                case 4:
                    _a.trys.push([4, 8, , 9]);
                    // 汎用コンバーターを呼び出す
                    return [4 /*yield*/, convert(file, settingsStore.commonOptions.sameDirectory
                            ? undefined
                            : settingsStore.commonOptions.outputPath)];
                case 5:
                    // 汎用コンバーターを呼び出す
                    _a.sent();
                    if (!settingsStore.commonOptions.deleteOriginal) return [3 /*break*/, 7];
                    // 元ファイル削除オプションが有効な場合、元ファイルを削除
                    return [4 /*yield*/, fileSystem.del(file)];
                case 6:
                    // 元ファイル削除オプションが有効な場合、元ファイルを削除
                    _a.sent();
                    console.info("Deleted original file: ".concat(file));
                    _a.label = 7;
                case 7: return [3 /*break*/, 9];
                case 8:
                    e_1 = _a.sent();
                    console.error(file, e_1);
                    dialog.value = false;
                    inProgress.value = false;
                    if (e_1 instanceof Error) {
                        globalStore.setMessage(e_1.message, 'red');
                    }
                    else {
                        globalStore.setMessage(String(e_1), 'red');
                    }
                    playErrorSound();
                    return [2 /*return*/];
                case 9:
                    progress.value = Math.floor(((i + 1) / files.length) * 100);
                    _a.label = 10;
                case 10:
                    i++;
                    return [3 /*break*/, 2];
                case 11:
                    dialog.value = false;
                    inProgress.value = false;
                    playCompleteSound();
                    globalStore.setMessage(t('completed'), 'success');
                    return [2 /*return*/];
            }
        });
    }); };
    /** パスリストからファイル一覧を出力する */
    var scanFiles = function (paths) { return __awaiter(_this, void 0, void 0, function () {
        var files, e_2;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    dialog.value = true;
                    inProgress.value = true;
                    progress.value = 0;
                    currentFile.value = t('scanning');
                    return [4 /*yield*/, (0, vue_1.nextTick)()];
                case 1:
                    _a.sent();
                    files = [];
                    _a.label = 2;
                case 2:
                    _a.trys.push([2, 4, 5, 6]);
                    return [4 /*yield*/, fileSystem.collectFiles(paths, settingsStore.commonOptions.recursive)];
                case 3:
                    files = _a.sent();
                    return [3 /*break*/, 6];
                case 4:
                    e_2 = _a.sent();
                    console.error(paths, e_2);
                    if (e_2 instanceof Error) {
                        globalStore.setMessage(e_2.message);
                    }
                    else {
                        globalStore.setMessage(String(e_2));
                    }
                    return [3 /*break*/, 6];
                case 5:
                    dialog.value = false;
                    progress.value = 0;
                    inProgress.value = false;
                    return [7 /*endfinally*/];
                case 6:
                    if (!files.length) {
                        globalStore.setMessage(t('error.no_images_found_selected'));
                        playErrorSound();
                        return [2 /*return*/];
                    }
                    return [2 /*return*/, files];
            }
        });
    }); };
    // D&D
    (0, event_1.listen)('tauri://drag-drop', function (e) { return __awaiter(_this, void 0, void 0, function () {
        var inputs, files;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    inputs = e.payload.paths;
                    return [4 /*yield*/, scanFiles(inputs)];
                case 1:
                    files = _a.sent();
                    if (!files) {
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, processFiles(files)];
                case 2:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    }); });
    // ペースト処理
    function handlePaste(event) {
        return __awaiter(this, void 0, void 0, function () {
            var items, _i, items_1, item, file, buffer, _a, filtersMap, format, savePath, converted;
            var _b;
            return __generator(this, function (_c) {
                switch (_c.label) {
                    case 0:
                        items = (_b = event.clipboardData) === null || _b === void 0 ? void 0 : _b.items;
                        if (!items)
                            return [2 /*return*/];
                        globalStore.setLoading(true);
                        _i = 0, items_1 = items;
                        _c.label = 1;
                    case 1:
                        if (!(_i < items_1.length)) return [3 /*break*/, 7];
                        item = items_1[_i];
                        // 画像でない場合はスキップ
                        if (!item.type.startsWith('image/')) {
                            return [3 /*break*/, 6];
                        }
                        file = item.getAsFile();
                        if (!file)
                            return [3 /*break*/, 6];
                        _a = Uint8Array.bind;
                        return [4 /*yield*/, file.arrayBuffer()];
                    case 2:
                        buffer = new (_a.apply(Uint8Array, [void 0, _c.sent()]))();
                        filtersMap = {
                            avif: { name: t('type.avif'), extensions: ['avif'] },
                            jxl: { name: t('type.jxl'), extensions: ['jxl'] },
                            webp: { name: t('type.webp'), extensions: ['webp'] }
                        };
                        format = settingsStore.commonOptions.format;
                        if (!(format in filtersMap)) {
                            throw new Error('Unsupported format');
                        }
                        return [4 /*yield*/, (0, plugin_dialog_1.save)({
                                title: t('save_as_title'),
                                defaultPath: "".concat(settingsStore.commonOptions.outputPath).concat((0, path_1.sep)(), "image.").concat(settingsStore.commonOptions.format),
                                filters: [filtersMap[format]]
                            })];
                    case 3:
                        savePath = _c.sent();
                        if (!savePath) {
                            // キャンセルボタンが押された場合処理しない
                            return [3 /*break*/, 6];
                        }
                        return [4 /*yield*/, compress(buffer)];
                    case 4:
                        converted = _c.sent();
                        return [4 /*yield*/, fileSystem.save(savePath, converted)];
                    case 5:
                        _c.sent();
                        _c.label = 6;
                    case 6:
                        _i++;
                        return [3 /*break*/, 1];
                    case 7:
                        globalStore.setMessage(t('completed'), 'success');
                        globalStore.setLoading(false);
                        return [2 /*return*/];
                }
            });
        });
    }
    (0, usePaste_1.usePaste)(handlePaste);
    // ファイル選択
    var convertByDialog = function () { return __awaiter(_this, void 0, void 0, function () {
        var selected, e_3, files;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    selected = [];
                    _a.label = 1;
                case 1:
                    _a.trys.push([1, 3, , 4]);
                    return [4 /*yield*/, (0, plugin_dialog_1.open)({
                            title: t('select_files_title'),
                            multiple: true,
                            directory: false,
                            filters: [{ name: 'Image', extensions: extensions }]
                        })];
                case 2:
                    // ダイアログを表示
                    selected = _a.sent();
                    return [3 /*break*/, 4];
                case 3:
                    e_3 = _a.sent();
                    console.error(e_3);
                    return [3 /*break*/, 4];
                case 4:
                    console.log(selected);
                    if (!selected)
                        return [2 /*return*/];
                    return [4 /*yield*/, scanFiles(selected)];
                case 5:
                    files = _a.sent();
                    if (!files) {
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, processFiles(files)];
                case 6:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    }); };
    // フォルダを選択ボタンが押された
    var convertByDirDialog = function () { return __awaiter(_this, void 0, void 0, function () {
        var picked, e_4, dir, files;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    picked = null;
                    _a.label = 1;
                case 1:
                    _a.trys.push([1, 3, , 4]);
                    return [4 /*yield*/, (0, plugin_dialog_1.open)({
                            title: t('select_directory_title'),
                            directory: true,
                            recursive: true
                        })];
                case 2:
                    picked = _a.sent();
                    return [3 /*break*/, 4];
                case 3:
                    e_4 = _a.sent();
                    console.error(e_4);
                    return [3 /*break*/, 4];
                case 4:
                    if (!picked)
                        return [2 /*return*/];
                    dir = Array.isArray(picked) ? picked[0] : picked;
                    return [4 /*yield*/, scanFiles(dir)];
                case 5:
                    files = _a.sent();
                    if (!files) {
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, processFiles(files)];
                case 6:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    }); };
    return {
        // state
        dialog: dialog,
        inProgress: inProgress,
        currentFile: currentFile,
        progress: progress,
        message: message,
        // methods
        convertByDialog: convertByDialog,
        convertByDirDialog: convertByDirDialog
    };
}
