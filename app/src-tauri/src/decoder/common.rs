#[allow(dead_code)]
/// ICC profile information for bit depth detection and color space analysis
#[derive(Debug, Clone)]
pub struct IccProfileInfo {
    pub size: usize,
    pub suggests_wide_gamut: bool,
    pub color_space: String,
    pub profile_description: String,
    pub has_high_precision: bool,
    pub transfer_function: TransferFunction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferFunction {
    Unknown,
    Srgb,
    Gamma(f32),
    Pq,  // ST.2084 PQ (HDR)
    Hlg, // HLG (HDR)
    Linear,
}

impl IccProfileInfo {
    /// Analyze ICC profile data and extract relevant information
    pub fn analyze(profile: &[u8]) -> Self {
        let size = profile.len();

        // Extract profile details if the profile is large enough
        let (color_space, profile_description, has_high_precision, transfer_function) =
            if size >= 128 {
                let color_space = extract_color_space(profile);
                let description = extract_profile_description(profile);
                let high_precision = detect_high_precision_profile(profile, &color_space);
                let transfer_fn = detect_transfer_function(profile, &description);
                (color_space, description, high_precision, transfer_fn)
            } else {
                (
                    "Unknown".to_string(),
                    "Invalid or truncated profile".to_string(),
                    false,
                    TransferFunction::Unknown,
                )
            };

        // Display P3, Rec2020, and other wide gamut profiles typically range 400-2000 bytes
        // Small profiles (< 400 bytes) are usually sRGB or basic profiles
        let suggests_wide_gamut = size > 400
            && (color_space.contains("RGB")
                || profile_description.contains("Display P3")
                || profile_description.contains("DCI-P3")
                || profile_description.contains("Rec2020")
                || profile_description.contains("ProPhoto")
                || profile_description.contains("Adobe RGB"));

        Self {
            size,
            suggests_wide_gamut,
            color_space,
            profile_description,
            has_high_precision,
            transfer_function,
        }
    }

    /// Check if this profile suggests high bit depth processing
    pub fn requires_high_precision(&self) -> bool {
        self.has_high_precision || self.suggests_wide_gamut || self.is_hdr()
    }

    /// Check if this is an HDR profile (PQ or HLG transfer function)
    pub fn is_hdr(&self) -> bool {
        matches!(
            self.transfer_function,
            TransferFunction::Pq | TransferFunction::Hlg
        )
    }

