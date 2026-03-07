import { defineStore } from 'pinia';
import { ref, type Ref } from 'vue';

/** Global Store */
export default defineStore('global', () => {
  // State

  /** Loading overlay */
  const loading: Ref<boolean> = ref(true);
  /** ProgressBar Percentage */
  const progress: Ref<number | null> = ref(null);
  /** SnackBar Text */
  const message: Ref<string> = ref('');
  /** SnackBar Color */
  const snackbarColor: Ref<string | undefined> = ref();
  // Actions

  /**
   * Show loading Overlay
   *
   * @param display - visibility
   */
  function setLoading(display: boolean): void {
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
  function setProgress(v: number | null = null): void {
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
  function setMessage(msg = '', color?: string): void {
    // put snackbar text
    message.value = msg;
    snackbarColor.value = color;
  }

  return { loading, progress, message, snackbarColor, setLoading, setProgress, setMessage };
});
