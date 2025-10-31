import { ref, type Ref } from 'vue';

/**
 * 変換処理のUI状態を管理するComposable
 */
export function useConversionState() {
  /** 進捗ダイアログ表示制御 */
  const dialog = ref(false);

  /** 現在のファイル */
  const currentFile: Ref<string | undefined> = ref();

  /** 処理中フラグ */
  const inProgress = ref(false);

  /** 進捗 */
  const progress: Ref<number> = ref(0);

  /** ダイアログのメッセージ */
  const message: Ref<string> = ref('');

  /**
   * 状態をリセット
   */
  const reset = () => {
    dialog.value = false;
    inProgress.value = false;
    progress.value = 0;
    message.value = '';
    currentFile.value = undefined;
  };

  /**
   * 処理開始
   */
  const start = (msg: string) => {
    dialog.value = true;
    inProgress.value = true;
    progress.value = 0;
    message.value = msg;
  };

  /**
   * 処理終了
   */
  const complete = () => {
    dialog.value = false;
    inProgress.value = false;
  };

  /**
   * 進捗更新
   */
  const updateProgress = (current: number, total: number) => {
    progress.value = Math.floor((current / total) * 100);
  };

  return {
    // state
    dialog,
    currentFile,
    inProgress,
    progress,
    message,
    // actions
    reset,
    start,
    complete,
    updateProgress
  };
}
