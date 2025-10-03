use std::io::Cursor;

#[allow(dead_code)]
/// ICC profile information for bit depth detection and color space analysis
#[derive(Debug, Clone)]
pub struct IccProfileInfo {
    pub size: usize,
    pub suggests_wide_gamut: bool,
    pub color_space: String,
    pub profile_description: String,
    pub has_high_precision: bool,
}

impl IccProfileInfo {
    /// Analyze ICC profile data and extract relevant information
    pub fn analyze(profile: &[u8]) -> Self {
        let size = profile.len();

        // Extract profile details if the profile is large enough
        let (color_space, profile_description, has_high_precision) = if size >= 128 {
            let color_space = extract_color_space(profile);
            let description = extract_profile_description(profile);
            let high_precision = detect_high_precision_profile(profile, &color_space);
            (color_space, description, high_precision)
        } else {
            (
                "Unknown".to_string(),
                "Invalid or truncated profile".to_string(),
                false,
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
        }
    }

    /// Check if this profile suggests high bit depth processing
    pub fn requires_high_precision(&self) -> bool {
        self.has_high_precision || self.suggests_wide_gamut
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
        pixel_count: usize,
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
    // This is a simplified implementation
    // A full implementation would parse the tag table and locate the 'desc' tag

    // Look for common profile description patterns in the profile data
    let profile_str = String::from_utf8_lossy(profile);

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

/// Extract ICC profile from various image formats
pub fn extract_icc_profile(bytes: &[u8]) -> Option<Vec<u8>> {
    // --- PNG format ---
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        let decoder = png::Decoder::new(Cursor::new(bytes));
        if let Ok(reader) = decoder.read_info() {
            if let Some(profile) = &reader.info().icc_profile {
                return Some(profile.to_vec());
            }
        }
        return None;
    }

    // --- JPEG format ---
    if bytes.starts_with(&[0xFF, 0xD8]) {
        return extract_jpeg_icc_profile(bytes);
    }

    // Add more formats as needed
    None
}

/// Extract ICC profile from JPEG APP2 segments
fn extract_jpeg_icc_profile(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut icc_chunks = std::collections::BTreeMap::new();
    let mut pos = 2; // Skip SOI marker

    while pos < bytes.len() - 4 {
        // Look for markers (starting with FF)
        if bytes[pos] != 0xFF {
            pos += 1;
            continue;
        }

        let marker = bytes[pos + 1];

        // Check for APP2 marker (FF E2)
        if marker == 0xE2 {
            let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
            if pos + 2 + length > bytes.len() {
                break;
            }

            let segment_data = &bytes[pos + 4..pos + 2 + length];

            // Check for "ICC_PROFILE" identifier
            if segment_data.starts_with(b"ICC_PROFILE\0") && segment_data.len() > 14 {
                let chunk_index = segment_data[12];
                let total_chunks = segment_data[13];
                let profile_part = &segment_data[14..];

                icc_chunks.insert(chunk_index, profile_part);

                // Check if all chunks are collected
                if icc_chunks.len() == total_chunks as usize {
                    let mut full_profile = Vec::new();
                    for i in 1..=total_chunks {
                        if let Some(chunk) = icc_chunks.get(&i) {
                            full_profile.extend_from_slice(chunk);
                        }
                    }
                    return Some(full_profile);
                }
            }
        }

        // Move to next marker
        let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
        pos += 2 + length;
    }

    None
}
