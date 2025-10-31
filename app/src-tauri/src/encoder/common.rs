use crate::options::HighBitDepthImage;

/// Encoding quality analysis and optimization
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EncodingAnalysis {
    pub has_hdr_content: bool,
    pub has_wide_gamut: bool,
    pub max_luminance: f32,
    pub dynamic_range: f32,
    pub is_hdr_or_wide_gamut: bool,
    pub recommended_bit_depth: RecommendedBitDepth,
    pub tone_mapping_required: bool,
    pub alpha_channel_present: bool,
}

#[derive(Debug, Clone)]
pub enum RecommendedBitDepth {
    Eight,   // Standard 8-bit content
    Ten,     // Wide gamut or light HDR content
    Sixteen, // High dynamic range content
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ToneMappingType {
    None,
    Reinhard,
    Filmic,
    GammaCorrection { gamma: f32 },
}

impl EncodingAnalysis {
    /// Analyze image content and ICC profile to determine optimal encoding settings
    pub fn analyze(pixel_data: &HighBitDepthImage, icc_profile: Option<&[u8]>) -> Self {
        let (pixels_f32, has_alpha) = extract_pixel_data_for_analysis(pixel_data);

        // Calculate luminance statistics
        let max_luminance = pixels_f32
            .iter()
            .enumerate()
            .filter(|(i, _)| if has_alpha { i % 4 != 3 } else { true }) // Skip alpha channel
            .map(|(_, &p)| p)
            .fold(0.0f32, |max, p| max.max(p));

        let min_luminance = pixels_f32
            .iter()
            .enumerate()
            .filter(|(i, _)| if has_alpha { i % 4 != 3 } else { true })
            .map(|(_, &p)| p)
            .fold(1.0f32, |min, p| min.min(p.max(0.0)));

        let dynamic_range = if min_luminance > 0.0 {
            max_luminance / min_luminance
        } else {
            max_luminance
        };

        // ICC profile analysis
        let has_wide_gamut =
            icc_profile.map_or(false, |profile| analyze_icc_for_wide_gamut(profile));

        // HDR detection
        let has_hdr_content = max_luminance > 1.0 || dynamic_range > 100.0;

        let is_hdr_or_wide_gamut = has_hdr_content || has_wide_gamut;

        // Recommend bit depth based on content analysis
        let recommended_bit_depth = if has_hdr_content && max_luminance > 4.0 {
            RecommendedBitDepth::Sixteen
        } else if has_hdr_content || has_wide_gamut {
            RecommendedBitDepth::Ten
        } else {
            RecommendedBitDepth::Eight
        };

        let tone_mapping_required = has_hdr_content;

        Self {
            has_hdr_content,
            has_wide_gamut,
            max_luminance,
            dynamic_range,
            is_hdr_or_wide_gamut,
            recommended_bit_depth,
            tone_mapping_required,
            alpha_channel_present: has_alpha,
        }
    }
}

/// Apply tone mapping to HDR content for 8-bit output formats
pub fn apply_tone_mapping(
    pixels: &[f32],
    has_alpha: bool,
    tone_mapping: ToneMappingType,
    exposure: f32,
) -> Vec<f32> {
    pixels
        .iter()
        .enumerate()
        .map(|(i, &pixel)| {
            let clamped = pixel.max(0.0);

            // Skip tone mapping for alpha channel
            if has_alpha && (i % 4 == 3) {
                return clamped.min(1.0);
            }

            match tone_mapping {
                ToneMappingType::None => clamped.min(1.0),
                ToneMappingType::Reinhard => {
                    if clamped > 1.0 {
                        let adjusted = clamped * exposure;
                        adjusted / (1.0 + adjusted)
                    } else {
                        clamped
                    }
                }
                ToneMappingType::Filmic => {
                    // Filmic tone mapping (simplified ACES-like curve)
                    let x = clamped * exposure;
                    let a = 2.51;
                    let b = 0.03;
                    let c = 2.43;
                    let d = 0.59;
                    let e = 0.14;
                    ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(0.0, 1.0)
                }
                ToneMappingType::GammaCorrection { gamma } => {
                    let normalized = clamped.min(1.0);
                    normalized.powf(1.0 / gamma)
                }
            }
        })
        .collect()
}

/// Convert f32 pixels to u8 with proper clamping and rounding
pub fn convert_f32_to_u8(pixels_f32: &[f32]) -> Vec<u8> {
    pixels_f32
        .iter()
        .map(|&p| (p * 255.0).round().clamp(0.0, 255.0) as u8)
        .collect()
}

/// Log encoding analysis results for debugging
pub fn log_encoding_analysis(analysis: &EncodingAnalysis, format_name: &str) {
    println!("{}:  Encoding Analysis Results:", format_name);
    println!(
        "{}:   - HDR Content: {}",
        format_name, analysis.has_hdr_content
    );
    println!(
        "{}:   - Wide Gamut: {}",
        format_name, analysis.has_wide_gamut
    );
    println!(
        "{}:   - Max Luminance: {:.3}",
        format_name, analysis.max_luminance
    );
    println!(
        "{}:   - Dynamic Range: {:.1}",
        format_name, analysis.dynamic_range
    );
    println!(
        "{}:   - Alpha Channel: {}",
        format_name, analysis.alpha_channel_present
    );
    println!(
        "{}:   - Recommended Bit Depth: {:?}",
        format_name, analysis.recommended_bit_depth
    );
    println!(
        "{}:   - Tone Mapping Required: {}",
        format_name, analysis.tone_mapping_required
    );
}

/// Provide encoding recommendations based on analysis
pub fn get_encoding_recommendations(analysis: &EncodingAnalysis, format_name: &str) {
    if analysis.has_hdr_content {
        println!("{}:  HDR Recommendations:", format_name);
        println!("{}:   - Use 10-bit or higher bit depth", format_name);
        println!(
            "{}:   - Consider tone mapping for display compatibility",
            format_name
        );
        if analysis.max_luminance > 10.0 {
            println!(
                "{}:   - Very high dynamic range detected - use 16-bit if supported",
                format_name
            );
        }
    }

    if analysis.has_wide_gamut {
        println!("{}:  Wide Gamut Recommendations:", format_name);
        println!(
            "{}:   - Use RGB color model to preserve color accuracy",
            format_name
        );
        println!(
            "{}:   - Enable ICC profile embedding if supported",
            format_name
        );
        println!(
            "{}:   - Use higher quality settings to preserve gradients",
            format_name
        );
    }

    if analysis.is_hdr_or_wide_gamut {
        println!("{}:  Quality Recommendations:", format_name);
        println!(
            "{}:   - Use lossless compression if file size allows",
            format_name
        );
        println!(
            "{}:   - If lossy, use quality >= 85 to preserve detail",
            format_name
        );
    }
}

/// Extract pixel data for analysis purposes
fn extract_pixel_data_for_analysis(img: &HighBitDepthImage) -> (&[f32], bool) {
    match img {
        HighBitDepthImage::Rgba(buffer) => (buffer.as_raw(), true),
        HighBitDepthImage::Rgb(buffer) => (buffer.as_raw(), false),
        HighBitDepthImage::Argb(buffer) => (buffer.as_raw(), true),
    }
}

/// Analyze ICC profile for wide gamut characteristics
fn analyze_icc_for_wide_gamut(profile: &[u8]) -> bool {
    // Check profile size - wide gamut profiles tend to be larger
    if profile.len() < 400 {
        return false;
    }

    // Look for wide gamut indicators in the profile
    let profile_str = String::from_utf8_lossy(profile);

    profile_str.contains("Display P3")
        || profile_str.contains("DCI-P3")
        || profile_str.contains("Rec2020")
        || profile_str.contains("BT.2020")
        || profile_str.contains("ProPhoto")
        || profile_str.contains("Adobe RGB")
        || profile.len() > 1000 // Large profiles often indicate complex tone curves or wide gamuts
}

/// Calculate optimal quality settings based on content analysis
#[allow(dead_code)]
pub fn calculate_optimal_quality(
    analysis: &EncodingAnalysis,
    base_quality: f32,
    format_supports_lossless: bool,
) -> OptimalQuality {
    let mut recommended_quality = base_quality;
    let mut use_lossless = false;

    // Adjust quality based on content
    if analysis.is_hdr_or_wide_gamut {
        // High-quality content deserves better compression
        recommended_quality = (base_quality + 15.0).min(100.0);

        // Consider lossless for critical content
        if format_supports_lossless && base_quality > 90.0 {
            use_lossless = true;
        }
    }

    // Alpha channel content often benefits from higher quality
    if analysis.alpha_channel_present {
        recommended_quality = (recommended_quality + 5.0).min(100.0);
    }

    OptimalQuality {
        quality: recommended_quality,
        use_lossless,
        reason: if analysis.is_hdr_or_wide_gamut {
            "HDR/Wide gamut content detected - increased quality"
        } else if analysis.alpha_channel_present {
            "Alpha channel present - slightly increased quality"
        } else {
            "Standard content - base quality"
        }
        .to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct OptimalQuality {
    pub quality: f32,
    pub use_lossless: bool,
    pub reason: String,
}

/// Unified ICC profile embedding interface for all formats
#[allow(dead_code)]
pub trait IccProfileEmbedder {
    fn supports_icc_embedding(&self) -> bool;
    fn embed_icc_profile(&self, data: &[u8], profile: &[u8]) -> Result<Vec<u8>, String>;
    fn get_embedding_recommendations(&self) -> Vec<String>;
}

/// Default ICC profile embedding warnings for formats that don't support it
pub fn warn_about_icc_limitations(format_name: &str, profile_size: usize) {
    println!("{}:  ICC Profile Limitations:", format_name);
    println!(
        "{}:   - ICC profile embedding not fully supported",
        format_name
    );
    println!("{}:   - Profile size: {} bytes", format_name, profile_size);
    println!(
        "{}:   - Consider using sRGB or alternative workflow",
        format_name
    );
}

/// Unified ICC profile embedding result
#[derive(Debug)]
#[allow(dead_code)]
pub enum IccEmbeddingResult {
    Success(Vec<u8>),
    NotSupported(Vec<u8>),   // Returns original data with reason
    Failed(Vec<u8>, String), // Returns original data with error message
}

/// Handle ICC profile embedding for all supported formats
pub fn handle_icc_profile_embedding(
    original_data: Vec<u8>,
    icc_profile: Option<Vec<u8>>,
    format_name: &str,
) -> Vec<u8> {
    match icc_profile {
        Some(profile_data) => {
            println!(
                "{}: ICC profile provided (size: {} bytes)",
                format_name,
                profile_data.len()
            );

            let result = match format_name {
                "JXL" => embed_jxl_icc_profile(&original_data, &profile_data),
                "AVIF" => embed_avif_icc_profile(&original_data, &profile_data),
                "WebP" => embed_webp_icc_profile(&original_data, &profile_data),
                _ => IccEmbeddingResult::NotSupported(original_data),
            };

            match result {
                IccEmbeddingResult::Success(data_with_icc) => {
                    println!("{}: Successfully embedded ICC profile", format_name);
                    data_with_icc
                }
                IccEmbeddingResult::NotSupported(original) => {
                    warn_about_icc_limitations(format_name, profile_data.len());
                    println!("{}: Continuing without ICC profile embedding", format_name);
                    original
                }
                IccEmbeddingResult::Failed(original, error) => {
                    println!("{}: Failed to embed ICC profile: {}", format_name, error);
                    println!("{}: Continuing without ICC profile embedding", format_name);
                    original
                }
            }
        }
        None => {
            println!("{}: No ICC profile provided", format_name);
            original_data
        }
    }
}

/// Embed ICC profile in JXL format (implemented)
fn embed_jxl_icc_profile(jxl_data: &[u8], _icc_profile: &[u8]) -> IccEmbeddingResult {
    // Note: This is a placeholder - actual JXL ICC embedding happens at encoder level
    // JXL ICC embedding is handled during encoding via jpegxl-rs metadata API
    IccEmbeddingResult::Success(jxl_data.to_vec())
}

/// Embed ICC profile in AVIF format (experimental/not implemented)
fn embed_avif_icc_profile(avif_data: &[u8], _icc_profile: &[u8]) -> IccEmbeddingResult {
    // AVIF ICC profile embedding is technically challenging
    // AVIF uses ISO-based container format requiring precise binary manipulation

    println!("AVIF: Warning - ICC profile embedding is currently not supported for AVIF format");
    println!("AVIF: To minimize color changes, we recommend:");
    println!("AVIF:   1. Use ColorModel::RGB");
    println!("AVIF:   2. Use BitDepth::Ten or higher");
    println!("AVIF:   3. Use high quality settings");

    // TODO: Implement ICC profile embedding using libheif-rs or dedicated library in the future
    IccEmbeddingResult::NotSupported(avif_data.to_vec())
}

/// Embed ICC profile in WebP format (requires WebPMux)
fn embed_webp_icc_profile(webp_data: &[u8], _icc_profile: &[u8]) -> IccEmbeddingResult {
    // WebP ICC profile embedding requires WebPMux functionality
    // Current webp crate doesn't expose WebPMux APIs directly

    println!("WebP: Warning - ICC profile embedding requires WebPMux functionality");
    println!("WebP: Consider the following alternatives:");
    println!("WebP:   1. Use external tools like cwebp with -icc flag");
    println!("WebP:   2. Implement WebPMux bindings");
    println!("WebP:   3. Use alternative formats like JXL for better color management");

    IccEmbeddingResult::NotSupported(webp_data.to_vec())
}

/// Provide format-specific recommendations for ICC profile handling
pub fn provide_icc_recommendations(format_name: &str, has_wide_gamut: bool, is_hdr: bool) {
    println!("{}: ICC Profile Recommendations:", format_name);

    match format_name {
        "JXL" => {
            println!("{}:   ✓ Full ICC profile embedding support", format_name);
            println!("{}:   ✓ Use LinearSrgb color encoding for HDR", format_name);
            println!("{}:   ✓ Use Srgb color encoding for SDR", format_name);
        }
        "AVIF" => {
            println!("{}:   ⚠ Limited ICC profile support", format_name);
            if has_wide_gamut || is_hdr {
                println!(
                    "{}:   → Use ColorModel::RGB for better compatibility",
                    format_name
                );
                println!("{}:   → Use BitDepth::Ten or higher", format_name);
            }
            println!(
                "{}:   → Consider JXL for better color management",
                format_name
            );
        }
        "WebP" => {
            println!("{}:   ⚠ No built-in ICC profile support", format_name);
            if has_wide_gamut || is_hdr {
                println!("{}:   → Apply tone mapping before encoding", format_name);
                println!(
                    "{}:   → Consider AVIF or JXL for wide gamut content",
                    format_name
                );
            }
            println!(
                "{}:   → Use external tools for ICC embedding if needed",
                format_name
            );
        }
        _ => {
            println!(
                "{}:   ? Format-specific recommendations not available",
                format_name
            );
        }
    }
}
