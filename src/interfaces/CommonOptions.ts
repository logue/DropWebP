export interface CommonOptions {
  /** Output image format */
  format: 'avif' | 'webp';
  /** Overwrite original file */
  overwrite: boolean;
  /** Delete original file after conversion */
  deleteOriginal: boolean;
  /** Include subdirectories */
  recursive: boolean;
  /** Save in the same directory as the original */
  sameDirectory: boolean;
  /** Ignore JPEG files */
  ignoreJpeg: boolean;
  /** Output directory */
  outputPath: string;
}
