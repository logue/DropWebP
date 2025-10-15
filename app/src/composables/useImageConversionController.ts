import { useGlobalStore, useSettingsStore } from '@/store';
import { ref, type Ref, nextTick } from 'vue';
import type { ComposerTranslation } from 'vue-i18n';

import { open, save } from '@tauri-apps/plugin-dialog';
import { useSound } from '@vueuse/sound';

import completeSound from '../assets/sounds/complete.mp3';
import errorSound from '../assets/sounds/error.mp3';

import { useDragAndDrop } from './useDragAndDrop';
import { useFileSystem } from './useFileSystem';
import { useImageConverter } from './useImageConverter'; // 汎用コンバーターをインポート
import { useNotification } from './useNotification';
import { usePaste } from './usePaste';

export function useImageConversionController(t: ComposerTranslation) {
  const globalStore = useGlobalStore();
  const fileSystem = useFileSystem();
  const settingsStore = useSettingsStore();

  const { play: playCompleteSound } = useSound(completeSound, {
    volume: settingsStore.commonOptions.volume
  });
  const { play: playErrorSound } = useSound(errorSound, {
    volume: settingsStore.commonOptions.volume
  });

  const { convert, compress, extensions } = useImageConverter(); // コアロジックを取得
  const { notifyConversionComplete, notifyBatchComplete, notifyError } = useNotification();

  // --- UIの状態管理 ---
  const dialog = ref(false); // 進捗ダイアログ表示制御
  const currentFile: Ref<string | undefined> = ref(); // 現在のファイル
  const inProgress = ref(false); // 処理中フラグ
  const progress: Ref<number> = ref(0); // 進捗
  const message: Ref<string> = ref(''); // ダイアログのメッセージ

  /**
   * 変換処理
   * @param files 変換対象のファイルパスのリスト
   */
  const processFiles = async (files: string[]) => {
    dialog.value = true;
    inProgress.value = true;
    progress.value = 0;

    await nextTick();

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      if (!file) {
        continue;
      }
      // 進捗メッセージを更新
      const pathInfo = await fileSystem.pathInfo(file);
      message.value = t('progress', {
        type: t(`type.${settingsStore.commonOptions.format}`)
      });
      if (!settingsStore.commonOptions.overwrite && pathInfo.exists) {
        // 上書き禁止オプションが有効で、出力先に同名ファイルが存在する場合はスキップ
        console.info(`Skipping ${file} as it already exists and overwrite is disabled.`);
        continue;
      }
      if (!extensions.includes(pathInfo.extension)) {
        // 拡張子がマッチしない場合はスキップ
        continue;
      }

      currentFile.value = file;
      try {
        // 汎用コンバーターを呼び出す
        await convert(
          file,
          settingsStore.commonOptions.sameDirectory
            ? undefined
            : settingsStore.commonOptions.outputPath
        );
        if (settingsStore.commonOptions.deleteOriginal) {
          // 元ファイル削除オプションが有効な場合、元ファイルを削除
          await fileSystem.del(file);
          console.info(`Deleted original file: ${file}`);
        }
      } catch (e) {
        console.error(file, e);
        dialog.value = false;
        inProgress.value = false;
        const errorMessage = e instanceof Error ? e.message : String(e);
        globalStore.setMessage(errorMessage, 'red');

        // エラー通知を送信
        notifyError(errorMessage);
        if (settingsStore.commonOptions.sound) {
          playErrorSound();
        }
        return;
      }
      progress.value = Math.floor(((i + 1) / files.length) * 100);
    }

    // すべての変換が完了した場合の通知
    const format = settingsStore.commonOptions.format;
    if (files.length > 1) {
      notifyBatchComplete(files.length, format);
    } else if (files.length === 1 && files[0]) {
      // 単一ファイルの場合はファイル名を取得
      const pathInfo = await fileSystem.pathInfo(files[0]);
      notifyConversionComplete(pathInfo.fileName, format);
    }

    globalStore.setMessage(t('completed'), 'success');

    dialog.value = false;
    inProgress.value = false;
    if (settingsStore.commonOptions.sound) {
      playCompleteSound();
    }
  };

  /** パスリストからファイル一覧を出力する */
  const scanFiles = async (paths: string[]): Promise<string[] | undefined> => {
    dialog.value = true;
    inProgress.value = true;
    progress.value = 0;
    currentFile.value = t('scanning');
    await nextTick();

    // ファイルリストを作成
    let files: string[] = [];
    try {
      files = await fileSystem.collectFiles(paths, settingsStore.commonOptions.recursive);
    } catch (e) {
      console.error(paths, e);
      if (e instanceof Error) {
        globalStore.setMessage(e.message);
      } else {
        globalStore.setMessage(String(e));
      }
    } finally {
      dialog.value = false;
      progress.value = 0;
      inProgress.value = false;
    }

    if (!files.length) {
      globalStore.setMessage(t('error.no_images_found_selected'));
      if (settingsStore.commonOptions.sound) {
        playErrorSound();
      }
      return;
    }

    return files;
  };

  // ファイル選択
  const convertByDialog = async () => {
    let selected: string[] | null = [];
    try {
      // ダイアログを表示
      selected = await open({
        title: t('select_files_title'),
        multiple: true,
        directory: false,
        filters: [{ name: t('image'), extensions }]
      });
    } catch (e) {
      console.error(e);
    }
    if (!selected) return;

    const files = await scanFiles(selected);
    if (!files) {
      return;
    }
    await processFiles(files);
  };

  // フォルダを選択ボタンが押された
  const convertByDirDialog = async () => {
    let picked: string | null = null;
    try {
      picked = await open({
        title: t('select_directory_title'),
        directory: true,
        recursive: true
      });
    } catch (e) {
      console.error(e);
    }

    if (!picked) return;
    const dir = Array.isArray(picked) ? picked[0] : picked;
    // ディレクトリを走査
    const files = await scanFiles(dir);
    if (!files) {
      return;
    }
    await processFiles(files);
  };

  // クリップボードからのペーストイベントを監視
  usePaste(async (event: ClipboardEvent) => {
    // クリップボード内のデータを取得
    const items = event.clipboardData?.items;
    if (!items) return;

    globalStore.setLoading(true);
    for (const item of items) {
      // 画像でない場合はスキップ
      if (!item.type.startsWith('image/')) {
        continue;
      }
      // ドラッグ&ドロップされたものはファイルとする
      const file = item.getAsFile();
      if (!file) continue;

      // ファイルをUint8Arrayバイナリとして読み込む
      const buffer = new Uint8Array(await file.arrayBuffer());

      const filtersMap = {
        avif: { name: t('type.avif'), extensions: ['avif'] },
        jxl: { name: t('type.jxl'), extensions: ['jxl'] },
        webp: { name: t('type.webp'), extensions: ['webp'] }
      };
      type Format = keyof typeof filtersMap;

      const format = settingsStore.commonOptions.format as Format;

      if (!(format in filtersMap)) {
        throw new Error('Unsupported format');
      }

      // 保存先のダイアログを表示
      const savePath = await save({
        title: t('save_as_title'),
        defaultPath: `${settingsStore.commonOptions.outputPath}${fileSystem.sep()}image.${
          settingsStore.commonOptions.format
        }`,
        filters: [filtersMap[format]]
      });
      if (!savePath) {
        // キャンセルボタンが押された場合処理しない
        continue;
      }
      // 圧縮処理
      const converted = await compress(buffer);
      await fileSystem.save(savePath, converted);
    }
    globalStore.setMessage(t('completed'), 'success');
    globalStore.setLoading(false);
    if (settingsStore.commonOptions.sound) {
      playCompleteSound();
    }
  });

  // ドラッグ&ドロップイベントを監視
  const { isDragging } = useDragAndDrop(async paths => {
    const files = await scanFiles(paths);
    if (!files) {
      return;
    }
    await processFiles(files);
  });

  return {
    // state
    dialog,
    inProgress,
    currentFile,
    progress,
    message,
    isDragging,
    // methods
    convertByDialog,
    convertByDirDialog
  };
}
