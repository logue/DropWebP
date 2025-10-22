import type { PngInterlace, PngFilter } from '@/types/PngTypes';

// 型と定数を再エクスポート
export { PngFilter, PngInterlace } from '@/types/PngTypes';
export type {
  PngFilter as PngFilterType,
  PngInterlace as PngInterlaceType
} from '@/types/PngTypes';

/**
 * PNG最適化オプション
 */
export interface PngOptions {
  /** Zopfliの反復回数（15-255、高いほど高圧縮だが遅い） */
  zopfliIterations: number;
  /** ICCプロファイルを含めるか */
  embedIccProfile: boolean;
  /** ビット深度削減を有効にする（8bit→1/2/4bit） */
  bitDepthReduction: boolean;
  /** カラータイプ削減を有効にする（RGBA→RGB、RGB→Grayscaleなど） */
  colorTypeReduction: boolean;
  /** パレット削減を有効にする */
  paletteReduction: boolean;
  /** インターレース設定 */
  interlace: PngInterlace;
  /** フィルター戦略 */
  filter: PngFilter;
}
