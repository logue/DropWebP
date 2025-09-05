import type { AvifOptions } from './AvifOptions';
import type { WebpOptions } from './WebpOptions';

/**
 * Rustの `EncodeOptions` 構造体に対応
 */
export interface EncodeOptions {
  avif?: AvifOptions;
  webp?: WebpOptions;
}
