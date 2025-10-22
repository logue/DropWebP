import type { OutputFormat } from '@/types/SettingsTypes';

// 型と定数を再エクスポート
export { OutputFormat } from '@/types/SettingsTypes';
export type { OutputFormat as OutputFormatType } from '@/types/SettingsTypes';

/** 共通オプション */
export interface CommonOptions {
  /** Play sound when done */
  sound: boolean;
  /** Sound Volume */
  volume: number;
  /** Notify in desktop */
  notify: boolean;
  /** Output image format */
  format: OutputFormat;
  /** Overwrite original file */
  overwrite: boolean;
  /** Delete original file after conversion */
  deleteOriginal: boolean;
  /** Include subdirectories */
  recursive: boolean;
  /** Save in the same directory as the original */
  sameDirectory: boolean;
  /** Ignore JPEG files */
  ignoreJpeg: boolean;
  /** Output directory */
  outputPath: string;
}
