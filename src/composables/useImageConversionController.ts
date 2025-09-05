import { useGlobalStore, useSettingsStore } from '@/store';
import { ref, type Ref, nextTick } from 'vue';
import type { ComposerTranslation } from 'vue-i18n';

import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';

import { useFileSystem } from './useFileSystem';
import { useImageConverter } from './useImageConverter'; // 汎用コンバーターをインポート

export function useImageConversionController(t: ComposerTranslation) {
  const globalStore = useGlobalStore();
  const fileSystem = useFileSystem();
  const settingStore = useSettingsStore();
  const { convert } = useImageConverter(); // コアロジックを取得

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
      if (!file || !settingStore.extensionPattern.test(file)) {
        // 拡張子がマッチしない場合はスキップ
        continue;
      }
      currentFile.value = file;
      try {
        // 汎用コンバーターを呼び出す
        await convert(
          file,
          settingStore.commonOptions.sameDirectory
            ? undefined
            : settingStore.commonOptions.outputPath
        );
      } catch (e: unknown) {
        console.error(e);
        if (e instanceof Error) {
          globalStore.setMessage(e.message);
        } else {
          globalStore.setMessage(String(e));
        }
        dialog.value = false;
        inProgress.value = false;
        return;
      }
      progress.value = Math.floor(((i + 1) / files.length) * 100);
    }

    dialog.value = false;
    inProgress.value = false;
  };

  // D&D
  listen('tauri://drag-drop', async e => {
    const inputs = (e.payload as { paths: string[] }).paths;
    const files = await fileSystem.collectFiles(
      inputs,
      settingStore.extensionPattern,
      settingStore.commonOptions.recursive
    );
    if (!files.length) {
      globalStore.setMessage(t('error.no_images_found_dropped'));
      return;
    }
    await processFiles(files);
  });

  // ファイル選択
  const convertByDialog = async () => {
    // ダイアログを表示
    const selected = await open({
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
    const paths = Array.isArray(selected) ? selected : [selected];
    // ファイルリストを作成
    const files = await fileSystem.collectFiles(
      paths,
      settingStore.extensionPattern,
      settingStore.commonOptions.recursive
    );

    if (!files.length) {
      globalStore.setMessage(t('error.no_images_found_selected'));
      return;
    }
    await processFiles(files);
  };

  // フォルダ選択
  const convertByDirDialog = async () => {
    const picked = await open({ directory: true, recursive: true });
    if (!picked) return;
    const dir = Array.isArray(picked) ? picked[0] : picked;

    dialog.value = true;
    inProgress.value = true;
    progress.value = 0;
    currentFile.value = t('scanning');
    await nextTick();

    const files = await fileSystem.collectFiles(
      dir,
      settingStore.extensionPattern,
      settingStore.commonOptions.recursive
    );

    if (!files.length) {
      dialog.value = false;
      progress.value = 0;
      inProgress.value = false;
      globalStore.setMessage(t('error.no_images_found_in_folder'));
      return;
    }
    await processFiles(files);
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
