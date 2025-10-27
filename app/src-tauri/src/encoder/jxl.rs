use super::common::{
    EncodingAnalysis, get_encoding_recommendations, log_encoding_analysis,
    provide_icc_recommendations,
};
use crate::{
    encoder::{HighBitDepthImage, extract_pixel_data},
    error::AppError,
};
use jpegxl_rs::encode::{EncoderFrame, EncoderResult, EncoderSpeed::*, encoder_builder};
use serde::{Deserialize, Serialize};

/// Image type classification based on pixel data and ICC profile
#[derive(Debug, Clone, Copy)]
enum ImageType {
    Standard8Bit,
    WideGamutSdr,
    Hdr,
}

/// Encoding configuration optimized for different image types
#[derive(Debug)]
struct EncodingConfig {
    image_type: ImageType,
    use_8bit_data: bool,
    color_encoding: jpegxl_rs::encode::ColorEncoding,
    description: &'static str,
}

/// Estimate original bit depth from f32 pixel data
/// This function reverse-engineers the likely source bit depth
fn estimate_original_bit_depth(pixels_f32: &[f32], icc_profile: &Option<Vec<u8>>) -> u8 {
    let profile_suggests_high_bit = icc_profile.as_ref().map_or(false, |p| p.len() > 400);

    // Analyze pixel value precision
    let sample_size = (pixels_f32.len() / 100).max(1000).min(10000);
    let mut unique_values = std::collections::HashSet::new();

    for &pixel in pixels_f32.iter().take(sample_size) {
        if (0.0..=1.0).contains(&pixel) {
            // Reverse-convert f32 value to 8-bit scale
            let scaled_8bit = (pixel * 255.0).round() as u8;
            let rescaled = scaled_8bit as f32 / 255.0;

            // If the difference is small, it's likely from 8-bit source
            if (pixel - rescaled).abs() < 0.002 {
                unique_values.insert(scaled_8bit);
            }
        }
    }

    let appears_8bit_quantized =
        unique_values.len() <= 256 && pixels_f32.iter().take(sample_size).all(|&p| p <= 1.0);

    if appears_8bit_quantized && !profile_suggests_high_bit {
        println!("JXL: Detected 8-bit quantization pattern - standard 8-bit image");
        8
    } else if profile_suggests_high_bit {
        println!("JXL: ICC profile analysis suggests 10-bit equivalent");
        10
    } else {
        println!("JXL: High bit-depth pattern detected");
        16
    }
}

/// JPEG XL encoding options
///
/// Note: jpegxl-rs v0.11.2 has known issues with lossless encoding.
/// RGBA images with lossless mode cause ApiUsage errors.
/// This implementation automatically falls back to high quality mode.
///
/// * `lossless` - Use lossless compression (auto-fallback for RGBA images)
/// * `speed` - Encoding speed (0-10), lower values are faster but lower quality
/// * `quality` - Quality (0.1-15.0), higher values mean better quality. Default 1.0, recommended 0.5-3.0
/// * `use_container` - Configure encoder to use JPEG XL container format
/// * `uses_original_profile` - Use original color profile (always enabled for lossless)
/// * `decoding_speed` - Decoding speed setting (0-4), lower values mean higher quality
/// * `init_buffer_size` - Initial output buffer size (UI sends KB, converted to bytes internally), minimum 32KB
/// * `color_encoding` - Color encoding method, default is sRGB
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

