import { useGlobalStore } from '@/store';
import { ref, computed, type Ref, nextTick } from 'vue';
import type { ComposerTranslation } from 'vue-i18n';

import { listen } from '@tauri-apps/api/event';
import { documentDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/plugin-dialog';

import { useImageConverter } from './useImageConverter'; // 汎用コンバーターをインポート

import { collectFiles } from '@/utilities/collectFiles';

const docDir = await documentDir();
// ... 他のインポート

export function useImageConversionController(t: ComposerTranslation) {
  const globalStore = useGlobalStore();
  const { convert } = useImageConverter(); // コアロジックを取得

  // --- UIの状態管理 ---
  const dialog = ref(false); // 進捗ダイアログ表示制御
  const currentFile: Ref<string | undefined> = ref(); // 現在のファイル
  const inProgress = ref(false); // 処理中フラグ
  const progress: Ref<string | number | undefined> = ref(); // 進捗
  // ...

  // --- 変換オプション ---
  const isLossless = ref(true); // ロスレス
  const targetFormat = ref<'webp' | 'avif'>('webp'); // フォーマットを選択できるように
  const ignoreJpeg = ref(false); // JPEGを無視
  const isOverwrite = ref(true); // 上書き
  const quality = ref(80); // 品質
  const isRecursive = ref(false); // サブディレクトリを含むか
  const isSameDirectory = ref(true); // 同じディレクトリに出力するか
  const isDeleteOriginal = ref(false); // オリジナルを削除するか
  const outputPath: Ref<string | null> = ref(docDir); // 出力先
  // ...

  // 拡張子のマッチパターン
  const imageRegExp = computed(() =>
    ignoreJpeg.value
      ? /\.(png|gif|tif?f|bmp|heic|heif)$/i
      : /\.(jpe?g|png|gif|tif?f|bmp|heic|heif)$/i
  );
  const browse = async () => {
    outputPath.value = await open({
      multiple: false,
      directory: true
    });
  };

  // 変換処理
  const processFiles = async (files: string[]) => {
    dialog.value = true;
    inProgress.value = true;
    progress.value = undefined;

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      if (!file) {
        continue;
      }
      try {
        // 汎用コンバーターを呼び出す
        await convert(
          targetFormat.value,
          file,
          isSameDirectory.value ? null : outputPath.value,
          isOverwrite.value,
          {
            quality: isLossless.value ? 100 : quality.value
          }
        );
      } catch (e: unknown) {
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
    const files = await collectFiles(inputs, imageRegExp.value, isRecursive.value);
    if (!files.length) {
      globalStore.setMessage(t('error.no_images_found_dropped'));
      return;
    }
    await processFiles(files);
  });

  // ファイル選択
  const convertByDialog = async () => {
    const selected = await open({
      multiple: true,
      directory: false,
      filters: [
        {
          name: 'Image',
          extensions: ['png', 'jpeg', 'jpg', 'tif', 'tiff', 'gif', 'bmp', 'heic', 'heif']
        }
      ]
    });
    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    const files = await collectFiles(paths, imageRegExp.value, isRecursive.value);

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
    progress.value = undefined;
    currentFile.value = t('scanning');
    await nextTick();

    const files = await collectFiles(dir, imageRegExp.value, isRecursive.value);

    if (!files.length) {
      dialog.value = false;
      progress.value = undefined;
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
    isLossless,
    quality,
    isRecursive,
    ignoreJpeg,
    isOverwrite,
    dialog,
    inProgress,
    currentFile,
    progress,
    isSameDirectory,
    outputPath,
    isDeleteOriginal,
    // methods
    browse,
    convertByDialog,
    convertByDirDialog
  };
}
