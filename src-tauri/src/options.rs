use ravif;
use serde::{Deserialize, Serialize}; // 変換先の型にアクセスするために必要

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Auto,
    Eight,
    Ten,
}

// BitDepthに変換メソッドを実装
impl BitDepth {
    pub fn to_ravif(self) -> ravif::BitDepth {
        match self {
            BitDepth::Auto => ravif::BitDepth::Auto,
            BitDepth::Eight => ravif::BitDepth::Eight,
            BitDepth::Ten => ravif::BitDepth::Ten,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    YCbCr,
    RGB,
}

// ColorModelに変換メソッドを実装
impl ColorModel {
    pub fn to_ravif(self) -> ravif::ColorModel {
        match self {
            ColorModel::YCbCr => ravif::ColorModel::YCbCr,
            ColorModel::RGB => ravif::ColorModel::RGB,
        }
    }
}

// ...AlphaColorModeも同様に実装...
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaColorMode {
    UnassociatedDirty,
    UnassociatedClean,
    Premultiplied,
}

impl AlphaColorMode {
    pub fn to_ravif(self) -> ravif::AlphaColorMode {
        match self {
            AlphaColorMode::UnassociatedDirty => ravif::AlphaColorMode::UnassociatedDirty,
            AlphaColorMode::UnassociatedClean => ravif::AlphaColorMode::UnassociatedClean,
            AlphaColorMode::Premultiplied => ravif::AlphaColorMode::Premultiplied,
        }
    }
}

/// AVIF形式のオプション
/// quality: 0-100 (0は可逆圧縮、100は最高品質)
/// bit_depth: ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten, BitDepth::Twelve)
/// alpha_quality: アルファチャンネルの品質 (0-100)
/// speed: エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。
/// color_model: カラーモデル (ColorModel::YCbCr, ColorModel::RGB)
/// threads: 使用するスレッド数 (Noneの場合は自動設定)
/// alpha_color_mode: アルファチャネルの色モード (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// 注意: BitDepth::Autoを選択した場合、入力画像のビット深度に基づいて自動的に決定されます。
///     例えば、8ビット画像ならBitDepth::Eight、10ビット画像ならBitDepth::Tenが選択されます。
///     ただし、入力画像が8ビット以上であっても、AVIFエンコード時にBitDepth::Eightを選択することも可能です。
///     逆に、10ビット以上の画像に対してBitDepth::Eightを選択すると、情報の損失が発生する可能性があります。
///    そのため、可能な限り入力画像のビット深度に合わせた設定を推奨します。
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvifOptions {
    pub quality: f32,
    pub bit_depth: BitDepth,
    pub alpha_quality: f32,
    pub speed: u8,
    pub color_model: ColorModel,
    pub threads: Option<usize>,
    pub alpha_color_mode: AlphaColorMode,
}

/// WebP形式のオプション
/// quality: 0-100 (0は最低品質、100は最高品質)
/// lossless: true/false (可逆圧縮を使うかどうか
/// 注意: losslessがtrueの場合、qualityは無視される)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebpOptions {
    pub quality: f32,
    pub lossless: bool,
}

/// 全てのエンコードオプションをまとめる親構造体
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodeOptions {
    pub avif: Option<AvifOptions>,
    pub webp: Option<WebpOptions>,
    // 将来的にJPEG EXRのオプションもここに追加できる
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")] // JS側でキャメルケースになるように設定
pub struct PathInfo {
    pub(crate) file_name: Option<String>,
    pub(crate) extension: Option<String>,
    pub(crate) parent_dir: Option<String>,
}
