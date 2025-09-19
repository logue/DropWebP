use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use imgref::Img;
use ravif::{EncodedImage, Encoder};
use serde::{Deserialize, Serialize};

/// AVIF形式のオプション
/// quality: 0-100 (01~100。値が高いほど高品質)
/// bit_depth: ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten)
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
pub fn encode(img: &HighBitDepthImage, options: &AvifOptions) -> Result<Vec<u8>, AppError> {
    // ★ 1. 先に画像サイズとピクセルデータを取得
    let (width, height) = match img {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(img);

    let encoded_avif: EncodedImage = match options.bit_depth.to_ravif() {
        ravif::BitDepth::Eight => {
            // ★ 8bit用のエンコーダーをここで生成
            let encoder = Encoder::new()
                .with_quality(options.quality)
                .with_bit_depth(ravif::BitDepth::Eight) // 明示的に指定
                .with_internal_color_model(options.color_model.to_ravif())
                .with_num_threads(options.threads)
                .with_alpha_color_mode(options.alpha_color_mode.to_ravif())
                .with_speed(options.speed)
                .with_alpha_quality(options.alpha_quality);

            let pixels_u8: Vec<u8> = pixels_f32
                .iter()
                .map(|&p| (p * 255.0).round().clamp(0.0, 255.0) as u8)
                .collect();

            if is_rgba {
                let image_view = Img::new(
                    bytemuck::cast_slice(&pixels_u8),
                    width as usize,
                    height as usize,
                );
                encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
            } else {
                let image_view = Img::new(
                    bytemuck::cast_slice(&pixels_u8),
                    width as usize,
                    height as usize,
                );
                encoder.encode_rgb(image_view).map_err(AppError::Ravif)?
            }
        }
        ravif::BitDepth::Ten | ravif::BitDepth::Auto  /* | ravif::BitDepth::Twelve  */=> {
            // ★ 10bit以上用のエンコーダーをここで生成
            let encoder = Encoder::new()
                .with_quality(options.quality)
                .with_bit_depth(ravif::BitDepth::Ten) // 明示的に指定 (AutoやTwelveの場合もravifがよしなにしてくれる)
                .with_internal_color_model(options.color_model.to_ravif())
                .with_num_threads(options.threads)
                .with_alpha_color_mode(options.alpha_color_mode.to_ravif())
                .with_speed(options.speed)
                .with_alpha_quality(options.alpha_quality);

            let pixels_u16: Vec<u16> = pixels_f32
                .iter()
                .map(|&p| (p * 65535.0).round().clamp(0.0, 65535.0) as u16)
                .collect();

            if is_rgba {
                // こちらは u16 のデータをそのまま渡す
                let image_view = Img::new(
                    bytemuck::cast_slice(&pixels_u16),
                    width as usize,
                    height as usize,
                );
                encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
            } else {
                let image_view = Img::new(
                    bytemuck::cast_slice(&pixels_u16),
                    width as usize,
                    height as usize,
                );
                encoder.encode_rgb(image_view).map_err(AppError::Ravif)?
            }
        }
    };

    Ok(encoded_avif.avif_file)
}