/// Encoding speed enumeration
/// - Lightning: Fastest speed, lowest quality
/// - Thunder: Very fast, low quality  
/// - Falcon: Fast, slightly low quality
/// - Cheetah: Balanced speed and quality
/// - Hare: Slightly slow, good quality
/// - Wombat: Slow, very good quality
/// - Squirrel: Very slow, highest quality
/// - Kitten: Best quality, very slow
/// - Tortoise: Best quality, very slow
/// - Glacier: Best quality, very slow, for archival use
///
/// Note: Slower speeds produce higher quality but take longer to encode.
/// Speed settings range from 0-10, where 0 is fastest and 10 is highest quality.
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
            EncoderSpeed::Lightning => Lightning,
            EncoderSpeed::Thunder => Thunder,
            EncoderSpeed::Falcon => Falcon,
            EncoderSpeed::Cheetah => Cheetah,
            EncoderSpeed::Hare => Hare,
            EncoderSpeed::Wombat => Wombat,
            EncoderSpeed::Squirrel => Squirrel,
            EncoderSpeed::Kitten => Kitten,
            EncoderSpeed::Tortoise => Tortoise,
            EncoderSpeed::Glacier => Glacier,
        }
    }
}

/// Color encoding method enumeration
/// - Srgb: Standard sRGB color space
/// - LinearSrgb: Linear sRGB color space
/// - SrgbLuma: sRGB color space with luminance information
/// - LinearSrgbLuma: Linear sRGB color space with luminance information
///
/// Note: Selecting appropriate color encoding optimizes image quality.  
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

