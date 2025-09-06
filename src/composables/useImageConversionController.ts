import { useGlobalStore, useSettingsStore } from '@/store';
import { ref, type Ref, nextTick } from 'vue';
import type { ComposerTranslation } from 'vue-i18n';

import { listen } from '@tauri-apps/api/event';
import { sep } from '@tauri-apps/api/path';
import { open, save } from '@tauri-apps/plugin-dialog';
import { useSound } from '@vueuse/sound';

// eslint-disable-next-line import/no-unresolved
import completeSound from '../assets/sounds/complete.mp3';
// eslint-disable-next-line import/no-unresolved
import errorSound from '../assets/sounds/error.mp3';

import { useFileSystem } from './useFileSystem';
import { useImageConverter } from './useImageConverter'; // 汎用コンバーターをインポート
import { usePaste } from './usePaste';

export function useImageConversionController(t: ComposerTranslation) {
  const globalStore = useGlobalStore();
  const fileSystem = useFileSystem();
  const settingsStore = useSettingsStore();

  const { play: playCompleteSound } = useSound(completeSound);
  const { play: playErrorSound } = useSound(errorSound);

  const { convert, compress } = useImageConverter(); // コアロジックを取得

  // --- UIの状態管理 ---
  const dialog = ref(false); // 進捗ダイアログ表示制御
  const currentFile: Ref<string | undefined> = ref(); // 現在のファイル
  const inProgress = ref(false); // 処理中フラグ
  const progress: Ref<number> = ref(0); // 進捗

  // 変換処理
  const processFiles = async (files: string[]) => {
    dialog.value = true;
    inProgress.value = true;
    progress.value = 0;

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      if (!file || !settingsStore.extensionPattern.test(file)) {
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
      } catch (e: unknown) {
        console.error(e);
        if (e instanceof Error) {
          globalStore.setMessage(e.message);
        } else {
          globalStore.setMessage(String(e));
        }
        playErrorSound();
        dialog.value = false;
        inProgress.value = false;
        return;
      }
      progress.value = Math.floor(((i + 1) / files.length) * 100);
    }

    dialog.value = false;
    inProgress.value = false;
    playCompleteSound();
    globalStore.setMessage(t('completed'));
  };

  // D&D
  listen('tauri://drag-drop', async e => {
    const inputs = (e.payload as { paths: string[] }).paths;
    const files = await fileSystem.collectFiles(
      inputs,
      settingsStore.extensionPattern,
      settingsStore.commonOptions.recursive
    );
    if (!files.length) {
      globalStore.setMessage(t('error.no_images_found_dropped'));
      playErrorSound();
      return;
    }
    await processFiles(files);
  });

  // ペースト処理
  async function handlePaste(event: ClipboardEvent) {
    const items = event.clipboardData?.items;
    if (!items) return;

    globalStore.setLoading(true);
    for (const item of items) {
      if (item.type.startsWith('image/')) {
        const file = item.getAsFile();
        if (!file) continue;
        const buffer = new Uint8Array(await file.arrayBuffer());
        const savePath = await save({
          title: t('save_as_title'),
          defaultPath: `${settingsStore.commonOptions.outputPath}${sep()}image.${
            settingsStore.commonOptions.format
          }`,
          filters: [
            settingsStore.commonOptions.format === 'webp'
              ? { name: t('type.webp'), extensions: ['webp'] }
              : { name: t('type.avif'), extensions: ['avif'] }
          ]
        });
        if (savePath) {
          const converted = await compress(buffer);
          await fileSystem.save(savePath, converted);
          globalStore.setMessage(t('completed'));
        }
      }
    }
    globalStore.setLoading(false);
  }
  usePaste(handlePaste);

  // ファイル選択
  const convertByDialog = async () => {
    try {
      // ダイアログを表示
      const selected = await open({
        title: t('select_files_title'),
        multiple: true,
        directory: false,
        filters: [
          {
            name: 'Image',
            extensions: [
              'png',
              'jpeg',
              'jpg',
              'tif',
              'tiff',
              'gif',
              'bmp',
              'heic',
              'heif',
              'jp2',
              'j2k'
            ]
          }
        ]
      });
      if (!selected) return;
      dialog.value = true;
      inProgress.value = true;
      progress.value = 0;
      currentFile.value = t('scanning');
      await nextTick();
      const paths = Array.isArray(selected) ? selected : [selected];
      // ファイルリストを作成
      const files = await fileSystem.collectFiles(
        paths,
        settingsStore.extensionPattern,
        settingsStore.commonOptions.recursive
      );

      if (!files.length) {
        globalStore.setMessage(t('error.no_images_found_selected'));
        playErrorSound();
        dialog.value = false;
        progress.value = 0;
        inProgress.value = false;
        return;
      }
      await processFiles(files);
    } catch (e) {
      console.error(e);
      if (e instanceof Error) {
        globalStore.setMessage(e.message);
      } else {
        globalStore.setMessage(String(e));
      }
      playErrorSound();
      return;
    }
  };

  // フォルダ選択
  const convertByDirDialog = async () => {
    try {
      const picked = await open({
        title: t('select_directory_title'),
        directory: true,
        recursive: true
      });

      if (!picked) return;
      const dir = Array.isArray(picked) ? picked[0] : picked;

      dialog.value = true;
      inProgress.value = true;
      progress.value = 0;
      currentFile.value = t('scanning');
      await nextTick();

      const files = await fileSystem.collectFiles(
        dir,
        settingsStore.extensionPattern,
        settingsStore.commonOptions.recursive
      );

      if (!files.length) {
        dialog.value = false;
        progress.value = 0;
        inProgress.value = false;
        globalStore.setMessage(t('error.no_images_found_in_folder'));
        playErrorSound();
        return;
      }
      await processFiles(files);
    } catch (e) {
      console.error(e);
      if (e instanceof Error) {
        globalStore.setMessage(e.message);
      } else {
        globalStore.setMessage(String(e));
      }
      dialog.value = false;
      progress.value = 0;
      inProgress.value = false;
      playErrorSound();
      return;
    }
  };

  // ファイル選択やD&Dのロジックはここに残す
  // ... listen('tauri://drag-drop', ...) や convertByDialog など
  // これらのメソッドは最終的に `processFiles` を呼び出す

  return {
    // state
    dialog,
    inProgress,
    currentFile,
    progress,
    // methods
    convertByDialog,
    convertByDirDialog
  };
}
