import { invoke } from '@tauri-apps/api/core';

import type ConversionOptions from '@/interfaces/ConersionOptions';

export function useImageConverter() {
  /**
   * 画像を変換するコア機能
   * @param format - 'webp' | 'avif' などの出力フォーマット
   * @param inputPath - 入力ファイルパス
   * @param outputPath - 出力ファイルパス (nullの場合は同階層)
   * @param isOverwrite - 上書きフラグ
   * @param options - 変換オプション
   */
  const convert = async (
    _format: 'webp' | 'avif',
    inputPath: string,
    outputPath: string | null,
    isOverwrite: boolean,
    options: ConversionOptions
  ) => {
    // Rust側もフォーマットを受け取れるように修正が必要
    await invoke('convert_image', {
      // _format, // TODO: バックエンドにフォーマットを伝える
      input: inputPath,
      output: outputPath,
      overwrite: isOverwrite,
      quality: options.quality
    });
  };

  // ペースト用の処理も同様に
  const convertFromBuffer = async (
    _format: 'webp' | 'avif',
    data: Uint8Array,
    outputPath: string,
    options: ConversionOptions
  ) => {
    await invoke('convert_u8i', {
      // _format, // TODO: バックエンドにフォーマットを伝える
      data,
      output: outputPath,
      quality: options.quality
    });
  };

  return {
    convert,
    convertFromBuffer
  };
}
