/**
 * JPEG (jpegli) エンコードオプション
 *
 * jpegliは、libjxlプロジェクトに含まれる高品質なJPEGエンコーダーです。
 * 標準のJPEGエンコーダーよりも優れた圧縮率と画質を提供します。
 */
export interface JpegOptions {
  /**
   * 画質 (1-100の範囲)
   * 85以上を推奨（高品質）
   * @default 95
   */
  quality: number;

  /**
   * プログレッシブJPEG（推奨）
   * Webでの読み込みパフォーマンスが向上
   * @default true
   */
  progressive: boolean;

  /**
   * 最適化（推奨）
   * ファイルサイズをさらに削減
   * @default true
   */
  optimize: boolean;
}
