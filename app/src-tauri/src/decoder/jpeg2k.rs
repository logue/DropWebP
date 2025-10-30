use super::common::{log_icc_profile_details, BitDepthAnalysis, IccProfileInfo};
use crate::error::AppError;
use crate::options::HighBitDepthImage;
use image::{ImageBuffer, Rgb, Rgba};
use jpeg2k::Image as Jp2Image;

/// Decode JPEG 2000 file to HighBitDepthImage with ICC profile analysis
pub fn decode(bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    println!("JPEG2K: Starting JPEG 2000 decode process...");

    let jp2_image = Jp2Image::from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;

    let width = jp2_image.width();
    let height = jp2_image.height();
    let components = jp2_image.components();

    // Extract ICC profile if available (JPEG 2000 can contain ICC profiles)
    let icc_profile = extract_jp2_icc_profile(bytes);
    let profile_info = icc_profile.as_ref().map(|profile| {
        println!(
            "JPEG2K: ICC profile detected (size: {} bytes)",
            profile.len()
        );
        log_icc_profile_details(profile);
        IccProfileInfo::analyze(profile)
    });

    // Analyze bit depth and processing requirements
    let bit_depth = components[0].precision() as u8;
    let pixel_count = (width * height) as usize;
    let bit_analysis = BitDepthAnalysis::analyze(bit_depth, profile_info.as_ref(), pixel_count);

    println!(
        "JPEG2K: Image properties - {}x{}, {} bit depth, {} components",
        width,
        height,
        bit_depth,
        components.len()
    );
    println!(
        "JPEG2K: Processing analysis - Type: {:?}, Format: {:?}",
        bit_analysis.processing_type, bit_analysis.recommended_format
    );

    let result: HighBitDepthImage = match components.len() {
        3 => {
            let r = components[0].data();
            let g = components[1].data();
            let b = components[2].data();

            // Create f32 pixel data with proper normalization based on bit depth analysis
            let max_val = bit_analysis.max_value as f32;
            let mut pixels_f32 = Vec::with_capacity((width * height * 3) as usize);
            for i in 0..(width * height) as usize {
                // Normalize i32 values to f32 (0.0-1.0 range)
                pixels_f32.push(r[i] as f32 / max_val);
                pixels_f32.push(g[i] as f32 / max_val);
                pixels_f32.push(b[i] as f32 / max_val);
            }

            let buffer = ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, pixels_f32)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create f32 RGB ImageBuffer".to_string())
                })?;
            HighBitDepthImage::Rgb(buffer)
        }
        4 => {
            let r = components[0].data();
            let g = components[1].data();
            let b = components[2].data();
            let a = components[3].data();

            let max_val = bit_analysis.max_value as f32;
            let mut pixels_f32 = Vec::with_capacity((width * height * 4) as usize);
            for i in 0..(width * height) as usize {
                pixels_f32.push(r[i] as f32 / max_val);
                pixels_f32.push(g[i] as f32 / max_val);
                pixels_f32.push(b[i] as f32 / max_val);
                pixels_f32.push(a[i] as f32 / max_val); // Alpha channel normalization
            }

            let buffer = ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, pixels_f32)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create f32 RGBA ImageBuffer".to_string())
                })?;
            HighBitDepthImage::Rgba(buffer)
        }
        _ => {
            return Err(AppError::Decode(
                "Unsupported number of components in JPEG 2000 file".into(),
            ));
        }
    };

    println!("JPEG2K: Successfully decoded JPEG 2000 to f32 buffer");
    Ok((result, icc_profile))
}

/// Extract ICC profile from JPEG 2000 data
/// JPEG 2000 files can contain ICC profiles in their color specification boxes
fn extract_jp2_icc_profile(bytes: &[u8]) -> Option<Vec<u8>> {
    // This is a simplified implementation
    // A full implementation would parse the JP2 box structure and locate the 'colr' box
    // containing ICC profile data

    // JPEG 2000 uses box-based structure similar to ISO BMFF
    // Look for 'colr' (color specification) box which may contain ICC profile

    let mut pos = 0;
    while pos + 8 < bytes.len() {
        // Read box length (4 bytes, big-endian)
        if pos + 4 > bytes.len() {
            break;
        }
        let box_length =
            u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
                as usize;

        if box_length < 8 || pos + box_length > bytes.len() {
            pos += 4;
            continue;
        }

        // Read box type (4 bytes)
        if pos + 8 > bytes.len() {
            break;
        }
        let box_type = &bytes[pos + 4..pos + 8];

        // Check for color specification box
        if box_type == b"colr" && box_length > 12 {
            let box_data = &bytes[pos + 8..pos + box_length];

            // Check if this is an ICC profile (METH field = 0x02)
            if box_data.len() > 3 && box_data[0] == 0x02 {
                // ICC profile data starts after the method and precedence fields
                if box_data.len() > 4 {
                    return Some(box_data[3..].to_vec());
                }
            }
        }

        pos += box_length;
    }

    None
}
