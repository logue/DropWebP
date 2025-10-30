use super::common::{log_icc_profile_details, BitDepthAnalysis, IccProfileInfo, ProcessingType};
use crate::error::AppError;
use crate::options::HighBitDepthImage;
use libheif_rs::{HeifContext, ImageHandle, LibHeif};

/// Analyze pixel data to determine actual bit depth and processing requirements
fn analyze_bit_depth(
    interleaved_data: &[u8],
    bytes_per_pixel: usize,
    profile_info: Option<&IccProfileInfo>,
    nominal_bit_depth: u8,
) -> BitDepthAnalysis {
    let pixel_count = interleaved_data.len() / bytes_per_pixel.max(1);
    BitDepthAnalysis::analyze(nominal_bit_depth, profile_info, pixel_count)
}

/// Calculate bytes per pixel from data layout
fn calculate_bytes_per_pixel(
    data_len: usize,
    total_pixels: usize,
    expected_channels: usize,
) -> usize {
    if data_len % (total_pixels * expected_channels) == 0 {
        data_len / (total_pixels * expected_channels)
    } else {
        // Try RGB (3 channels) recalculation
        let expected_channels_rgb = 3;
        if data_len % (total_pixels * expected_channels_rgb) == 0 {
            println!("HEIC: Recalculating as RGB (3 channels)");
            data_len / (total_pixels * expected_channels_rgb)
        } else {
            // Estimation fallback
            println!("HEIC: Data size mismatch, using estimation");
            data_len / total_pixels / 3
        }
    }
}

/// Create HighBitDepthImage from analyzed pixel data
fn create_high_bit_depth_image(
    analysis: &BitDepthAnalysis,
    data: &[u8],
    bytes_per_pixel: usize,
    width: u32,
    height: u32,
    has_alpha: bool,
) -> Result<HighBitDepthImage, AppError> {
    let pixels_f32 = match analysis.processing_type {
        ProcessingType::Standard8Bit => {
            println!("HEIC: Creating standard 8-bit image buffer");
            data.iter().map(|&p| p as f32 / 255.0).collect()
        }
        ProcessingType::WideGamut8BitAs10Bit => {
            println!("HEIC: Processing 8-bit data as wide gamut equivalent");
            // Preserve original 8-bit values but mark for wide gamut processing
            data.iter().map(|&p| p as f32 / 255.0).collect()
        }
        ProcessingType::HighBitDepth | ProcessingType::UltraHighBitDepth => {
            println!(
                "HEIC: Processing high bit-depth data ({}bit)",
                analysis.detected_depth
            );
            if bytes_per_pixel == 2 {
                let data_u16: &[u16] = bytemuck::cast_slice(data);
                data_u16
                    .iter()
                    .map(|&p| p as f32 / analysis.max_value as f32)
                    .collect()
            } else {
                data.iter()
                    .map(|&p| p as f32 / analysis.max_value as f32)
                    .collect()
            }
        }
    };

    // Create appropriate image buffer
    if has_alpha {
        let buffer = image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(width, height, pixels_f32)
            .ok_or_else(|| AppError::Decode("Failed to create RGBA f32 ImageBuffer".to_string()))?;
        Ok(HighBitDepthImage::Rgba(buffer))
    } else {
        // Convert RGBA to RGB by removing alpha channel
        let rgb_pixels: Vec<f32> = pixels_f32
            .chunks_exact(4)
            .flat_map(|rgba| &rgba[0..3])
            .cloned()
            .collect();
        let rgb_buffer =
            image::ImageBuffer::<image::Rgb<f32>, _>::from_raw(width, height, rgb_pixels)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create RGB f32 ImageBuffer".to_string())
                })?;
        Ok(HighBitDepthImage::Rgb(rgb_buffer))
    }
}

/// Decode HEIF/HEIC file to HighBitDepthImage with HDR and wide gamut support
///
/// This function preserves color information and bit depth as much as possible,
/// delegating tone mapping and color space conversion to encoders.
pub fn decode(bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    // Initialize HEIF library and decode image handle
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(bytes)
        .map_err(|e| AppError::Decode(format!("Failed to create HEIF context: {}", e)))?;
    let handle: ImageHandle = ctx
        .primary_image_handle()
        .map_err(|e| AppError::Decode(format!("Failed to get primary image handle: {}", e)))?;

    // Extract ICC profile if present
    let icc_profile: Option<Vec<u8>> = handle.color_profile_raw().map(|p| p.data.to_vec());

    // Get basic image properties
    let width = handle.width();
    let height = handle.height();
    let has_alpha = handle.has_alpha_channel();
    let luma_bits = handle.luma_bits_per_pixel();
    let chroma_bits = handle.chroma_bits_per_pixel();

    // Log image metadata
    println!(
        "HEIC: Image metadata - {}x{}, luma: {}bit, chroma: {}bit, alpha: {}",
        width, height, luma_bits, chroma_bits, has_alpha
    );

    // Analyze ICC profile if present
    let profile_info = if let Some(ref profile) = icc_profile {
        let info = IccProfileInfo::analyze(profile);
        println!(
            "HEIC: ICC profile detected - {} bytes{}",
            info.size,
            if info.suggests_wide_gamut {
                " (wide gamut suspected)"
            } else {
                ""
            }
        );

        if profile.len() >= 128 {
            log_icc_profile_details(profile);
        }
        Some(info)
    } else {
        println!("HEIC: No ICC profile found (assuming sRGB)");
        None
    };

    // Decode image data using standard RGBA format
    println!("HEIC: Performing standard RGBA decode...");
    let img = lib_heif
        .decode(
            &handle,
            libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
            None,
        )
        .map_err(|e| AppError::Decode(format!("HEIF decode failed: {}", e)))?;

    let interleaved_plane = img
        .planes()
        .interleaved
        .ok_or_else(|| AppError::Decode("Failed to get interleaved plane".to_string()))?;

    // Calculate data layout
    let total_pixels = width as usize * height as usize;
    let expected_channels = if has_alpha { 4 } else { 3 };
    let data_len = interleaved_plane.data.len();

    let bytes_per_pixel = calculate_bytes_per_pixel(data_len, total_pixels, expected_channels);

    println!(
        "HEIC: Data analysis - {} pixels, {} channels, {} bytes, {} bytes/pixel",
        total_pixels, expected_channels, data_len, bytes_per_pixel
    );

    // Perform comprehensive bit depth analysis
    let nominal_bit_depth = luma_bits.max(chroma_bits);
    let bit_depth_analysis = analyze_bit_depth(
        interleaved_plane.data,
        bytes_per_pixel,
        profile_info.as_ref(),
        nominal_bit_depth,
    );

    // Log final analysis results
    println!(
        "HEIC: Final analysis - detected: {}bit, max_value: {}, type: {:?}",
        bit_depth_analysis.detected_depth,
        bit_depth_analysis.max_value,
        bit_depth_analysis.processing_type
    );

    // Create appropriate HighBitDepthImage based on analysis
    let high_bit_depth_image = create_high_bit_depth_image(
        &bit_depth_analysis,
        &interleaved_plane.data,
        bytes_per_pixel,
        width,
        height,
        has_alpha,
    )?;

    // Return pixel data and ICC profile
    Ok((high_bit_depth_image, icc_profile))
}
