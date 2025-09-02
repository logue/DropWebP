import { useGlobalStore } from '@/store';
import { ref, computed, nextTick, type Ref } from 'vue';

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open, save } from '@tauri-apps/plugin-dialog';

import { usePaste } from './usePaste';

import { collectFiles } from '@/utilities/collectFiles';

// WebP変換機能
export function useWebPConverter() {
  const globalStore = useGlobalStore();

  // ロスレス
  const isLossless = ref(true);
  // 品質
  const quality = ref(80);
  // ディレクトリ再帰
  const recursive = ref(false);
  // JPEGを無視
  const ignoreJpeg = ref(false);

  // 進捗ダイアログ表示制御
  const dialog = ref(false);
  // 処理中フラグ
  const inProgress = ref(false);
  // 現在のファイル
  const currentFile: Ref<string | undefined> = ref();
  // 進捗
  const progress: Ref<string | number | undefined> = ref();

  // 拡張子のマッチパターン
  const imageRegExp = computed(() =>
    ignoreJpeg.value ? /\.(png|gif|bmp|heic|heif)$/i : /\.(jpe?g|png|gif|bmp|heic|heif)$/i
  );

  // D&D
  listen('tauri://drag-drop', async e => {
    const inputs = (e.payload as { paths: string[] }).paths;
    const files = await collectFiles(inputs, imageRegExp.value, recursive.value);
    if (!files.length) {
      globalStore.setMessage('No images found in the dropped items.');
      return;
    }
    await convert(files);
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
    const files = await collectFiles(paths, imageRegExp.value, recursive.value);

    if (!files.length) {
      globalStore.setMessage('No images found in the selected items.');
      return;
    }
    await convert(files);
  };

  // フォルダ選択
  const convertByDirDialog = async () => {
    const picked = await open({ directory: true, recursive: true });
    if (!picked) return;
    const dir = Array.isArray(picked) ? picked[0] : picked;

    dialog.value = true;
    inProgress.value = true;
    progress.value = undefined;
    currentFile.value = 'Scanning images...';
    await nextTick();

    const files = await collectFiles(dir, imageRegExp.value, recursive.value);

    if (!files.length) {
      dialog.value = false;
      progress.value = undefined;
      inProgress.value = false;
      globalStore.setMessage('No images found in the selected folder.');
      return;
    }
    await convert(files);
  };

  // 変換処理
  const convert = async (files: string[], output: string | null = null) => {
    if (!files.length) return;
    if (progress.value) {
      globalStore.setMessage('Please wait until the current process is completed.');
      return;
    }
    dialog.value = true;
    inProgress.value = true;
    progress.value = 0;
    await nextTick();
    console.log(files);

    let i = 0;
    for (i = 0; i < files.length; i++) {
      currentFile.value = files[i];
      try {
        await invoke('convert_image', {
          input: files[i],
          output,
          quality: isLossless.value ? 100 : quality.value
        });
      } catch (error) {
        progress.value = undefined;
        dialog.value = false;
        inProgress.value = false;
        globalStore.setMessage(String(error));
        console.error('Error converting image:', error);
        return;
      }

      progress.value = Math.floor(((i + 1) / files.length) * 100);
      if (!inProgress.value) {
        progress.value = undefined;
        dialog.value = false;
        globalStore.setMessage('Interrupted.');
        return;
      }
      await nextTick();
    }

    progress.value = undefined;
    dialog.value = false;
    inProgress.value = false;
    globalStore.setMessage(`${i} images have been converted successfully.`);
  };

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
          title: 'Save As...',
          filters: [{ name: 'WebP Image', extensions: ['webp'] }]
        });
        if (savePath) {
          try {
            await invoke('convert_u8i', {
              data: buffer,
              output: savePath,
              quality: quality.value
            });
          } catch (error) {
            console.error('Error converting image:', error);
          } finally {
            globalStore.setLoading(false);
          }
          globalStore.setMessage('Image pasted and converted to WebP successfully.');
        }
      }
    }
    globalStore.setLoading(false);
  }
  usePaste(handlePaste);

  return {
    // state
    isLossless,
    quality,
    recursive,
    ignoreJpeg,
    dialog,
    inProgress,
    currentFile,
    progress,

    // methods
    convertByDialog,
    convertByDirDialog
  };
}
