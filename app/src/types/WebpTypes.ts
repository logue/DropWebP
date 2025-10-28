export type WebPImageHint = (typeof WebPImageHint)[keyof typeof WebPImageHint];

/**
 * WebP画像ヒント
 * 画像の種類に応じてエンコーダーの最適化を制御
 */
export const WebPImageHint = {
  /** デフォルト - 標準的な用途 */
  Default: 'Default',
  /** Picture - 写真やリアルな画像向け */
  Picture: 'Picture',
  /** Photo - 写真向け（Pictureと同様） */
  Photo: 'Photo',
  /** Graph - 図やイラスト向け（シャープエッジに最適化） */
  Graph: 'Graph',
  /** Last - 最後のヒント値（内部使用） */
  Last: 'Last'
} as const;

export type WebPPreset = (typeof WebPPreset)[keyof typeof WebPPreset];

/**
 * WebPプリセット
 * 異なる画像タイプに最適化された設定プロファイル
 */
export const WebPPreset = {
  /** Default - 標準設定 */
  Default: 'Default',
  /** Picture - デジタル写真（ポートレートやインドアショット） */
  Picture: 'Picture',
  /** Photo - 写真（アウトドア、自然光）※品質+5% */
  Photo: 'Photo',
  /** Drawing - 描画やライン画像 ※品質-10%、サイズ重視 */
  Drawing: 'Drawing',
  /** Icon - アイコンやファビコン ※品質-20%、ロスレス推奨 */
  Icon: 'Icon',
  /** Text - テキストライクな画像 ※最高品質、ロスレス推奨 */
  Text: 'Text'
} as const;
