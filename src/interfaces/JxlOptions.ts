import type { EncoderSpeed, ColorEncoding } from '@/types/JxlTypes';

export interface JxlOptions {
  /** ロスレス圧縮するか */
  lossless: boolean;
  /** エンコード速度（0~10） */
  speed: EncoderSpeed;
  /** 品質（0.1〜15.0） */
  quality: number;
  /** JPEG XLコンテナ形式を使用するようにエンコーダを構成する */
  useContainer: boolean;
  /** エンコーダを元のカラープロファイルを使用するように設定する。 */
  usesOriginalProfile: boolean;
  /** デコード速度を設定（0~4）値が低いほど高品質 */
  decodingSpeed: number;
  /** 出力バッファの初期サイズ  */
  initBufferSize: number;
  /** カラーエンコード方法を設定する */
  colorEncoding: ColorEncoding;
}
