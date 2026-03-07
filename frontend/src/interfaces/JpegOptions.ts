/**
 * JPEG (jpegli) エンコードオプション
 *
 * jpegliは、libjxlプロジェクトに含まれる高品質なJPEGエンコーダーです。
 * 標準のJPEGエンコーダーよりも優れた圧縮率と画質を提供します。
 *
 * ## Ultra HDR サポート
 * `ultraHdr`オプションを有効にすると、HDRメタデータを持つ画像をUltra HDR (JPEG-R)形式で
 * エンコードします。これにはGainmapが含まれ、HDR対応ディスプレイで高ダイナミックレンジを
 * 表現できます。Android 14+やiOS 17+などのモダンOSでサポートされています。
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

  /**
   * Ultra HDR (JPEG-R with gainmap) エンコード
   * HDRメタデータを持つ画像をUltra HDR形式でエンコード
   * 注: HDRディスプレイで高ダイナミックレンジを表現可能
   * @default false
   */
  ultraHdr: boolean;

  /**
   * Ultra HDR Gainmapの画質 (1-100の範囲)
   * ultraHdrがtrueの場合のみ有効
   * @default 85
   */
  gainmapQuality: number;
}
