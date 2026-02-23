# PLAN 4.0: VRM(glTF) 画像再圧縮対応

## 概要

Drop Compress Image v4.0 では、VRM(glTF) ファイルに含まれる**画像アセットのみ**を対象に再圧縮機能を追加する。
モデル構造（メッシュ、マテリアル定義、ボーン、アニメーション等）は変更しない。

## 目的

- VRM/glTF の見た目を維持しながら画像データ容量を削減する
- テクスチャ用途ごとに適した圧縮・減色を行う
- 既存の画像変換パイプライン（Rust エンコーダ群）を活用しつつ、VRM 専用フローを追加する

## スコープ

### 基本方針（長期）

- 本ツールは画像データ圧縮に特化し、今後のバージョンでもテクスチャデータ以外の要素は扱わない
- 対象は「画像の再圧縮・減色・解像度変更」のみとし、モデル編集機能は追加しない

### 対象

- `.vrm` / `.gltf`（必要に応じて `.glb` を同等扱い）に内包・参照される画像
- PNG/JPEG などの一般的テクスチャ
- 主要テクスチャ種別（ベースカラー、ORM、法線、発光）

### 非対象

- モデル形状、ボーン、スキン、アニメーション、メタ情報の意味変更
- マテリアルパラメータの再設計
- リグやレンダリングロジックそのものの改変
- テクスチャ以外のアセット最適化（メッシュ簡略化、アニメーション圧縮、骨構造変更など）

## 必須要件（ユーザー要件反映）

1. 原則 `oxipng` の **Zopfli** で圧縮する
2. モデルは触らない（画像データのみ更新）
3. ORM マップは、R/G/B チャンネルを一度分離し、各チャンネルでパレット減色後に再結合する
4. 発光マップ・ノーマルマップもパレット減色対象にする
5. ベースカラーは `jpegli` 圧縮を選択可能にする（オプション）
6. テクスチャ解像度を下げるオプションを提供する
7. 画像縮小時のリサンプリング手法（例: Lanczos）を選択可能にする

## 実装方針

### 1) VRM/glTF パイプライン

- 入力: VRM/glTF を解析し画像参照を収集
- 処理: テクスチャ種別ごとの再圧縮
- 出力: 画像データのみ差し替えて再パック
- 制約: JSON・バイナリ構造における画像参照整合性は維持

### 2) テクスチャ種別ごとの戦略

- **共通既定**: PNG 系は `oxipng + zopfli`
- **ORM**:
  - R(occlusion) / G(roughness) / B(metallic) を分離
  - 各チャンネル単位でパレット減色
  - 再結合して ORM として保存
- **ノーマル**:
  - 法線情報劣化を抑えるため、減色強度は控えめな既定値
  - 必要なら高品質モードを優先
- **発光**:
  - パレット減色を適用（グラデーション破綻の検出を考慮）
- **ベースカラー**:
  - 既定はロスレス/PNG 系最適化
  - オプションで `jpegli`（品質指定あり）へ変換可能

### 3) 解像度ダウンサンプル

- オプションで長辺または倍率指定による縮小を実施
- リサンプリング方式を選択可能にする（既定は Lanczos）
  - 例: Nearest / Triangle / Catmull-Rom / Gaussian / Lanczos3
- UV/モデル側には手を入れない（テクスチャ画像のみリサイズ）
- 最小サイズ制限を設け、過度な劣化を防ぐ

## UI/設定項目案

- VRM最適化モード: ON/OFF
- ベースカラー `jpegli` 化: ON/OFF
- `jpegli` 品質（例: 70–100）
- 解像度縮小:
  - 無効
  - 1/2, 1/4
  - 長辺上限指定（例: 2048, 1024）
- 縮小アルゴリズム:
  - Lanczos（既定）
  - Triangle
  - Catmull-Rom
  - Nearest（高速）
- ORM/ノーマル/発光の減色強度（プリセット: 安全/標準/強）

## enum設計案（Rust / TypeScript）

### Rust（`app/src-tauri/src/options.rs` 想定）

```rust
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResizeFilter {
  Nearest,
  Triangle,
  CatmullRom,
  Gaussian,
  Lanczos3,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResizeMode {
  Disabled,
  ScaleHalf,
  ScaleQuarter,
  MaxEdge,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuantizePreset {
  Safe,
  Standard,
  Aggressive,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextureRole {
  BaseColor,
  Normal,
  Emissive,
  Orm,
  Other,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmImageOptimizeOptions {
  pub enable_vrm_optimization: bool,
  pub resize_mode: ResizeMode,
  pub max_edge: Option<u32>,
  pub resize_filter: ResizeFilter,
  pub quantize_preset: QuantizePreset,
  pub enable_basecolor_jpegli: bool,
  pub jpegli_quality: Option<u8>,
}
```

