use super::common::{
    EncodingAnalysis, ToneMappingType, apply_tone_mapping, convert_f32_to_u8,
    get_encoding_recommendations, handle_icc_profile_embedding, log_encoding_analysis,
    provide_icc_recommendations,
};
use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use serde::{Deserialize, Serialize};
use webp::{Encoder, WebPMemory};

/// WebP format encoding options
/// quality: 0-100 (0 is lowest quality, 100 is highest quality)
/// lossless: true/false (whether to use lossless compression)
/// method: 0-6 (0 is fast, 6 is high quality)
/// autofilter: true/false (whether to use automatic filtering)
/// hint: Image hint (WebPImageHint enumeration)
/// Note: When lossless is true, quality is ignored)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebpOptions {
    pub quality: f32,
    pub lossless: bool,
    // pub method: u8,
    // pub autofilter: bool,
    // pub hint: WebPImageHint,
    // pub preset: WebPPreset,
}

/*
/// WebPの画像ヒント
/// - Default: 標準的な用途
/// - Picture: 写真やリアルな画像向け
/// - Photo: 写真向け
/// - Graph: 図やイラスト向け
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum WebPImageHint {
    Default = libwebp_sys::WEBP_HINT_DEFAULT as isize,
    Picture = libwebp_sys::WEBP_HINT_PICTURE as isize,
    Photo = libwebp_sys::WEBP_HINT_PHOTO as isize,
    Graph = libwebp_sys::WEBP_HINT_GRAPH as isize,
    Last = libwebp_sys::WEBP_HINT_LAST as isize,
}
*/

/// Encode image to WebP format with advanced content analysis
/// # Arguments
/// - `pixel_data`: Source image to encode (HighBitDepthImage)
/// - `icc_profile`: ICC profile for color management
/// - `options`: WebP encoding options (WebpOptions)
/// # Returns
/// - Success: WebP format byte data as Vec<u8>
/// - Failure: AppError
/// # Notes
/// - Uses `libwebp-sys` crate for WebP encoding. Build requires `libwebp` library installed on system
/// - Performs content analysis for optimal encoding settings
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &WebpOptions,
) -> Result<Vec<u8>, AppError> {
    println!("WebP: Starting WebP encoding process...");

    // Perform content analysis for optimal encoding
    let analysis = EncodingAnalysis::analyze(pixel_data, icc_profile.as_deref());
    log_encoding_analysis(&analysis, "WebP");
    get_encoding_recommendations(&analysis, "WebP");

    // Get image dimensions and pixel data
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(pixel_data);

    println!(
        "WebP: Image properties - {}x{}, {} channels",
        width,
        height,
        if is_rgba { 4 } else { 3 }
    );
    println!(
        "WebP: Encoding settings - Quality: {}, Lossless: {}",
        options.quality, options.lossless
    );

    // Apply tone mapping if HDR content is detected
    let processed_pixels = if analysis.tone_mapping_required {
        println!(
            "WebP: Applying tone mapping for HDR content (max luminance: {:.3})",
            analysis.max_luminance
        );
        apply_tone_mapping(&pixels_f32, is_rgba, ToneMappingType::Reinhard, 1.0)
    } else if analysis.has_wide_gamut {
        println!("WebP: Processing wide gamut content");
        pixels_f32.to_vec()
    } else {
        pixels_f32.to_vec()
    };

    // Convert f32 pixels to u8 (WebP encoders primarily work with 8-bit input)
    let pixels_u8 = convert_f32_to_u8(&processed_pixels);

    // ★ 4. RGB/RGBAに応じてエンコーダーを生成
    let encoder = if is_rgba {
        Encoder::from_rgba(&pixels_u8, width, height)
    } else {
        Encoder::from_rgb(&pixels_u8, width, height)
    };

    // ★ 5. オプションに応じてエンコード処理を呼び出し
    let webp_memory: WebPMemory = if options.lossless {
        // ロスレスエンコード
        encoder.encode_lossless()
    } else {
        // 非可逆エンコード (品質指定)
        encoder.encode(options.quality)
    };

    println!("WebP: Successfully encoded WebP data");

    // Handle ICC profile using common implementation
    let final_webp_data = handle_icc_profile_embedding(webp_memory.to_vec(), icc_profile, "WebP");

    // Provide format-specific ICC recommendations
    provide_icc_recommendations("WebP", analysis.has_wide_gamut, analysis.has_hdr_content);

    Ok(final_webp_data)
}
