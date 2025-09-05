import type { BitDepth, ColorModel, AlphaColorMode } from '@/types/AvifTypes';
/**
 * Rustの `AvifOptions` 構造体に対応
 */
export interface AvifOptions {
  /** 品質 */
  quality: number;
  /** ビット進度 */
  bitDepth: BitDepth;
  /** アルファチャンネルの品質 */
  alphaQuality: number;
  /** エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。 */
  speed: number;
  /** カラーモデル */
  colorModel: ColorModel;
  /** 使用するスレッド数 (undefinedの場合は自動設定) */
  threads?: number;
  /** アルファ */
  alphaColorMode: AlphaColorMode;
}