### TypeScript（`app/src/interfaces/` 想定）

```ts
export type ResizeFilter = 'nearest' | 'triangle' | 'catmullRom' | 'gaussian' | 'lanczos3';

export type ResizeMode = 'disabled' | 'scaleHalf' | 'scaleQuarter' | 'maxEdge';

export type QuantizePreset = 'safe' | 'standard' | 'aggressive';

export type TextureRole = 'baseColor' | 'normal' | 'emissive' | 'orm' | 'other';

export interface VrmImageOptimizeOptions {
  enableVrmOptimization: boolean;
  resizeMode: ResizeMode;
  maxEdge?: number;
  resizeFilter: ResizeFilter;
  quantizePreset: QuantizePreset;
  enableBasecolorJpegli: boolean;
  jpegliQuality?: number;
}
```

### 設計メモ

- Rust/TypeScript 間の命名は `camelCase` で統一し、Tauri invoke のシリアライズ差異を減らす
- `TextureRole` は自動判定（gltf 参照解析）結果を内部で持つ用途
- `resize_filter` の既定値は `Lanczos3`、`quantize_preset` の既定値は `Standard`
- `jpegli_quality` は `enable_basecolor_jpegli = true` のときのみ有効

## 互換性・品質方針

- デフォルトは見た目破綻を避ける**保守的設定**
- 画質優先/容量優先のプリセットを用意
- 変換前後で参照切れ・読み込み不可が発生しないことを最優先

## 開発フェーズ

### Phase 1: 基盤

- VRM/glTF 画像抽出・差し戻し機構
- 既存エンコーダ連携（oxipng/jpegli）

### Phase 2: 種別最適化

- ORM 分離→減色→再結合
- ノーマル/発光の減色処理
- ベースカラー `jpegli` オプション

### Phase 3: 解像度オプション

- ダウンサンプル実装
- リサンプリング方式選択実装（Lanczos ほか）
- UI 設定連携

### Phase 4: 検証

- 代表 VRM アセットでの見た目/容量/速度比較
- 失敗時フォールバック（元画像維持）

## フェーズ別チェックリスト

### Phase 1: 基盤

- [ ] VRM/glTF 画像抽出（内包/外部参照）を実装
- [ ] 画像差し替え後の再パック処理を実装
- [ ] `VrmImageOptimizeOptions` を Tauri コマンドに受け渡し
- [ ] 既存 `oxipng/jpegli` ルートと統合（非VRM処理を壊さない）

### Phase 2: 種別最適化

- [ ] `TextureRole` 判定（baseColor/normal/emissive/orm）を実装
- [ ] ORM の RGB 分離→各チャンネル減色→再結合を実装
- [ ] ノーマル/発光の減色処理を実装（プリセット対応）
- [ ] ベースカラー `jpegli` オプション処理を実装

### Phase 3: 解像度オプション

- [ ] `ResizeMode`（無効・1/2・1/4・長辺上限）を実装
- [ ] `ResizeFilter`（Lanczos3 等）選択を実装
- [ ] 最小サイズ制限と境界値バリデーションを実装
- [ ] UI 設定（Store/画面）とバックエンド設定を接続

### Phase 4: 検証

- [ ] 主要サンプルで変換成功率を確認
- [ ] 容量削減率・処理時間・見た目比較を計測
- [ ] Unity/Three.js/VRM Viewer で読込確認
- [ ] 失敗時フォールバック（元画像維持）を確認
- [ ] 既存変換機能の回帰テストを実施

## 受け入れ条件（Doneの定義）

- VRM/glTF のモデルデータを変更せず、画像のみ再圧縮できる
- テクスチャデータ以外（メッシュ/ボーン/アニメーション/ロジック）に変更が入らない
- 既定で `oxipng + zopfli` が適用される
- ORM 分離減色再結合が機能する
- ノーマル/発光のパレット減色が機能する
- ベースカラーの `jpegli` オプションが機能する
- 解像度縮小オプションが機能する
- 縮小アルゴリズム選択（Lanczos など）が機能する
- 変換後ファイルが主要ビューアで読み込み可能

## リスクと対策

- **ノーマル破綻**: 減色強度を抑えた既定値＋無効化オプション
- **ORM精度低下**: チャンネル別最適化で影響を局所化
- **互換性問題**: 主要ランタイム（Unity/Three.js/VRM Viewer）で回帰確認
- **処理時間増加**: 高負荷処理はオプション化し、プリセットで制御

## 備考

- v4.0 の主眼は「VRMの見た目を壊さず画像容量を削減する」こと。
- モデル編集機能は v4.0 スコープ外とする。
