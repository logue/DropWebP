"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var pinia_1 = require("pinia");
var vue_1 = require("vue");
/** Global Store */
exports.default = (0, pinia_1.defineStore)('global', function () {
    // State
    /** Loading overlay */
    var loading = (0, vue_1.ref)(true);
    /** ProgressBar Percentage */
    var progress = (0, vue_1.ref)(null);
    /** SnackBar Text */
    var message = (0, vue_1.ref)('');
    /** SnackBar Color */
    var snackbarColor = (0, vue_1.ref)();
    // Actions
    /**
     * Show loading Overlay
     *
     * @param display - visibility
     */
    function setLoading(display) {
        loading.value = display;
        if (!display) {
            // Reset Progress value
            progress.value = null;
        }
    }
    /**
     * Update progress value
     *
     * @param v - progress value
     */
    function setProgress(v) {
        if (v === void 0) { v = null; }
        // update progress value
        progress.value = v;
        // display loading overlay
        loading.value = v !== null;
    }
    /**
     * Show snackbar message
     *
     * @param msg - snackbar message
     * @param color - snackbar color
     */
    function setMessage(msg, color) {
        if (msg === void 0) { msg = ''; }
        // put snackbar text
        message.value = msg;
        snackbarColor.value = color;
    }
    return { loading: loading, progress: progress, message: message, snackbarColor: snackbarColor, setLoading: setLoading, setProgress: setProgress, setMessage: setMessage };
});