    /// Check if this is a BT.2020 (Rec. 2020) color profile
    pub fn is_bt2020(&self) -> bool {
        self.profile_description.contains("Rec2020")
            || self.profile_description.contains("BT.2020")
            || self.profile_description.contains("BT2020")
            || self.profile_description.contains("ITU-R BT.2020")
    }
}

/// Bit depth analysis result for decoder optimization
#[derive(Debug, Clone)]
pub struct BitDepthAnalysis {
    pub detected_depth: u8,
    pub max_value: u32,
    pub processing_type: ProcessingType,
    pub recommended_format: RecommendedFormat,
}

#[derive(Debug, Clone)]
pub enum ProcessingType {
    Standard8Bit,
    WideGamut8BitAs10Bit,
    HighBitDepth,
    UltraHighBitDepth, // 16-bit+
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RecommendedFormat {
    U8Optimized,  // For simple 8-bit images
    F32Required,  // For wide gamut or high bit depth
    F32Preferred, // When quality is priority
}

impl BitDepthAnalysis {
    /// Analyze bit depth requirements based on source data and ICC profile
    pub fn analyze(
        source_bit_depth: u8,
        icc_profile: Option<&IccProfileInfo>,
        _pixel_count: usize,
    ) -> Self {
        let max_value = ((1 << source_bit_depth) - 1) as u32;

        let (processing_type, recommended_format) = match source_bit_depth {
            depth if depth <= 8 => {
                if let Some(profile) = icc_profile {
                    if profile.requires_high_precision() {
                        // 8-bit data with wide gamut profile - process as higher precision
                        (
                            ProcessingType::WideGamut8BitAs10Bit,
                            RecommendedFormat::F32Required,
                        )
                    } else {
                        // Standard 8-bit processing
                        (ProcessingType::Standard8Bit, RecommendedFormat::U8Optimized)
                    }
                } else {
                    // No ICC profile, assume standard processing
                    (ProcessingType::Standard8Bit, RecommendedFormat::U8Optimized)
                }
            }
            depth if depth <= 12 => (ProcessingType::HighBitDepth, RecommendedFormat::F32Required),
            _ => (
                ProcessingType::UltraHighBitDepth,
                RecommendedFormat::F32Required,
            ),
        };

        Self {
            detected_depth: source_bit_depth,
            max_value,
            processing_type,
            recommended_format,
        }
    }
}

/// Log detailed ICC profile information for debugging
pub fn log_icc_profile_details(profile: &[u8]) {
    if profile.len() < 128 {
        println!(
            "ICC Profile: Invalid or too small (size: {} bytes)",
            profile.len()
        );
        return;
    }

    let profile_size = u32::from_be_bytes([profile[0], profile[1], profile[2], profile[3]]);
    let preferred_cmm = std::str::from_utf8(&profile[4..8]).unwrap_or("????");
    let device_class = std::str::from_utf8(&profile[12..16]).unwrap_or("????");
    let color_space = std::str::from_utf8(&profile[16..20]).unwrap_or("????");
    let profile_signature = std::str::from_utf8(&profile[36..40]).unwrap_or("????");

    println!("ICC Profile Analysis:");
    println!(
        "  Size: {} bytes (header declares: {} bytes)",
        profile.len(),
        profile_size
    );
    println!("  Preferred CMM: {}", preferred_cmm);
    println!("  Device Class: {}", device_class);
    println!("  Color Space: {}", color_space);
    println!("  Profile Signature: {}", profile_signature);

    // Extract and display profile description if available
    let description = extract_profile_description(profile);
    if !description.is_empty() && description != "Unknown" {
        println!("  Description: {}", description);
    }

    // Analyze wide gamut indicators
    let info = IccProfileInfo::analyze(profile);
    if info.suggests_wide_gamut {
        println!("  Wide Gamut: Likely (profile suggests extended color space)");
    } else {
        println!("  Wide Gamut: No (appears to be standard sRGB or similar)");
    }
}

/// Extract color space information from ICC profile
fn extract_color_space(profile: &[u8]) -> String {
    if profile.len() < 20 {
        return "Unknown".to_string();
    }

    match std::str::from_utf8(&profile[16..20]) {
        Ok(space) => match space {
            "RGB " => "RGB".to_string(),
            "CMYK" => "CMYK".to_string(),
            "GRAY" => "Grayscale".to_string(),
            "LAB " => "LAB".to_string(),
            "XYZ " => "XYZ".to_string(),
            _ => format!("Unknown ({})", space),
        },
        Err(_) => "Invalid".to_string(),
    }
}

/// Extract profile description from ICC profile tags
fn extract_profile_description(profile: &[u8]) -> String {
    // First check for color space metadata appended by HEIC decoder
    let profile_str = String::from_utf8_lossy(profile);

    // Check for [ColorSpace] metadata tag from macOS HEIC decoder
    if let Some(idx) = profile_str.find("[ColorSpace]") {
        let color_space_name = &profile_str[idx + 12..]; // Skip "[ColorSpace]"
        // Extract the color space name (up to end or newline)
        let name = color_space_name.lines().next().unwrap_or("").trim();
        if !name.is_empty() {
            println!("ICC Profile: Found color space metadata: {}", name);
            return name.to_string();
        }
    }

    // Look for common profile description patterns in the profile data
    if profile_str.contains("Display P3") {
        "Display P3".to_string()
    } else if profile_str.contains("DCI-P3") {
        "DCI-P3".to_string()
    } else if profile_str.contains("Rec2020") || profile_str.contains("BT.2020") {
        "Rec.2020".to_string()
    } else if profile_str.contains("Adobe RGB") {
        "Adobe RGB".to_string()
    } else if profile_str.contains("ProPhoto") {
        "ProPhoto RGB".to_string()
    } else if profile_str.contains("sRGB") {
        "sRGB".to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Detect if the ICC profile suggests high precision processing
fn detect_high_precision_profile(profile: &[u8], color_space: &str) -> bool {
    // Check for high precision indicators in the profile
    let profile_str = String::from_utf8_lossy(profile);

    // Wide gamut color spaces typically benefit from high precision
    color_space.contains("RGB")
        && (
            profile_str.contains("Display P3")
                || profile_str.contains("DCI-P3")
                || profile_str.contains("Rec2020")
                || profile_str.contains("BT.2020")
                || profile_str.contains("ProPhoto")
                || profile_str.contains("Adobe RGB")
                || profile.len() > 1000
            // Large profiles often indicate complex tone curves
        )
}

/// Detect transfer function from ICC profile
/// PQ (ST.2084) and HLG are HDR transfer functions
fn detect_transfer_function(profile: &[u8], description: &str) -> TransferFunction {
    let profile_str = String::from_utf8_lossy(profile);

    // Priority 1: Check description from [ColorSpace] metadata
    // Apple Gain Map HDR uses a different approach, but we treat it as PQ for compatibility
    if description.contains("Gain Map HDR") || description.contains("GainMap") {
        println!("ICC Profile: Detected Apple Gain Map HDR (treating as PQ for compatibility)");
        return TransferFunction::Pq;
    }

    if description.contains("PQ")
        || description.contains("ST.2084")
        || description.contains("SMPTE2084")
        || description.contains("ST2084")
        || description.contains("BT.2100")
    {
        println!("ICC Profile: Detected PQ transfer from color space metadata");
        return TransferFunction::Pq;
    }

    if description.contains("HLG") || description.contains("Hybrid Log") {
        println!("ICC Profile: Detected HLG transfer from color space metadata");
        return TransferFunction::Hlg;
    }

    // Priority 2: Check ICC profile content
    if profile_str.contains("PQ")
        || profile_str.contains("ST.2084")
        || profile_str.contains("SMPTE2084")
        || profile_str.contains("ST2084")
    {
        return TransferFunction::Pq;
    }

    if profile_str.contains("HLG") || profile_str.contains("Hybrid Log") {
        return TransferFunction::Hlg;
    }

    // Check for sRGB
    if profile_str.contains("sRGB") || description.contains("sRGB") {
        return TransferFunction::Srgb;
    }

    // Check for linear
    if profile_str.contains("linear") || profile_str.contains("Linear") {
        return TransferFunction::Linear;
    }

    // Check for Display P3 (typically uses sRGB transfer function)
    if description.contains("Display P3") {
        return TransferFunction::Srgb;
    }

    TransferFunction::Unknown
}
