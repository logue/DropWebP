"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.usePaste = usePaste;
var vue_1 = require("vue");
// クリップボードからペーストイベントを監視
function usePaste(handlePaste) {
    (0, vue_1.onMounted)(function () {
        window.addEventListener('paste', handlePaste);
    });
    (0, vue_1.onUnmounted)(function () {
        window.removeEventListener('paste', handlePaste);
    });
}
