/**
 * PNG最適化オプション
 */
export interface PngOptions {
  /** Zopfliの反復回数 */
  zopfliIterations: number;
  /** ICCプロファイルを含めるか */
  embedIccProfile: boolean;
}
