import type { WebPImageHint } from '@/types/WebpTypes';

// 型と定数を再エクスポート
export { WebPImageHint } from '@/types/WebpTypes';
export type { WebPImageHint as WebPImageHintType } from '@/types/WebpTypes';

/**
 * Rustの `WebpOptions` 構造体に対応
 */
export interface WebpOptions {
  /** 品質（0~100） */
  quality: number;
  /** ロスレス圧縮にするか */
  lossless: boolean;
  /** エンコード方法（0~6） */
  method: number;
  /** 自動フィルタリングを使うか */
  autofilter: boolean;
  /** 画像のヒント */
  hint: WebPImageHint;
}
