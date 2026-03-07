/**
 * PNG最適化オプション
 */
export interface PngOptions {
  /** 最適化レベル (0-6、0=最速/最小圧縮、6=最遅/最大圧縮) */
  optimizationLevel: number;
  /** Zopfli圧縮を使用（遅いが高圧縮） */
  useZopfli: boolean;
  /** メタデータを削除 */
  stripMetadata: boolean;
  /** ビット深度削減を有効にする（8bit→1/2/4bit） */
  bitDepthReduction: boolean;
  /** カラータイプ削減を有効にする（RGBA→RGB、RGB→Grayscaleなど） */
  colorTypeReduction: boolean;
  /** パレット削減を有効にする */
  paletteReduction: boolean;
  /** グレースケール変換を試行 */
  grayscaleReduction: boolean;
  /** インターレース設定 (null=変更なし, true=有効, false=無効) */
  interlace: boolean | null;
  /** 透明ピクセルの最適化 */
  optimizeAlpha: boolean;
  /** 高速評価モード */
  fastEvaluation: boolean;
  /** 16ビットを強制的に8ビットにスケール */
  scale16: boolean;
}
