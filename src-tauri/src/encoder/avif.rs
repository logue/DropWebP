use crate::error::AppError;
use image::DynamicImage;
use imgref::Img;
use ravif::Encoder;
use rgb::{RGB8, RGBA8};
use serde::{Deserialize, Serialize};

/// AVIF形式のオプション
/// quality: 0-100 (01~100。値が高いほど高品質)
/// bit_depth: ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten, BitDepth::Twelve)
/// alpha_quality: アルファチャンネルの品質 (1~100。値が高いほど高品質)
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
    pub quality: f32,
    pub bit_depth: BitDepth,
    pub alpha_quality: f32,
    pub speed: u8,
    pub color_model: ColorModel,
    pub threads: Option<usize>,
    pub alpha_color_mode: AlphaColorMode,
}

/// ビット深度の列挙型
/// - Auto: 入力画像のビット深度に基づいて自動的に決定
/// - Eight: 8ビット
/// - Ten: 10ビット
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Auto,
    Eight,
    Ten,
}

impl BitDepth {
    pub fn to_ravif(self) -> ravif::BitDepth {
        match self {
            BitDepth::Auto => ravif::BitDepth::Auto,
            BitDepth::Eight => ravif::BitDepth::Eight,
            BitDepth::Ten => ravif::BitDepth::Ten,
        }
    }
}

/// カラーモデルの列挙型
/// - YCbCr: YCbCrカラーモデル
/// - RGB: RGBカラーモデル
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    YCbCr,
    RGB,
}

impl ColorModel {
    pub fn to_ravif(self) -> ravif::ColorModel {
        match self {
            ColorModel::YCbCr => ravif::ColorModel::YCbCr,
            ColorModel::RGB => ravif::ColorModel::RGB,
        }
    }
}

/// アルファチャネルの色モードの列挙型
/// - UnassociatedDirty: 非関連アルファ（未クリーン）
/// - UnassociatedClean: 非関連アルファ（クリーン）
/// - Premultiplied: 乗算済みアルファ
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

/// DynamicImage を AVIF 形式のバイトデータに変換する (raif クレート使用)
///
/// # 引数
/// * `img` - 変換元のDynamicImage
/// * `options` - AVIFエンコードオプション
/// # 戻り値
/// * 成功した場合はAVIF形式のバイト列をVec<u8>として返します。
/// * 失敗した場合はAppErrorを返します。
/// # 注意
/// * `ravif` クレートを使用してAVIFエンコードを行います。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
pub fn encode(img: &DynamicImage, options: &AvifOptions) -> Result<Vec<u8>, AppError> {
    // エンコーダーの設定は先に済ませておく
    let encoder = Encoder::new()
        .with_quality(options.quality)
        .with_bit_depth(options.bit_depth.to_ravif())
        .with_internal_color_model(options.color_model.to_ravif())
        .with_num_threads(options.threads)
        .with_alpha_color_mode(options.alpha_color_mode.to_ravif())
        .with_speed(options.speed)
        .with_alpha_quality(options.alpha_quality);

    // DynamicImageの具体的な型でマッチングして処理を分岐
    let encoded_avif = match img {
        // --- RGB8形式の場合 ---
        DynamicImage::ImageRgb8(rgb_image) => {
            println!("Optimized path: Encoding as RGB...");
            let width = rgb_image.width() as usize;
            let height = rgb_image.height() as usize;

            // &[u8] を &[RGB8] に変換
            let pixels_rgb8: &[RGB8] = bytemuck::cast_slice(rgb_image.as_raw());
            let image_view = Img::new(pixels_rgb8, width, height);

            // encode_rgb を使用
            encoder.encode_rgb(image_view).map_err(AppError::Ravif)?
        }
        // --- RGBA8形式の場合 ---
        DynamicImage::ImageRgba8(rgba_image) => {
            println!("Standard path: Encoding as RGBA...");
            let width = rgba_image.width() as usize;
            let height = rgba_image.height() as usize;

            // &[u8] を &[RGBA8] に変換
            let pixels_rgba8: &[RGBA8] = bytemuck::cast_slice(rgba_image.as_raw());
            let image_view = Img::new(pixels_rgba8, width, height);

            // encode_rgba を使用
            encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
        }
        // --- その他の形式の場合 (Luma8, Bgr8など) ---
        // 汎用的なRGBA8に変換してから処理する（フォールバック）
        _ => {
            println!("Fallback path: Converting to RGBA then encoding...");
            let rgba_image = img.to_rgba8();
            let width = rgba_image.width() as usize;
            let height = rgba_image.height() as usize;

            let pixels_rgba8: &[RGBA8] = bytemuck::cast_slice(rgba_image.as_raw());
            let image_view = Img::new(pixels_rgba8, width, height);

            encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
        }
    };
    println!(
        "Finished encoding AVIF. (Color byte size: {}/ Alpha byte size: {})",
        encoded_avif.color_byte_size, encoded_avif.alpha_byte_size
    );

    Ok(encoded_avif.avif_file)
}
