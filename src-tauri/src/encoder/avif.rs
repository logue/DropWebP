use crate::error::AppError;
use image::DynamicImage;
use imgref::Img;
use ravif::{AlphaColorMode, BitDepth, ColorModel, Encoder};
use rgb::{RGB8, RGBA8};

/// DynamicImage を AVIF 形式のバイトデータに変換する (raif クレート使用)
///
/// # 引数
/// * `img` - 変換元のDynamicImage
/// * `quality` - 品質 (1-100)。値が高いほど最高品質。
/// * `bit_depth` - ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten)
/// * `alpha_quality` - アルファチャンネルの品質（1-100）。値が高いほど最高品質。
/// * `speed` - エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。
/// * `color_model` - カラーモデル (ColorModel::YCbCr, ColorModel::RGB)
/// * `threads` - 使用するスレッド数 (Noneの場合は自動設定)
/// * `alpha_color_mode` - アルファチャネルの色モード (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// # 戻り値
/// * 成功した場合はAVIF形式のバイト列をVec<u8>として返します。
/// * 失敗した場合はAppErrorを返します。
/// # 注意
/// * `ravif` クレートを使用してAVIFエンコードを行います。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
pub fn encode(
    img: &DynamicImage,
    quality: f32,
    bit_depth: BitDepth,
    alpha_quality: f32,
    speed: u8,
    color_model: ColorModel,
    threads: Option<usize>,
    alpha_color_mode: AlphaColorMode,
) -> Result<Vec<u8>, AppError> {
    // エンコーダーの設定は先に済ませておく
    let encoder = Encoder::new()
        .with_quality(quality)
        .with_bit_depth(bit_depth)
        .with_internal_color_model(color_model)
        .with_num_threads(threads)
        .with_alpha_color_mode(alpha_color_mode)
        .with_speed(speed)
        .with_alpha_quality(alpha_quality);

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
    println!("Finished encoding AVIF.");

    Ok(encoded_avif.avif_file)
}