/// Encode HighBitDepthImage to JPEG XL format with advanced content analysis
///
/// # Arguments
/// * `pixel_data` - Source HighBitDepthImage to encode
/// * `icc_profile` - ICC profile for color management (embedded as custom metadata box if provided)
/// * `options` - JPEG XL encoding options (JxlOptions)
/// # Returns
/// - Success: JPEG XL format byte data as Vec<u8>
/// - Failure: AppError
/// # Notes
/// * Uses jpegxl-rs crate with fallback strategies for known v0.11.2 issues
/// * ICC profiles are embedded as custom metadata boxes when provided
/// * Automatic HDR/wide gamut content detection and optimization
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &JxlOptions,
) -> Result<Vec<u8>, AppError> {
    println!("JXL: Starting JPEG XL encoding process...");

    // Perform content analysis for optimal encoding
    let analysis = EncodingAnalysis::analyze(pixel_data, icc_profile.as_deref());
    log_encoding_analysis(&analysis, "JXL");
    get_encoding_recommendations(&analysis, "JXL");

    // Get image dimensions from HighBitDepthImage
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };

    // Extract f32 pixel data and alpha channel information from HighBitDepthImage
    let (pixels_f32, is_rgba) = extract_pixel_data(pixel_data);

    println!(
        "JXL: Image properties - {}x{}, {} channels",
        width,
        height,
        if is_rgba { 4 } else { 3 }
    );
    println!(
        "JXL: Encoding settings - Lossless: {}, Quality: {}, Speed: {:?}",
        options.lossless, options.quality, options.speed
    );

    // Encoder configuration (using safe default values)
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(options.speed.to_jxl())
        .use_container(options.use_container);

    // Validate decoding_speed values (out-of-range values cause ApiUsage errors)
    let safe_decoding_speed = options.decoding_speed.clamp(0, 4);
    if safe_decoding_speed != options.decoding_speed {
        println!(
            "JXL Warning: decoding_speed value adjusted {} -> {}",
            options.decoding_speed, safe_decoding_speed
        );
    }
    builder = builder.decoding_speed(safe_decoding_speed);

    // Validate init_buffer_size values (UI side assumes kilobyte specification)
    // Convert UI values from kilobyte to byte units
    let buffer_size_kb = options.init_buffer_size; // Value from UI (KB units)
    let buffer_size_bytes = buffer_size_kb * 1024; // KB → bytes conversion

    // jpegxl-rs minimum requirement: 32KB = 32768 bytes
    let safe_buffer_size = if buffer_size_bytes < 32768 {
        32768 // 32KB minimum (32768 bytes)
    } else {
        buffer_size_bytes
    };

    if safe_buffer_size != buffer_size_bytes {
        println!(
            "JXL Warning: init_buffer_size value adjusted to minimum requirement {}KB -> 32KB (32768 bytes)",
            buffer_size_kb
        );
    } else {
        // Confirmation message when specified in kilobyte units
        println!(
            "JXL: Buffer size configured: {}KB ({} bytes)",
            buffer_size_kb, buffer_size_bytes
        );
    }

    builder = builder.init_buffer_size(safe_buffer_size);

    // HDR画像の検出
    let max_pixel_value = pixels_f32
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap_or(1.0);

    // Use analysis results from common module instead of local estimation
    let estimated_original_bit_depth = match analysis.recommended_bit_depth {
        super::common::RecommendedBitDepth::Eight => 8,
        super::common::RecommendedBitDepth::Ten => 10,
        super::common::RecommendedBitDepth::Sixteen => 16,
    };

    println!(
        "JXL: Estimated original bit depth: {} bit",
        estimated_original_bit_depth
    );

    // Use analysis results for content classification
    let is_hdr = analysis.has_hdr_content;
    let has_wide_gamut_profile = analysis.has_wide_gamut;
    let is_likely_8bit_source = estimated_original_bit_depth <= 8 && analysis.max_luminance <= 1.0;
    let is_wide_gamut_sdr = has_wide_gamut_profile && !is_hdr && !is_likely_8bit_source;

    if is_hdr {
        println!(
            "JXL: HDR content detected (max luminance: {:.3}) - using linear color encoding",
            analysis.max_luminance
        );
        // Use linear color encoding for HDR images
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::LinearSrgb);
    } else if is_wide_gamut_sdr {
        println!(
            "JXL: Wide gamut SDR content detected (ICC profile: {} bytes) - using sRGB with ICC management",
            icc_profile.as_ref().unwrap().len()
        );
        // Use sRGB for wide gamut SDR images, let ICC profile manage color gamut
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
        println!("JXL: Applied high quality settings for wide gamut content");
    } else if is_likely_8bit_source {
        println!(
            "JXL: Standard 8-bit content detected (max value: {:.3}) - using efficient 8-bit settings",
            analysis.max_luminance
        );
        // Use standard sRGB settings for efficient 8-bit processing
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
    } else {
        // Other high bit depth images
        builder = builder.color_encoding(options.color_encoding.to_jxl());
    }

    if options.uses_original_profile {
        builder = builder.uses_original_profile(true);
    }

    // Comprehensive workaround for jpegxl-rs v0.11.2 lossless issues
    // Known issues:
    // 1. RGBA images + lossless = ApiUsage errors
    // 2. Certain setting combinations are unstable
    // 3. Strict pixel value range checking

    let (use_lossless, fallback_reason) = if options.lossless {
        if is_rgba {
            (false, Some("RGBA image lossless mode issues"))
        } else if width * height > 4096 * 4096 {
            // Large images with lossless mode are also unstable
            (false, Some("large image lossless mode instability"))
        } else {
            (true, None)
        }
    } else {
        (false, None)
    };

    if let Some(reason) = fallback_reason {
        println!("JXL: Falling back to high quality mode due to: {}", reason);
    }

    if use_lossless {
        builder = builder.lossless(true);
    } else {
        // Strictly validate quality values
        // Use high quality settings for RGBA images
        let target_quality = if is_rgba && options.lossless {
            0.5 // Use high quality for lossless fallback
        } else {
            options.quality
        };

        let safe_quality = target_quality.clamp(0.1, 15.0);
        if safe_quality != options.quality {
            if is_rgba && options.lossless {
                println!(
                    "JXL: Set quality to {:.3} for RGBA lossless fallback",
                    safe_quality
                );
            } else {
                println!(
                    "JXL: Adjusted quality value {:.3} -> {:.3}",
                    options.quality, safe_quality
                );
            }
        }
        builder = builder.quality(safe_quality);
    }

    // Construct the encoder
    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL encoder build failed: {}", e)))?;

    // Add ICC profile as custom metadata if provided
    if let Some(profile_data) = &icc_profile {
        println!(
            "JXL: Embedding ICC profile... (size: {} bytes)",
            profile_data.len()
        );

        // Add ICC profile as custom metadata box
        // 'icc ' (standard 4-character code for ICC profiles)
        let icc_type = *b"icc ";
        let metadata = jpegxl_rs::encode::Metadata::Custom(icc_type, profile_data);

        if let Err(e) = encoder.add_metadata(&metadata, false) {
            println!("JXL: Failed to embed ICC profile: {:?}", e);
            println!("JXL: Continuing processing without ICC profile");
        } else {
            println!("JXL: ICC profile embedding completed successfully");
        }
    }

    // Display encoding information
    let bit_depth_info = if is_likely_8bit_source {
        " [8-bit optimized]"
    } else if estimated_original_bit_depth > 8 {
        &format!(" [{}-bit high precision]", estimated_original_bit_depth)
    } else {
        ""
    };

    println!(
        "JXL: Processing {}x{} {} image{}{}{}",
        width,
        height,
        if is_rgba { "RGBA" } else { "RGB" },
        bit_depth_info,
        if icc_profile.is_some() {
            " (with ICC profile)"
        } else {
            ""
        },
        if is_rgba && options.lossless && !use_lossless {
            " [lossless fallback]"
        } else if use_lossless {
            " [lossless]"
        } else {
            ""
        }
    );

    // Check pixel value range (HDR compatible version)
    if let Some(min_val) = pixels_f32.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        if let Some(max_val) = pixels_f32.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            println!("JXL: Pixel value range [{:.3}, {:.3}]", min_val, max_val);

            if *max_val > 1.0 {
                println!(
                    "JXL: HDR range detected (max value: {:.3}) - preserving HDR information",
                    max_val
                );
            } else if is_wide_gamut_sdr {
                println!(
                    "JXL: Wide gamut SDR range (max value: {:.3}) - color gamut managed by ICC profile",
                    max_val
                );
            } else if is_likely_8bit_source {
                println!(
                    "JXL: Standard 8-bit SDR range (max value: {:.3}) - efficient 8-bit processing",
                    max_val
                );
            }

            if *min_val < 0.0 {
                println!("JXL: Warning - Negative values detected, will clamp to 0.0");
            }
        }
    }

    // Alpha channel support encoding
    // Apply GitHub Issue #96 solution: set has_alpha() in builder
    if is_rgba {
        println!("JXL: Processing RGBA image (preserving alpha channel)...");
        builder = builder.has_alpha(true);
        // Rebuild encoder (has_alpha must be set at builder time)
        encoder = builder.build().map_err(|e| {
            AppError::Encode(format!("JXL encoder rebuild with alpha failed: {}", e))
        })?;
    }

    // RGBA processing based on GitHub Issue #96 solution (HDR compatible version)
    let final_data: Vec<f32> = if is_rgba {
        println!("JXL: Processing RGBA image as-is (alpha channel preserved, HDR support)");

        // For RGBA images, preserve alpha channel as-is
        let mut rgba_data = pixels_f32.to_vec();

        // HDR support: clamp only negative values, no upper limit
        for pixel in rgba_data.iter_mut() {
            if *pixel < 0.0 {
                *pixel = 0.0; // Clamp only negative values to 0
            }
            // Preserve values > 1.0 as HDR information
        }

        rgba_data
    } else {
        // For RGB images
        let mut rgb_data = pixels_f32.to_vec();

        // HDR support: clamp only negative values, no upper limit
        for pixel in rgb_data.iter_mut() {
            if *pixel < 0.0 {
                *pixel = 0.0; // Clamp only negative values to 0
            }
            // Preserve values > 1.0 as HDR information
        }

        rgb_data
    };

    // 最終チェック
    let expected_channels = if is_rgba { 4 } else { 3 };
    let expected_length = (width * height * expected_channels) as usize;

    println!(
        "JXL: Starting encode - data length: {}, expected: {} ({} channels)",
        final_data.len(),
        expected_length,
        expected_channels
    );

    if final_data.len() != expected_length {
        return Err(AppError::Encode(format!(
            "JXL: data length mismatch: got {}, expected {} for {}x{} {}-channel image",
            final_data.len(),
            expected_length,
            width,
            height,
            expected_channels
        )));
    }

    // GitHub Issue #96 solution: Use EncoderFrame and encode_frame
    println!("JXL: Executing encode using EncoderFrame...");

    let encode_result: Result<Vec<u8>, _> = if is_likely_8bit_source {
        // 8-bit画像の場合：u8データを使用してビット深度を適切に設定
        println!("JXL: Encoding as 8-bit image with u8 data");
        let data_u8: Vec<u8> = final_data
            .iter()
            .map(|&f| (f * 255.0).round().clamp(0.0, 255.0) as u8)
            .collect();

        let encoder_frame_u8 =
            EncoderFrame::new(data_u8.as_slice()).num_channels(expected_channels as u32);
        encoder
            .encode_frame::<u8, u8>(&encoder_frame_u8, width, height)
            .map(|result| result.to_vec())
    } else {
        // 高ビット深度画像の場合：f32データを使用
        println!("JXL: Encoding as high bit-depth image with f32 data");
        let encoder_frame_f32 =
            EncoderFrame::new(final_data.as_slice()).num_channels(expected_channels as u32);
        encoder
            .encode_frame::<f32, f32>(&encoder_frame_f32, width, height)
            .map(|result| result.to_vec())
    };

    let buffer = match encode_result {
        Ok(result) => {
            println!(
                "JXL: Encoding successful - output size: {} bytes",
                result.len()
            );
            result
        }
        Err(e) => {
            println!("JXL: Initial encoding failed - error details: {:?}", e);

            // Staged fallback strategy for known jpegxl-rs issues
            println!("JXL: Attempting emergency fallback for jpegxl-rs compatibility issues...");

            // 最も安全な設定で再試行
            let mut fallback_encoder = encoder_builder()
                .speed(Cheetah) // 中程度の速度
                .quality(1.0) // デフォルト品質
                .use_container(false) // コンテナなし
                .color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb)
                .build()
                .map_err(|e| {
                    AppError::Encode(format!("JXL fallback encoder build failed: {}", e))
                })?;

            println!("JXL: 緊急フォールバック設定でエンコード再試行中...");

            // フォールバック時はRGB（3チャンネル）に変換
            let fallback_data = if is_rgba {
                let mut rgb = Vec::with_capacity((final_data.len() / 4) * 3);
                for chunk in final_data.chunks_exact(4) {
                    rgb.push(chunk[0]); // R
                    rgb.push(chunk[1]); // G
                    rgb.push(chunk[2]); // B
                    // Discard alpha channel
                }
                rgb
            } else {
                final_data.clone()
            };

            // Use appropriate data type based on 8-bit detection for fallback too
            let fallback_result: Result<Vec<u8>, _> = if is_likely_8bit_source {
                println!("JXL: Fallback - encoding with 8-bit u8 data");
                let fallback_u8: Vec<u8> = fallback_data
                    .iter()
                    .map(|&f| (f * 255.0).round().clamp(0.0, 255.0) as u8)
                    .collect();
                let fallback_frame_u8 = EncoderFrame::new(fallback_u8.as_slice()).num_channels(3);
                fallback_encoder
                    .encode_frame::<u8, u8>(&fallback_frame_u8, width, height)
                    .map(|result| result.to_vec())
            } else {
                println!("JXL: Fallback - encoding with f32 data");
                let fallback_frame_f32 =
                    EncoderFrame::new(fallback_data.as_slice()).num_channels(3);
                fallback_encoder
                    .encode_frame::<f32, f32>(&fallback_frame_f32, width, height)
                    .map(|result| result.to_vec())
            };

            match fallback_result {
                Ok(result) => {
                    println!(
                        "JXL: Emergency fallback successful - output size: {} bytes",
                        result.len()
                    );
                    println!("JXL: Note: Used safe settings instead of original configuration");
                    result
                }
                Err(fallback_err) => {
                    println!("JXL: Emergency fallback also failed");
                    println!("JXL: Original error: {:?}", e);
                    println!("JXL: Fallback error: {:?}", fallback_err);
                    println!("JXL: Configuration information:");
                    eprintln!("  - Width: {}, Height: {}", width, height);
                    eprintln!("  - Is RGBA: {}", is_rgba);
                    eprintln!("  - Data length: {}", final_data.len());
                    eprintln!("  - Lossless (requested): {}", options.lossless);
                    eprintln!("  - Lossless (actual): {}", use_lossless);
                    eprintln!("  - Quality: {}", options.quality);
                    eprintln!("  - Speed: {:?}", options.speed);
                    eprintln!("  - Use container: {}", options.use_container);
                    eprintln!(
                        "  - Uses original profile: {}",
                        options.uses_original_profile
                    );
                    eprintln!("  - Color encoding: {:?}", options.color_encoding);
                    if let Some(reason) = fallback_reason {
                        println!("  - Fallback reason: {}", reason);
                    }
                    println!("JXL: Conversion failed due to known issues in jpegxl-rs v0.11.2");
                    println!(
                        "JXL: Consider using a newer version of the library or an alternative library"
                    );
                    return Err(AppError::Encode(format!(
                        "JXL encode failed even with fallback: original={:?}, fallback={:?}",
                        e, fallback_err
                    )));
                }
            }
        }
    };

    // Provide ICC profile recommendations
    provide_icc_recommendations("JXL", analysis.has_wide_gamut, analysis.has_hdr_content);

    Ok(buffer)
}

