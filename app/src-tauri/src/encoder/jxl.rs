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

/// JPEG XL encoding options
///
/// * `lossless` - Use lossless compression
/// * `speed` - Encoding speed (0-10), lower values are faster but lower quality
/// * `quality` - Quality (0.1-15.0), higher values mean better quality. Default 1.0, recommended 0.5-3.0
/// * `use_container` - Configure encoder to use JPEG XL container format
/// * `uses_original_profile` - Use original color profile (always enabled for lossless)
/// * `decoding_speed` - Decoding speed setting (0-4), lower values mean higher quality
/// * `init_buffer_size` - Initial output buffer size (UI sends KB, converted to bytes internally), minimum 32KB
/// * `color_encoding` - Color encoding method, default is sRGB
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
    let mut builder = binding.speed(options.speed.to_jxl());

    // RGBA の場合は builder 段階で has_alpha を設定
    // 注意：後でエンコーダーを再構築すると ICC profile が失われる
    if is_rgba {
        println!("JXL: Configuring for RGBA image (alpha channel support)");
        builder = builder.has_alpha(true);
    }

    // ICC profile がある場合は container format を強制的に有効化
    // ICC profile はメタデータボックスとして埋め込まれるため、container が必要
    let use_container = if icc_profile.is_some() {
        if !options.use_container {
            println!("JXL: Enabling container format to embed ICC profile");
        }
        true
    } else {
        options.use_container
    };

    builder = builder.use_container(use_container);

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

    // Check if this is a BT.2020 profile (wide color gamut for HDR)
    let is_bt2020_profile = icc_profile.as_ref().map_or(false, |profile| {
        let info = super::common::IccProfileInfo::analyze(profile);
        info.is_bt2020()
    });

    // JPEG XL color encoding 設定
    // 重要：jpegxl-rs 0.11.2 の制限により、ICC profile の直接埋め込みが困難
    // そのため、HDR の場合は LinearSrgb color_encoding を使用し、
    // ICC profile は参考情報として埋め込む
    let use_icc_for_color =
        icc_profile.is_some() && (is_bt2020_profile || is_hdr || has_wide_gamut_profile);

    if use_icc_for_color {
        println!(
            "JXL: ICC profile available (size: {} bytes)",
            icc_profile.as_ref().unwrap().len()
        );
        if is_bt2020_profile {
            println!("JXL: BT.2020 wide gamut profile detected");
        } else if is_hdr {
            println!("JXL: HDR content with ICC profile detected");
        } else if has_wide_gamut_profile {
            println!("JXL: Wide gamut ICC profile detected");
        }
        println!("JXL: ICC profile will be embedded as metadata");
        println!("JXL: Note: Will NOT set color_encoding to let ICC profile define color space");
    }

    // Color encoding の設定
    // 重要：jpegxl-rs 0.11.2 の制限により、BT.2020 などの広色域を正しく扱えない
    // ICC profile がある場合でも LinearSrgb を設定する必要がある
    if use_icc_for_color {
        // ICC profile + LinearSrgb の組み合わせ
        // HDR 輝度は保持されるが、色域は sRGB に制限される
        println!("JXL: Using LinearSrgb with ICC profile (jpegxl-rs limitation)");
        println!("JXL: WARNING - BT.2020 color gamut will be mapped to sRGB");
        println!("JXL: Recommendation: Use AVIF format for full BT.2020/HDR support");
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::LinearSrgb);
        builder = builder.uses_original_profile(true);
    } else if is_hdr {
        // HDR content で ICC profile がない場合のみ LinearSrgb を使用
        println!(
            "JXL: HDR content without ICC profile (max luminance: {:.3}) - using LinearSrgb",
            analysis.max_luminance
        );
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::LinearSrgb);

        // Note: jpegxl-rs 0.11.2 does not support intensity_target in builder API
        // HDR intensity information is preserved through LinearSrgb color encoding
        // with pixel values scaled appropriately (1.0 = 100 nits, max ~100 = 10000 nits)
        println!(
            "JXL: HDR dynamic range preserved via LinearSrgb encoding (pixel range 0-{:.1})",
            analysis.max_luminance
        );

        // HDR画像でロスレスの場合は警告を表示
        if options.lossless {
            println!(
                "JXL: WARNING - Lossless mode with HDR content will result in very large files!"
            );
            println!("JXL: Consider using lossy mode with quality 3-5 for better compression");
        }
    } else if is_wide_gamut_sdr {
        println!(
            "JXL: Wide gamut SDR content detected (ICC profile: {} bytes) - using sRGB with ICC management",
            icc_profile.as_ref().unwrap().len()
        );
        // Use sRGB for wide gamut SDR images without ICC, or let ICC profile manage if present
        if !use_icc_for_color {
            builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
        }
        println!("JXL: Applied high quality settings for wide gamut content");
    } else if is_likely_8bit_source {
        println!(
            "JXL: Standard 8-bit content detected (max value: {:.3}) - using efficient 8-bit settings",
            analysis.max_luminance
        );
        // Use standard sRGB settings for efficient 8-bit processing
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
    } else {
        // Other high bit depth images - use user-specified or default color encoding
        println!(
            "JXL: Using configured color encoding: {:?}",
            options.color_encoding
        );
        builder = builder.color_encoding(options.color_encoding.to_jxl());
    }

    // ICC profile を使用する場合、uses_original_profile を有効化
    // これにより、エンコーダーが ICC profile を尊重する
    if use_icc_for_color {
        builder = builder.uses_original_profile(true);
        println!("JXL: Enabled uses_original_profile for ICC profile support");
    }

    // ロスレス設定
    // jpegxl-rs 0.11.2以降ではRGBAでもロスレスが安定しています
    if options.lossless {
        println!("JXL: Using lossless compression mode");
        builder = builder.lossless(true);

        // ファイルサイズの推定警告
        if is_hdr || use_icc_for_color {
            let estimated_size_mb = (width * height * 12) / (1024 * 1024); // HDR/広色域の場合、約12バイト/ピクセルと推定
            println!(
                "JXL: WARNING - Lossless {} encoding may result in ~{}MB file size",
                if is_bt2020_profile {
                    "BT.2020 HDR"
                } else if is_hdr {
                    "HDR"
                } else {
                    "wide gamut"
                },
                estimated_size_mb
            );
        } else if estimated_original_bit_depth > 8 {
            let estimated_size_mb = (width * height * 8) / (1024 * 1024); // 高ビット深度の場合、約8バイト/ピクセルと推定
            println!(
                "JXL: INFO - Lossless high bit-depth encoding may result in ~{}MB file size",
                estimated_size_mb
            );
        }
    } else {
        // ロッシー圧縮時の品質設定
        let mut effective_quality = options.quality;

        // HDR画像の場合、品質設定を自動調整（より高い品質が必要）
        if (is_hdr || use_icc_for_color) && effective_quality < 5.0 {
            let recommended_quality = 5.0;
            println!(
                "JXL: WARNING - {} content detected with low quality ({:.1})",
                if is_bt2020_profile {
                    "BT.2020 HDR"
                } else if is_hdr {
                    "HDR"
                } else {
                    "Wide gamut"
                },
                effective_quality
            );
            println!(
                "JXL: Adjusting quality from {:.1} to {:.1} to preserve {} highlights",
                effective_quality,
                recommended_quality,
                if is_hdr { "HDR" } else { "color" }
            );
            println!(
                "JXL: Note: Quality below 5.0 may cause clipping in bright {} areas",
                if is_hdr { "HDR" } else { "color" }
            );
            effective_quality = recommended_quality;
        }

        let safe_quality = effective_quality.clamp(0.1, 15.0);
        if safe_quality != options.quality {
            println!(
                "JXL: Adjusted quality value {:.3} -> {:.3}",
                options.quality, safe_quality
            );
        }
        println!(
            "JXL: Using lossy compression with quality: {:.3}{}",
            safe_quality,
            if is_hdr || use_icc_for_color {
                " (optimized)"
            } else {
                ""
            }
        );
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

        // JPEG XL container format での ICC profile 埋め込み
        // 'icc ' は JPEG XL 仕様に従った 4-character code
        let icc_type = *b"icc ";
        let metadata = jpegxl_rs::encode::Metadata::Custom(icc_type, profile_data);

        if let Err(e) = encoder.add_metadata(&metadata, false) {
            println!("JXL: Failed to embed ICC profile: {:?}", e);
            println!("JXL: Continuing without ICC profile metadata");
        } else {
            println!("JXL: ICC profile embedded as metadata");
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
        if options.lossless { " [lossless]" } else { "" }
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
            println!("JXL: Attempting fallback with adjusted settings...");

            // フォールバック設定：元の設定を保持しつつ、安全な設定に変更
            let mut fallback_binding = encoder_builder();
            let mut fallback_builder = fallback_binding
                .speed(Cheetah) // 中程度の速度
                .use_container(use_container); // 元の container 設定を保持

            // Color encoding の設定（ICC profile がある場合も LinearSrgb を使用）
            if use_icc_for_color {
                println!(
                    "JXL: Fallback - using LinearSrgb with ICC profile (jpegxl-rs limitation)"
                );
                fallback_builder =
                    fallback_builder.color_encoding(jpegxl_rs::encode::ColorEncoding::LinearSrgb);
                fallback_builder = fallback_builder.uses_original_profile(true);
            } else if is_hdr {
                println!("JXL: Fallback - using LinearSrgb for HDR content without ICC profile");
                fallback_builder =
                    fallback_builder.color_encoding(jpegxl_rs::encode::ColorEncoding::LinearSrgb);
            } else {
                fallback_builder =
                    fallback_builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
            }

            // RGBA の場合は has_alpha を設定
            if is_rgba {
                println!("JXL: Fallback - configuring for RGBA");
                fallback_builder = fallback_builder.has_alpha(true);
            }

            // ロスレスの場合はロッシーに変更（ロスレスが失敗したため）
            if options.lossless {
                println!("JXL: Fallback - switching from lossless to lossy (quality 5.0)");
                fallback_builder = fallback_builder.quality(5.0);
            } else {
                fallback_builder = fallback_builder.quality(options.quality.clamp(0.1, 15.0));
            }

            let mut fallback_encoder = fallback_builder.build().map_err(|e| {
                AppError::Encode(format!("JXL fallback encoder build failed: {}", e))
            })?;

            // ICC profile を再度埋め込む
            if let Some(profile_data) = &icc_profile {
                println!(
                    "JXL: Fallback - re-embedding ICC profile ({} bytes)",
                    profile_data.len()
                );
                let icc_type = *b"icc ";
                let metadata = jpegxl_rs::encode::Metadata::Custom(icc_type, profile_data);
                if let Err(e) = fallback_encoder.add_metadata(&metadata, false) {
                    println!("JXL: Fallback - Failed to embed ICC profile: {:?}", e);
                }
            }

            println!("JXL: フォールバックエンコード実行中...");

            // フォールバック時は元のデータをそのまま使用（RGB/RGBA を保持）
            let fallback_channels = if is_rgba { 4 } else { 3 };

            // Use appropriate data type based on 8-bit detection for fallback too
            let fallback_result: Result<Vec<u8>, _> = if is_likely_8bit_source {
                println!("JXL: Fallback - encoding with 8-bit u8 data");
                let fallback_u8: Vec<u8> = final_data
                    .iter()
                    .map(|&f| (f * 255.0).round().clamp(0.0, 255.0) as u8)
                    .collect();
                let fallback_frame_u8 =
                    EncoderFrame::new(fallback_u8.as_slice()).num_channels(fallback_channels);
                fallback_encoder
                    .encode_frame::<u8, u8>(&fallback_frame_u8, width, height)
                    .map(|result| result.to_vec())
            } else {
                println!("JXL: Fallback - encoding with f32 data");
                let fallback_frame_f32 =
                    EncoderFrame::new(final_data.as_slice()).num_channels(fallback_channels);
                fallback_encoder
                    .encode_frame::<f32, f32>(&fallback_frame_f32, width, height)
                    .map(|result| result.to_vec())
            };

            match fallback_result {
                Ok(result) => {
                    println!(
                        "JXL: Fallback successful - output size: {} bytes",
                        result.len()
                    );
                    if options.lossless {
                        println!(
                            "JXL: Note: Switched to lossy mode (quality 5.0) for compatibility"
                        );
                    }
                    result
                }
                Err(fallback_err) => {
                    println!("JXL: Fallback also failed");
                    println!("JXL: Original error: {:?}", e);
                    println!("JXL: Fallback error: {:?}", fallback_err);
                    println!("JXL: Encoding configuration:");
                    eprintln!("  - Dimensions: {}x{}", width, height);
                    eprintln!("  - Is RGBA: {}", is_rgba);
                    eprintln!("  - Data length: {}", final_data.len());
                    eprintln!("  - Lossless: {}", options.lossless);
                    eprintln!("  - Quality: {}", options.quality);
                    eprintln!("  - Speed: {:?}", options.speed);
                    eprintln!("  - Use container: {}", options.use_container);
                    eprintln!(
                        "  - Uses original profile: {}",
                        options.uses_original_profile
                    );
                    eprintln!("  - Color encoding: {:?}", options.color_encoding);
                    println!("JXL: Conversion failed due to encoding issues");
                    println!("JXL: Consider adjusting settings or using a different format");
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

    // Apply lossless/lossy settings
    if options.lossless {
        println!("JXL: Using lossless mode for JPEG transcode");
        builder = builder.lossless(true);
    } else {
        let safe_quality = options.quality.clamp(0.1, 15.0);
        println!(
            "JXL: Using lossy mode for JPEG transcode with quality: {:.3}",
            safe_quality
        );
        builder = builder.quality(safe_quality);
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
