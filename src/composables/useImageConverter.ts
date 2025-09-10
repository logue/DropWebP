import { useSettingsStore } from '@/store';
import { toRaw } from 'vue';

import { invoke } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path';

import { useFileSystem } from './useFileSystem';

export function useImageConverter() {
  const fileSystem = useFileSystem();
  const settingsStore = useSettingsStore();

  /**利用可能な拡張子 */
  const extensions = [
    // Imageクレートのサポートする拡張子
    'avif',
    'bmp',
    'dds',
    'ff',
    'gif',
    'hdr',
    'ico',
    'jpg',
    'jpeg',
    'exr',
    'png',
    'pnm',
    'qoi',
    'tga',
    'tif',
    'tiff',
    'webp',
    // 追加対応の拡張子
    'heic',
    'heif',
    'jp2',
    'j2k'
  ];

  /**
   * 単一ファイルの変換処理
   * @param input 入力ファイルのパス
   * @param options 変換パラメータ
   */
  const convert = async (input: string, output?: string) => {
    const pathInfo = await fileSystem.pathInfo(input);
    if (!pathInfo.exists || !pathInfo.isFile) {
      return;
    }
    // 入力ファイル名
    const fileName = pathInfo.fileName;
    // 変換
    const buffer = await compress(await fileSystem.read(input));
    // 出力ファイル名を生成
    const outputFileName = `${fileName.split('.').slice(0, -1).join('.')}.${settingsStore.commonOptions.format}`;
    // 保存先
    const savePath = output
      ? await join(output, outputFileName) // 出力先を指定して保存
      : await join(pathInfo.parentDir, outputFileName); // 入力パスと同じディレクトリに保存

    // 保存処理
    await fileSystem.save(savePath, buffer);
  };

  /**
   * 圧縮処理
   * @param data 元バイナリデータ
   * @returns 圧縮済みバイナリデータ
   */
  const compress = async (data: Uint8Array): Promise<Uint8Array> => {
    // 圧縮オプション
    const options =
      settingsStore.commonOptions.format === 'avif'
        ? { avif: toRaw(settingsStore.avifOptions) }
        : { webp: toRaw(settingsStore.webpOptions) };
    try {
      // rust側のVec<8>はnumber[]型になるのでUint8Arrayに変換する
      return new Uint8Array(await invoke<number[]>('convert', { data, options }));
    } catch (e) {
      console.error(e);
      throw e;
    }
  };

  return { extensions, convert, compress };
}
