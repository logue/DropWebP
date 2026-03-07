import type { BitDepth, ColorModel } from '@/types/AvifTypes';

// 型と定数を再エクスポート
export { BitDepth, ColorModel } from '@/types/AvifTypes';
export type { BitDepth as BitDepthType, ColorModel as ColorModelType } from '@/types/AvifTypes';

/**
 * AVIF最適化オプション
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
}
