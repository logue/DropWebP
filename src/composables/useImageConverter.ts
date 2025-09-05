import { useGlobalStore, useSettingsStore } from '@/store';
import { toRaw } from 'vue';

import { invoke } from '@tauri-apps/api/core';

import { useFileSystem } from './useFileSystem';

export function useImageConverter() {
  const globalStore = useGlobalStore();
  const fileSystem = useFileSystem();
  const settingsStore = useSettingsStore();

  /**
   * 単一ファイルの変換処理
   * @param input 入力ファイルのパス
   * @param options 変換パラメータ
   */
  const convert = async (input: string, output?: string) => {
    try {
      const fileName = await fileSystem.getFileName(input);
      const buffer: Uint8Array = await invoke('convert', {
        data: await fileSystem.read(input),
        options: {
          avif: toRaw(settingsStore.avifOptions),
          webp: toRaw(settingsStore.webpOptions)
        }
      });
      const savePath = output
        ? `${output}${fileName}.${settingsStore.commonOptions.format}` // 出力先を指定して保存
        : `${await fileSystem.getDir(input)}${fileName}.${settingsStore.commonOptions.format}`; // 入力パスと同じディレクトリに保存
      await fileSystem.save(savePath, buffer);
    } catch (e) {
      console.error(e);
      globalStore.setMessage(`Failed to convert file: ${input}`);
    }
  };

  return {
    convert
  };
}
