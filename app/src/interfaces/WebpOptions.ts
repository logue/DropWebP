import type { WebPImageHint, WebPPreset } from '@/types/WebpTypes';

// 型と定数を再エクスポート
export { WebPImageHint } from '@/types/WebpTypes';
export type { WebPImageHint as WebPImageHintType } from '@/types/WebpTypes';

/**
 * WebPエンコーディングオプション
 *
 * Rustの `WebpOptions` 構造体に対応
 * Advanced API使用により、すべてのオプションがエンコーディングに反映されます
 */
export interface WebpOptions {
  /**
   * 品質（0~100）
   * - 0: 最低品質（最小ファイルサイズ）
   * - 100: 最高品質（最大ファイルサイズ）
   * - プリセットにより自動調整される場合があります
   * - ロスレス時は無視されます
   */
  quality: number;

  /**
   * ロスレス圧縮にするか
   * - true: 完全可逆圧縮（品質劣化なし）
   * - false: 非可逆圧縮（ファイルサイズ重視）
   * - Icon/Textプリセットでは自動的にtrueが推奨されます
   */
  lossless: boolean;

  /**
   * エンコード方法（0~6）
   * - 0: 最速（低品質）
   * - 6: 最高品質（低速）
   * - 推奨値: 4（バランス）
   */
  method: number;

  /**
   * 自動フィルタリングを使うか
   * - true: 自動的に最適なフィルタ強度を選択
   * - false: 手動設定（filterStrengthを使用）
   */
  autofilter: boolean;

  /**
   * 画像のヒント
   * 画像タイプに応じてエンコーダーの最適化を制御
   * - Default: 標準的な用途
   * - Picture/Photo: 写真向け最適化
   * - Graph: 図やイラスト向け（シャープエッジ保持）
   */
  hint: WebPImageHint;

  /**
   * プリセット
   * 画像タイプに応じた最適化プロファイル
   * - Default: 標準設定
   * - Photo: 写真（品質+5%）
   * - Picture: デジタル写真
   * - Drawing: 描画（品質-10%、サイズ重視）
   * - Icon: アイコン（品質-20%、ロスレス推奨）
   * - Text: テキスト（最高品質、ロスレス推奨）
   */
  preset: WebPPreset;

  /**
   * デブロッキングフィルタの強さ（0~100）
   * - 0: フィルタなし（シャープだが圧縮アーティファクトが目立つ）
   * - 100: 最大フィルタ（滑らかだがボケる可能性）
   * - 推奨値: 20-50（画像タイプによる）
   */
  filterStrength: number;

  /**
   * シャープネスの強さ（0~7）
   * - 0: 最もシャープ
   * - 7: 最も滑らか
   * - 推奨値: 0-4（写真）、5-7（イラスト）
   */
  filterSharpness: number;

  /**
   * 空間ノイズシェーピング（SNS）の強度（0~100）
   * - 0: SNSなし
   * - 100: 最大SNS（ノイズ削減、ファイルサイズ縮小）
   * - 推奨値: 50-80（写真）、0-30（イラスト）
   */
  snsStrength: number;

  /**
   * 透過色（アルファチャンネル）の品質（0~100）
   * - 0: 最低品質（最小サイズ）
   * - 100: 最高品質（ロスレス相当）
   * - RGBA画像のみ有効
   * - 推奨値: 80-100
   */
  alphaQuality: number;
}
