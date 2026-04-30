import { useGlobalStore, useSettingsStore } from '@/store';
import { nextTick } from 'vue';
import type { ComposerTranslation } from 'vue-i18n';

import { open, save } from '@tauri-apps/plugin-dialog';
import { useSound } from '@vueuse/sound';

import { useConversionState } from './useConversionState';
import { useDragAndDrop } from './useDragAndDrop';
import { useFileSystem } from './useFileSystem';
import { useFormatConfig } from './useFormatConfig';
import { useImageConverter } from './useImageConverter';
import { useNotification } from './useNotification';
import { usePaste } from './usePaste';

import completeSound from '@/assets/sounds/complete.mp3';
import errorSound from '@/assets/sounds/error.mp3';

/**
 * Composable that orchestrates the image conversion workflow,
 * coordinating file selection, conversion, notifications, sound feedback,
 * drag-and-drop, and clipboard paste handling.
 *
 * @param t - Vue I18n translation function used for localized messages.
 * @returns Reactive state and action methods for the conversion UI.
 */
export function useImageConversionController(t: ComposerTranslation) {
  const globalStore = useGlobalStore();
  const settingsStore = useSettingsStore();
  const fileSystem = useFileSystem();

  // Composableの初期化
  const state = useConversionState();
  const { getFormatLabel } = useFormatConfig(t);
  const { convert, compress, extensions } = useImageConverter();
  const { notifyConversionComplete, notifyBatchComplete, notifyError } = useNotification(t);

  // サウンド
  const { play: playCompleteSound } = useSound(completeSound, {
    volume: settingsStore.commonOptions.volume
  });
  const { play: playErrorSound } = useSound(errorSound, {
    volume: settingsStore.commonOptions.volume
  });

  /**
   * 変換処理
   * @param files 変換対象のファイルパスのリスト
   */
  const processFiles = async (files: string[]) => {
    const formatLabel = getFormatLabel(settingsStore.commonOptions.format);
    state.start(t('progress', { type: formatLabel }));
    await nextTick();

    for (let i = 0; i < files.length; i++) {
      // eslint-disable-next-line security/detect-object-injection -- i is a numeric loop index over a controlled array
      const file = files[i];
      if (!file) continue;

      const pathInfo = await fileSystem.pathInfo(file);

      // スキップ条件チェック
      if (!settingsStore.commonOptions.overwrite && pathInfo.exists) {
        console.info(`Skipping ${file} (already exists)`);
        continue;
      }
      if (!extensions.includes(pathInfo.extension)) continue;

      state.currentFile.value = file;

      try {
        await convert(
          file,
          settingsStore.commonOptions.sameDirectory
            ? undefined
            : settingsStore.commonOptions.outputPath
        );

        if (settingsStore.commonOptions.deleteOriginal) {
          await fileSystem.del(file);
          console.info(`Deleted original file: ${file}`);
        }
      } catch (e) {
        console.error(file, e);
        state.complete();
        const errorMessage = e instanceof Error ? e.message : String(e);
        globalStore.setMessage(errorMessage, 'red');

        if (settingsStore.commonOptions.notify) notifyError(errorMessage);
        if (settingsStore.commonOptions.sound) playErrorSound();
        return;
      }

      state.updateProgress(i + 1, files.length);
    }

    // 完了通知
    if (settingsStore.commonOptions.notify) {
      const format = settingsStore.commonOptions.format;
      if (files.length > 1) {
        notifyBatchComplete(files.length, format);
      } else if (files.length === 1 && files[0]) {
        const pathInfo = await fileSystem.pathInfo(files[0]);
        notifyConversionComplete(pathInfo.fileName, format);
      }
    }

    globalStore.setMessage(t('completed'), 'success');
    state.complete();
    if (settingsStore.commonOptions.sound) playCompleteSound();
  };

  /**
   * パスリストからファイル一覧を取得
   */
  const scanFiles = async (
    paths: string[],
    recursiveOverride?: boolean
  ): Promise<string[] | undefined> => {
    state.start(t('scanning'));
    state.currentFile.value = t('scanning');
    await nextTick();

    let files: string[] = [];
    try {
      files = await fileSystem.collectFiles(
        paths,
        recursiveOverride ?? settingsStore.commonOptions.recursive
      );
    } catch (e) {
      console.error(paths, e);
      globalStore.setMessage(e instanceof Error ? e.message : String(e));
    } finally {
      state.complete();
    }

    if (!files.length) {
      globalStore.setMessage(t('error.no_images_found_selected'));
      if (settingsStore.commonOptions.sound) playErrorSound();
      return;
    }

    return files;
  };

  /**
   * ファイル選択ダイアログ
   */
  const convertByDialog = async () => {
    const selected = await open({
      title: t('select_files_title'),
      multiple: true,
      directory: false,
      filters: [{ name: t('image'), extensions }]
    }).catch(console.error);

    if (!selected) return;

    const files = await scanFiles(selected);
    if (files) await processFiles(files);
  };

  /**
   * フォルダ選択ダイアログ
   */
  const convertByDirDialog = async () => {
    const picked = await open({
      title: t('select_directory_title'),
      directory: true,
      recursive: true
    }).catch(console.error);

    if (!picked) return;

    const dir = Array.isArray(picked) ? picked[0] : picked;
    const files = await scanFiles([dir], true);
    if (files) await processFiles(files);
  };

  /**
   * クリップボードからのペースト処理
   */
  const handlePaste = async (event: ClipboardEvent) => {
    const items = event.clipboardData?.items;
    if (!items) return;

    globalStore.setLoading(true);

    for (const item of items) {
      if (!item.type.startsWith('image/')) continue;

      const file = item.getAsFile();
      if (!file) continue;

      const buffer = new Uint8Array(await file.arrayBuffer());

      // 保存先ダイアログ
      const format = settingsStore.commonOptions.format;
      const formatLabel = getFormatLabel(format);

      const savePath = await save({
        title: t('save_as_title'),
        defaultPath: `${settingsStore.commonOptions.outputPath}${fileSystem.sep()}image.${format}`,
        filters: [{ name: formatLabel, extensions: [format] }]
      });

      if (!savePath) continue;

      const converted = await compress(buffer);
      await fileSystem.save(savePath, converted);
    }

    globalStore.setMessage(t('completed'), 'success');
    globalStore.setLoading(false);
    if (settingsStore.commonOptions.sound) playCompleteSound();
  };

  // イベント監視
  usePaste(handlePaste);

  const { isDragging } = useDragAndDrop(async paths => {
    const pathInfos = await Promise.all(paths.map(path => fileSystem.pathInfo(path)));
    const hasDirectory = pathInfos.some(path => path.isDir);
    const files = await scanFiles(paths, hasDirectory ? true : undefined);
    if (files) await processFiles(files);
  });

  return {
    // state
    ...state,
    isDragging,
    // methods
    convertByDialog,
    convertByDirDialog
  };
}
