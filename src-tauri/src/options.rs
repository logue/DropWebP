use jpegxl_rs;
use ravif;
use serde::{Deserialize, Serialize};

/// ファイルパス情報
/// file_name: ファイル名 (拡張子含む)
/// extension: 拡張子 (ドット無し)
/// parent_dir: 親ディレクトリのパス
/// exists: パスが存在するか
/// is_file: ファイルであるか
/// is_dir: ディレクトリであるか
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")] // JS側でキャメルケースになるように設定
pub struct PathInfo {
    pub(crate) file_name: Option<String>,
    pub(crate) extension: Option<String>,
    pub(crate) parent_dir: Option<String>,
    pub(crate) exists: Option<String>,
    pub(crate) is_file: Option<bool>,
    pub(crate) is_dir: Option<bool>,
}

/// 全てのエンコードオプションをまとめる親構造体
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodeOptions {
    pub avif: Option<AvifOptions>,
    pub webp: Option<WebpOptions>,
    pub jxl: Option<JxlOptions>,
}

/// AVIF形式のオプション
/// lossless: true/false (可逆圧縮を使うかどうか)
/// quality: 0-100 (0は可逆圧縮、100は最高品質)
/// bit_depth: ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten, BitDepth::Twelve)
/// alpha_quality: アルファチャンネルの品質 (0は可逆圧縮、100は最高品質)
/// speed: エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。
/// color_model: カラーモデル (ColorModel::YCbCr, ColorModel::RGB)
/// threads: 使用するスレッド数 (Noneの場合は自動設定)
/// alpha_color_mode: アルファチャネルの色モード (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// 注意: BitDepth::Autoを選択した場合、入力画像のビット深度に基づいて自動的に決定されます。
///     例えば、8ビット画像ならBitDepth::Eight、10ビット画像ならBitDepth::Tenが選択されます。
///     ただし、入力画像が8ビット以上であっても、AVIFエンコード時にBitDepth::Eightを選択することも可能です。
///     逆に、10ビット以上の画像に対してBitDepth::Eightを選択すると、情報の損失が発生する可能性があります。
///     そのため、可能な限り入力画像のビット深度に合わせた設定を推奨します。
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvifOptions {
    pub lossless: bool,
    pub quality: f32,
    pub bit_depth: BitDepth,
    pub alpha_quality: f32,
    pub speed: u8,
    pub color_model: ColorModel,
    pub threads: Option<usize>,
    pub alpha_color_mode: AlphaColorMode,
}

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

/// JPEG XL形式のオプション
/// * `lossless` - ロスレス圧縮するか
/// * `speed` - エンコード速度（0~10）値が低いほど早いが品質が劣る
/// * `quality` - 品質（0.1〜15.0）値が高いほど高品質。デフォルトは1。推奨値0.5〜3.0。（ロスレス時は無視されます）
/// * `use_container` - JPEG XLコンテナ形式を使用するようにエンコーダを構成する
/// * `uses_original_profile` - エンコーダを元のカラープロファイルを使用するように設定する。（ロスレス時は常に有効）
/// * `decoding_speed` - デコード速度を設定（0~4）。値が低いほど高品質。デフォルトは0
/// * `init_buffer_size` - 出力バッファの初期サイズ（バイト単位）32未満は32kbに切り上げ
/// * `color_encoding` - カラーエンコード方法を設定する。デフォルトはsRGB
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JxlOptions {
    pub lossless: bool,
    pub speed: EncoderSpeed,
    pub quality: f32,
    pub use_container: bool,
    pub uses_original_profile: bool,
    pub decoding_speed: i64,
    pub init_buffer_size: usize,
    pub color_encoding: ColorEncoding,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncoderSpeed {
    Lightning,
    Thunder,
    Falcon,
    Cheetah,
    Hare,
    Wombat,
    Squirrel,
    Kitten,
    Tortoise,
    Glacier,
}

impl EncoderSpeed {
    pub fn to_jxl(self) -> jpegxl_rs::encode::EncoderSpeed {
        match self {
            EncoderSpeed::Lightning => jpegxl_rs::encode::EncoderSpeed::Lightning,
            EncoderSpeed::Thunder => jpegxl_rs::encode::EncoderSpeed::Thunder,
            EncoderSpeed::Falcon => jpegxl_rs::encode::EncoderSpeed::Falcon,
            EncoderSpeed::Cheetah => jpegxl_rs::encode::EncoderSpeed::Cheetah,
            EncoderSpeed::Hare => jpegxl_rs::encode::EncoderSpeed::Hare,
            EncoderSpeed::Wombat => jpegxl_rs::encode::EncoderSpeed::Wombat,
            EncoderSpeed::Squirrel => jpegxl_rs::encode::EncoderSpeed::Squirrel,
            EncoderSpeed::Kitten => jpegxl_rs::encode::EncoderSpeed::Kitten,
            EncoderSpeed::Tortoise => jpegxl_rs::encode::EncoderSpeed::Tortoise,
            EncoderSpeed::Glacier => jpegxl_rs::encode::EncoderSpeed::Glacier,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorEncoding {
    Srgb,
    LinearSrgb,
    SrgbLuma,
    LinearSrgbLuma,
}

impl ColorEncoding {
    pub fn to_jxl(self) -> jpegxl_rs::encode::ColorEncoding {
        match self {
            ColorEncoding::Srgb => jpegxl_rs::encode::ColorEncoding::Srgb,
            ColorEncoding::LinearSrgb => jpegxl_rs::encode::ColorEncoding::LinearSrgb,
            ColorEncoding::SrgbLuma => jpegxl_rs::encode::ColorEncoding::SrgbLuma,
            ColorEncoding::LinearSrgbLuma => jpegxl_rs::encode::ColorEncoding::LinearSrgbLuma,
        }
    }
}
