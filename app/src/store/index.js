"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.useSettingsStore = exports.useGlobalStore = exports.useConfigStore = void 0;
var pinia_1 = require("pinia");
var pinia_plugin_persistedstate_1 = require("pinia-plugin-persistedstate");
// Pinia Stores
var ConfigStore_1 = require("@/store/ConfigStore");
exports.useConfigStore = ConfigStore_1.default;
var GlobalStore_1 = require("@/store/GlobalStore");
exports.useGlobalStore = GlobalStore_1.default;
var SettingsStore_1 = require("@/store/SettingsStore");
exports.useSettingsStore = SettingsStore_1.default;
/** Pinia Store */
var pinia = (0, pinia_1.createPinia)();
pinia.use(pinia_plugin_persistedstate_1.default);
exports.default = pinia;