/// Lossless transcode JPEG to JPEG XL format
///
/// # Arguments
/// * `jpeg_data` - Source JPEG byte data for conversion
/// * `options` - JXL encoding options (JxlOptions)
/// # Returns
/// - On success, returns JPEG XL byte sequence as `Vec<u8>`.
/// - On failure, returns `AppError`.
#[allow(dead_code)]
pub fn transcode(jpeg_data: &[u8], options: &JxlOptions) -> Result<Vec<u8>, AppError> {
    // This function does not handle pixel data directly, so no modification is needed.
    // uses_original_profile(true) is effective for JPEG recompression.
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(options.speed.to_jxl())
        .use_container(options.use_container)
        .uses_original_profile(true)
        .decoding_speed(options.decoding_speed)
        .init_buffer_size(options.init_buffer_size)
        .color_encoding(options.color_encoding.to_jxl());

    // Apply lossless bug workarounds for JPEG transcode as well
    // JPEG is inherently lossy, so lossless has little meaning,
    // but use lossy mode to avoid library bugs
    let use_transcode_lossless = false; // Always lossy for safety

    if use_transcode_lossless && options.lossless {
        builder = builder.lossless(true);
    } else {
        // Use high quality settings for JPEG transcode
        let transcode_quality = if options.lossless {
            0.5
        } else {
            options.quality
        };
        builder = builder.quality(transcode_quality.clamp(0.1, 15.0));

        if options.lossless {
            println!("JXL: Using high quality mode for JPEG transcode due to library issues");
        }
    }

    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL transcoder build failed: {}", e)))?;

    let buffer: EncoderResult<u8> = encoder
        .encode_jpeg(jpeg_data)
        .map_err(|e| AppError::Encode(format!("JXL transcode failed: {}", e)))?;

    Ok(buffer.to_vec())
}

/// JXLファイルサイズを推定
pub fn estimate_size(img: &HighBitDepthImage, options: &JxlOptions) -> usize {
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

    // JXLの圧縮率推定（非常に効率的な圧縮）
    let compression_ratio = if options.lossless {
        0.3 // ロスレスの場合は30%圧縮
    } else {
        // 品質に基づく圧縮率
        let quality_factor = options.quality / 100.0;
        0.03 + (quality_factor * 0.12) // 3%-15%の範囲
    };

    (uncompressed_size as f64 * compression_ratio as f64) as usize
}
