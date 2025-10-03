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
exports.useFileSystem = useFileSystem;
var core_1 = require("@tauri-apps/api/core");
var path_1 = require("@tauri-apps/api/path");
var plugin_fs_1 = require("@tauri-apps/plugin-fs");
/** ファイルシステムコンポーサブル */
function useFileSystem() {
    /**
     * 指定されたファイルパスからファイルを読み込み、Uint8Arrayとして返す
     * @param filePath 読み込むファイルのフルパス
     * @returns ファイルのバイナリデータ
     */
    function read(filePath) {
        return __awaiter(this, void 0, void 0, function () {
            var contents, error_1;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, (0, plugin_fs_1.readFile)(filePath)];
                    case 1:
                        contents = _a.sent();
                        console.info("Successfully read ".concat(contents.length, " bytes from ").concat(filePath));
                        return [2 /*return*/, contents];
                    case 2:
                        error_1 = _a.sent();
                        console.error("Failed to read file: ".concat(filePath), error_1);
                        throw error_1;
                    case 3: return [2 /*return*/];
                }
            });
        });
    }
    /**
     * Uint8Arrayデータをファイルパスに保存する
     * @param path 保存先のフルパス
     * @param data 保存するバイナリデータ
     * @param isOverwrite 上書きするか
     */
    function save(path, data) {
        return __awaiter(this, void 0, void 0, function () {
            var error_2;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, (0, plugin_fs_1.writeFile)(path, data)];
                    case 1:
                        _a.sent();
                        console.info("Successfully saved file to ".concat(path));
                        return [3 /*break*/, 3];
                    case 2:
                        error_2 = _a.sent();
                        console.error('Failed to save file:', error_2);
                        return [3 /*break*/, 3];
                    case 3: return [2 /*return*/];
                }
            });
        });
    }
    /**
     * パスを削除する
     * @param path 削除するパス
     */
    function del(path) {
        return __awaiter(this, void 0, void 0, function () {
            var error_3;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, (0, core_1.invoke)('delete_path', { pathStr: path })];
                    case 1:
                        _a.sent(); // Rust側でdelete_pathコマンドを呼び出す
                        console.info("Successfully deleted: ".concat(path));
                        return [3 /*break*/, 3];
                    case 2:
                        error_3 = _a.sent();
                        console.error("Failed to delete path: ".concat(path), error_3);
                        throw error_3;
                    case 3: return [2 /*return*/];
                }
            });
        });
    }
    /**
     * ディレクトリからファイルを収集する
     * @param path ディレクトリ
     * @param recursive 再起的に探索するか
     * @returns ファイルパスの配列
     */
    function collectFilesFromDir(path_2) {
        return __awaiter(this, arguments, void 0, function (path, recursive) {
            var entries, files, _i, entries_1, entry, fullPath, sub;
            if (recursive === void 0) { recursive = false; }
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, (0, plugin_fs_1.readDir)(path)];
                    case 1:
                        entries = _a.sent();
                        files = [];
                        _i = 0, entries_1 = entries;
                        _a.label = 2;
                    case 2:
                        if (!(_i < entries_1.length)) return [3 /*break*/, 7];
                        entry = entries_1[_i];
                        return [4 /*yield*/, (0, path_1.join)(path, entry.name)];
                    case 3:
                        fullPath = _a.sent();
                        if (!entry.isFile) return [3 /*break*/, 4];
                        files.push(fullPath);
                        return [3 /*break*/, 6];
                    case 4:
                        if (!(recursive && entry.isDirectory)) return [3 /*break*/, 6];
                        return [4 /*yield*/, collectFilesFromDir(fullPath, recursive)];
                    case 5:
                        sub = _a.sent();
                        files = files.concat(sub);
                        _a.label = 6;
                    case 6:
                        _i++;
                        return [3 /*break*/, 2];
                    case 7: return [2 /*return*/, files];
                }
            });
        });
    }
    /**
     * ファイル or フォルダのパス配列を受け取ってファイル一覧に正規化
     * @param paths 入力パス配列
     * @param recursive 再起的に探索するか
     * @returns ファイルパスの配列
     */
    function collectFiles(paths_1) {
        return __awaiter(this, arguments, void 0, function (paths, recursive) {
            var results, _i, paths_2, path, subFiles;
            if (recursive === void 0) { recursive = false; }
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        results = [];
                        _i = 0, paths_2 = paths;
                        _a.label = 1;
                    case 1:
                        if (!(_i < paths_2.length)) return [3 /*break*/, 6];
                        path = paths_2[_i];
                        return [4 /*yield*/, pathInfo(path)];
                    case 2:
                        if (!(_a.sent()).isDir) return [3 /*break*/, 4];
                        return [4 /*yield*/, collectFilesFromDir(path, recursive)];
                    case 3:
                        subFiles = _a.sent();
                        results = results.concat(subFiles);
                        return [3 /*break*/, 5];
                    case 4:
                        results = results.concat(path);
                        _a.label = 5;
                    case 5:
                        _i++;
                        return [3 /*break*/, 1];
                    case 6: return [2 /*return*/, results];
                }
            });
        });
    }
    /**
     * パスからファイル名などを取得
     * @param path パス文字列
     * @returns ファイル名、拡張子、親ディレクトリ名
     */
    function pathInfo(path) {
        return __awaiter(this, void 0, void 0, function () {
            var ret, error_4;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, (0, core_1.invoke)('get_path_info', { pathStr: path })];
                    case 1:
                        ret = _a.sent();
                        // 出力前にミューテーションする
                        return [2 /*return*/, {
                                fileName: ret.fileName,
                                extension: ret.extension ? ret.extension.toLowerCase() : ret.extension, // 拡張子は常に小文字にする
                                parentDir: ret.parentDir + (0, path_1.sep)(), // 親ディレクトリの末尾の/がつかないのでここで追記
                                isFile: ret.isFile,
                                isDir: ret.isDir,
                                exists: ret.exists
                            }];
                    case 2:
                        error_4 = _a.sent();
                        console.error('Failed to parse path:', error_4);
                        throw error_4;
                    case 3: return [2 /*return*/];
                }
            });
        });
    }
    return { read: read, save: save, del: del, collectFiles: collectFiles, pathInfo: pathInfo };
}
