import type { AvifOptions } from './AvifOptions';
import type { JpegOptions } from './JpegOptions';
import type { JxlOptions } from './JxlOptions';
import type { PngOptions } from './PngOptions';
import type { WebpOptions } from './WebpOptions';
/**
 * Rustの `EncodeOptions` 構造体に対応
 */
export interface EncodeOptions {
  avif?: AvifOptions;
  webp?: WebpOptions;
  jxl?: JxlOptions;
  png?: PngOptions;
  jpeg?: JpegOptions;
}
