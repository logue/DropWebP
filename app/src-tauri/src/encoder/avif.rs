use super::common::{
    EncodingAnalysis, ToneMappingType, apply_tone_mapping, convert_f32_to_u8,
    get_encoding_recommendations, handle_icc_profile_embedding, log_encoding_analysis,
    provide_icc_recommendations,
};
use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use imgref::Img;
use ravif::{EncodedImage, Encoder};
use serde::{Deserialize, Serialize};

/// AVIF format encoding options
/// quality: 0-100 (higher values mean better quality)
/// bit_depth: Bit depth (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten)
/// alpha_quality: Alpha channel quality (1-100, higher values mean better quality)
/// speed: Encoding speed (0-10). 0 is highest quality but slowest, 10 is fastest
/// color_model: Color model (ColorModel::YCbCr, ColorModel::RGB)
/// threads: Number of threads to use (None for automatic)
/// alpha_color_mode: Alpha channel color mode (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// Note: When BitDepth::Auto is selected, bit depth is automatically determined based on input image.
///     For example, 8-bit images will use BitDepth::Eight, 10-bit images will use BitDepth::Ten.
///     However, it's possible to choose BitDepth::Eight even for images with higher bit depth.
///     Conversely, choosing BitDepth::Eight for 10-bit+ images may result in information loss.
///     Therefore, it's recommended to match the input image's bit depth whenever possible.
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

/// Bit depth enumeration
/// - Auto: Automatically determined based on input image bit depth
/// - Eight: 8-bit
/// - Ten: 10-bit
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

/// Color model enumeration
/// - YCbCr: YCbCr color model
/// - RGB: RGB color model
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

/// Alpha channel color mode enumeration
/// - UnassociatedDirty: Unassociated alpha (dirty)
/// - UnassociatedClean: Unassociated alpha (clean)
/// - Premultiplied: Premultiplied alpha
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

/// Encode HighBitDepthImage to AVIF format using ravif crate with advanced analysis
///
/// # Arguments
/// * `pixel_data` - Source HighBitDepthImage to encode
/// * `icc_profile` - ICC profile for color management (if provided, embedding will be attempted)
/// * `options` - AVIF encoding options
/// # Returns
/// * Success: AVIF format byte data as Vec<u8>
/// * Failure: AppError
/// # Notes
/// * Uses `ravif` crate for AVIF encoding. Build requires `libavif` library installed on system
/// * When ICC profile is provided, color consistency is maintained through embedding (limited by ravif crate capabilities)
/// * Advanced content analysis is performed to optimize encoding settings
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &AvifOptions,
) -> Result<Vec<u8>, AppError> {
    println!("AVIF: Starting AVIF encoding process...");

    // Perform content analysis for optimal encoding
    let analysis = EncodingAnalysis::analyze(pixel_data, icc_profile.as_deref());
    log_encoding_analysis(&analysis, "AVIF");
    get_encoding_recommendations(&analysis, "AVIF");

    // Get image dimensions and pixel data
    let (width, height) = match &pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(&pixel_data);

    println!(
        "AVIF: Image properties - {}x{}, {} channels",
        width,
        height,
        if is_rgba { 4 } else { 3 }
    );
    println!(
        "AVIF: Encoding settings - Quality: {}, Bit depth: {:?}, Color model: {:?}",
        options.quality, options.bit_depth, options.color_model
    );

    // ピクセルデータの整合性チェック
    let expected_len = (width * height * if is_rgba { 4 } else { 3 }) as usize;
    if pixels_f32.len() != expected_len {
        return Err(AppError::Encode(format!(
            "AVIF pixel data length mismatch: expected {}, got {}. Width={}, Height={}, RGBA={}",
            expected_len,
            pixels_f32.len(),
            width,
            height,
            is_rgba
        )));
    }

    let encoded_avif: EncodedImage = {
        // ★ エンコーダーを生成（すべてのピクセルを8ビットとして処理）
        let encoder = Encoder::new()
            .with_quality(options.quality)
            .with_bit_depth(options.bit_depth.to_ravif()) // 設定に従ってビット深度を決定
            .with_internal_color_model(options.color_model.to_ravif())
            .with_num_threads(options.threads)
            .with_alpha_color_mode(options.alpha_color_mode.to_ravif())
            .with_speed(options.speed)
            .with_alpha_quality(options.alpha_quality);

        // Apply tone mapping if HDR content is detected
        let processed_pixels = if analysis.tone_mapping_required {
            println!(
                "AVIF: Applying tone mapping for HDR content (max luminance: {:.3})",
                analysis.max_luminance
            );
            apply_tone_mapping(&pixels_f32, is_rgba, ToneMappingType::Reinhard, 1.0)
        } else if analysis.has_wide_gamut {
            println!("AVIF: Processing wide gamut content");
            pixels_f32.to_vec()
        } else {
            pixels_f32.to_vec()
        };

        // Convert f32 pixels to u8 with proper clamping
        let pixels_u8 = convert_f32_to_u8(&processed_pixels);

        if is_rgba {
            // RGBA: Vec<u8> を RGBA<u8> のスライスに変換
            use rgb::FromSlice;
            let rgba_pixels = pixels_u8.as_rgba();
            let image_view = Img::new(rgba_pixels, width as usize, height as usize);
            encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
        } else {
            // RGB: Vec<u8> を RGB<u8> のスライスに変換
            use rgb::FromSlice;
            let rgb_pixels = pixels_u8.as_rgb();
            let image_view = Img::new(rgb_pixels, width as usize, height as usize);
            encoder.encode_rgb(image_view).map_err(AppError::Ravif)?
        }
    };

    // Handle ICC profile using common implementation
    let final_avif_data = handle_icc_profile_embedding(encoded_avif.avif_file, icc_profile, "AVIF");

    // Provide format-specific ICC recommendations
    provide_icc_recommendations("AVIF", analysis.has_wide_gamut, analysis.has_hdr_content);

    // Additional AVIF-specific recommendations
    if analysis.is_hdr_or_wide_gamut {
        if options.color_model != ColorModel::RGB {
            println!(
                "AVIF: Recommendation - Use ColorModel::RGB for better ICC profile compatibility"
            );
        }

        if matches!(options.bit_depth, BitDepth::Eight) {
            println!(
                "AVIF: Recommendation - Use BitDepth::Ten or higher to preserve wide gamut content"
            );
        }
    }

    Ok(final_avif_data)
}

/// AVIFファイルサイズを推定
pub fn estimate_size(img: &HighBitDepthImage, options: &AvifOptions) -> usize {
    let (width, height) = match img {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };

    let channels = match img {
        HighBitDepthImage::Rgb(_) => 3,
        HighBitDepthImage::Rgba(_) => 4,
        HighBitDepthImage::Argb(_) => 4,
    };

    let uncompressed_size = (width * height * channels) as usize;

    // AVIFの圧縮率推定（非常に効率的な圧縮）
    let quality_factor = options.quality / 100.0;
    let compression_ratio = 0.05 + (quality_factor * 0.15); // 5%-20%の範囲

    (uncompressed_size as f64 * compression_ratio as f64) as usize
}
