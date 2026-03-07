export type OutputFormat = (typeof OutputFormat)[keyof typeof OutputFormat];

/** 出力フォーマット */
export const OutputFormat = {
  /** WebP形式 */
  WebP: 'webp',
  /** AVIF形式 */
  AVIF: 'avif',
  /** JPEG XL形式 */
  JXL: 'jxl',
  /** PNG形式 */
  PNG: 'png',
  /** JPEG形式 */
  JPEG: 'jpeg'
} as const;
