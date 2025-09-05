/**
 * Rustの `WebpOptions` 構造体に対応
 */
export interface WebpOptions {
  /** 品質（0~100） */
  quality: number;
  /** ロスレス圧縮にするか */
  lossless: boolean;
}
