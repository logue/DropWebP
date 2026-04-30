use crate::error::AppError;
use crate::options::HighBitDepthImage;
use jpegli_rs::encoder::{ChromaSubsampling, EncoderConfig, PixelLayout, Unstoppable};
use serde::{Deserialize, Serialize};

/// Default quality.
const DEFAULT_QUALITY: u8 = 95;

/// JPEG (jpegli) encoding options.
///
/// jpegli is a high-quality JPEG encoder bundled with the libjxl project.
/// It produces better compression and quality than a standard JPEG encoder
/// by reusing JPEG XL technology while still emitting standard JPEG output.
///
/// # Future Ultra HDR support
/// The `ultra_hdr` option is reserved for future support of Ultra HDR (JPEG-R).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JpegOptions {
    /// Quality (1-100 range, default: 95). 85 or higher is recommended.
    pub quality: u8,

    /// Progressive JPEG (recommended). Improves perceived load time on the web.
    #[serde(default = "default_progressive")]
    pub progressive: bool,

    /// Optimize Huffman tables (recommended). Further reduces file size.
    #[serde(default = "default_optimize")]
    pub optimize: bool,

    /// Ultra HDR (JPEG-R with gain map) encoding (planned). Currently ignored.
    #[serde(default = "default_ultra_hdr")]
    pub ultra_hdr: bool,

    /// Quality of the Ultra HDR gain map (1-100 range, default: 85). Reserved for future use.
    #[serde(default = "default_gainmap_quality")]
    pub gainmap_quality: u8,
}

fn default_progressive() -> bool {
    true
}

fn default_optimize() -> bool {
    true
}

fn default_ultra_hdr() -> bool {
    false
}

fn default_gainmap_quality() -> u8 {
    85
}

impl Default for JpegOptions {
    fn default() -> Self {
        Self {
            quality: DEFAULT_QUALITY,
            progressive: true,
            optimize: true,
            ultra_hdr: false,
            gainmap_quality: 85,
        }
    }
}

/// Encode an image to JPEG using jpegli.
///
/// # Arguments
/// - `img`: source high-bit-depth image.
/// - `icc_profile`: optional ICC profile to embed.
/// - `options`: JPEG encoding options.
///
/// # Returns
/// Encoded JPEG bytes on success.
///
/// # Errors
/// Returns `AppError` when jpegli fails.
///
/// # Notes
/// - jpegli yields higher quality than standard JPEG and reuses JPEG XL technology.
/// - Transparency is not supported; RGBA inputs are flattened to RGB.
/// - The recommended quality range is 85-100.
pub fn encode(
    img: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &JpegOptions,
) -> Result<Vec<u8>, AppError> {
    println!("Starting jpegli encoding...");
    println!(
        "Quality: {}, Progressive: {}, Optimize: {}",
        options.quality, options.progressive, options.optimize
    );

    // Validate the requested quality.
    if options.quality < 85 {
        println!(
            "Warning: Quality {} is below recommended minimum of 85 for high-quality JPEG",
            options.quality
        );
    }

    // Get image dimensions and RGB data.
    let (width, height, rgb_data) = match img {
        HighBitDepthImage::Rgb(buf) => {
            let (w, h) = buf.dimensions();
            // Convert from f32 to u8.
            let data: Vec<u8> = buf
                .as_raw()
                .iter()
                .map(|&x| (x.clamp(0.0, 1.0) * 255.0) as u8)
                .collect();
            (w, h, data)
        }
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
            let (w, h) = buf.dimensions();
            // Convert RGBA to RGB (alpha is discarded).
            println!("Warning: Converting RGBA to RGB (alpha channel will be discarded)");
            let mut rgb_data = Vec::with_capacity((w * h * 3) as usize);

            for pixel in buf.pixels() {
                let r = (pixel[0].clamp(0.0, 1.0) * 255.0) as u8;
                let g = (pixel[1].clamp(0.0, 1.0) * 255.0) as u8;
                let b = (pixel[2].clamp(0.0, 1.0) * 255.0) as u8;
                rgb_data.push(r);
                rgb_data.push(g);
                rgb_data.push(b);
            }

            (w, h, rgb_data)
        }
    };

    println!(
        "Image dimensions: {}x{}, data size: {} bytes",
        width,
        height,
        rgb_data.len()
    );

    // jpegli-rs 0.12 API: build a config via ycbcr().
    let mut config = EncoderConfig::ycbcr(options.quality, ChromaSubsampling::Quarter);

    if options.progressive {
        config = config.progressive(true);
    }

    if options.optimize {
        config = config.optimize_huffman(true);
    }

    // Add the ICC profile to the config (jpegli-rs 0.8+ supports this natively).
    if let Some(ref icc) = icc_profile {
        config = config.icc_profile(icc.clone());
    }

    println!("Encoding image data...");
    let mut encoder = config
        .encode_from_bytes(width, height, PixelLayout::Rgb8Srgb)
        .map_err(|e| AppError::Encode(format!("Failed to create JPEG encoder: {:?}", e)))?;

    encoder
        .push_packed(&rgb_data, Unstoppable)
        .map_err(|e| AppError::Encode(format!("Failed to push image data: {:?}", e)))?;

    let jpeg_data = encoder
        .finish()
        .map_err(|e| AppError::Encode(format!("Failed to finish encoding: {:?}", e)))?;

    println!("jpegli encoding completed: {} bytes", jpeg_data.len());

    Ok(jpeg_data)
}

/// Append an ICC profile to already-encoded JPEG data.
///
/// In JPEG, ICC profiles are embedded in APP2 markers.
///
/// # Notes
/// jpegli-rs 0.8 and later support ICC profiles natively, so this helper is
/// retained only for backwards compatibility.
#[allow(dead_code)]
fn add_icc_profile(jpeg_data: Vec<u8>, icc: &[u8]) -> Result<Vec<u8>, AppError> {
    // JPEG marker: insert APP2 right after the SOI (0xFFD8) marker.
    if jpeg_data.len() < 2 || jpeg_data[0] != 0xFF || jpeg_data[1] != 0xD8 {
        return Err(AppError::Encode(
            "Invalid JPEG: missing SOI marker".to_string(),
        ));
    }

    // APP2 marker (0xFFE2) + "ICC_PROFILE\0" + sequence number.
    const MAX_CHUNK_SIZE: usize = 65533 - 14; // 65535 - marker(2) - length(2) - "ICC_PROFILE\0"(12) - seq(2)
    let chunk_count = icc.len().div_ceil(MAX_CHUNK_SIZE);

    let mut result = Vec::with_capacity(jpeg_data.len() + icc.len() + chunk_count * 18);

    // Copy the SOI marker.
    result.extend_from_slice(&jpeg_data[0..2]);

    // Split the ICC profile into chunks and append them as APP2 markers.
    for (i, chunk) in icc.chunks(MAX_CHUNK_SIZE).enumerate() {
        result.push(0xFF); // Start of marker.
        result.push(0xE2); // APP2.

        let seg_len = 2 + 12 + 2 + chunk.len(); // length(2) + "ICC_PROFILE\0"(12) + seq(2) + data
        result.push((seg_len >> 8) as u8);
        result.push(seg_len as u8);

        result.extend_from_slice(b"ICC_PROFILE\0"); // Identifier.
        result.push((i + 1) as u8); // Current chunk number (1-based).
        result.push(chunk_count as u8); // Total chunk count.

        result.extend_from_slice(chunk);
    }

    // Copy the rest of the JPEG data.
    result.extend_from_slice(&jpeg_data[2..]);

    Ok(result)
}

/// Estimate JPEG file size.
///
/// # Arguments
/// - `img`: source high-bit-depth image.
/// - `options`: JPEG encoding options.
///
/// # Returns
/// Estimated JPEG file size in bytes.
///
/// # Notes
/// This is an estimate; actual file size may differ.
pub fn estimate_size(img: &HighBitDepthImage, options: &JpegOptions) -> usize {
    let (width, height) = match img {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };

    // Base pixel count (JPEG output is always RGB, so 3 channels).
    let pixel_count = (width * height) as usize;
    let uncompressed_size = pixel_count * 3; // RGB = 3 bytes per pixel

    // Estimate compression ratio based on quality.
    // JPEG compression depends heavily on the quality setting.
    let compression_ratio = match options.quality {
        95..=100 => 0.15, // High quality: ~85% reduction
        85..=94 => 0.10,  // Standard quality: ~90% reduction
        70..=84 => 0.08,  // Mid quality: ~92% reduction
        50..=69 => 0.06,  // Low quality: ~94% reduction
        _ => 0.04,        // Very low quality: ~96% reduction
    };

    // Account for optimization and progressive encoding effects.
    let optimization_factor = if options.optimize { 0.95 } else { 1.0 }; // ~5% reduction with optimization
    let progressive_factor = if options.progressive { 1.02 } else { 1.0 }; // ~2% growth with progressive

    // Base estimated size.
    let base_size = (uncompressed_size as f64 * compression_ratio) as usize;

    // Apply factors.
    let estimated_size = (base_size as f64 * optimization_factor * progressive_factor) as usize;

    // Guarantee a minimum size (header and metadata overhead).
    let min_size = 2048; // At least 2 KB.
    estimated_size.max(min_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = JpegOptions::default();
        assert_eq!(options.quality, DEFAULT_QUALITY);
        assert!(options.progressive);
        assert!(options.optimize);
    }

    #[test]
    fn test_estimate_size_quality_variations() {
        use image::ImageBuffer;

        // Create a test image (100x100 RGB).
        let img_buffer: ImageBuffer<image::Rgb<f32>, Vec<f32>> =
            ImageBuffer::from_raw(100, 100, vec![0.5f32; 100 * 100 * 3]).unwrap();
        let img = HighBitDepthImage::Rgb(img_buffer);

        // High quality settings.
        let high_quality_options = JpegOptions {
            quality: 95,
            progressive: true,
            optimize: true,
            ultra_hdr: true,
            gainmap_quality: 95,
        };
        let high_quality_size = estimate_size(&img, &high_quality_options);

        // Low quality settings.
        let low_quality_options = JpegOptions {
            quality: 50,
            progressive: false,
            optimize: false,
            ultra_hdr: false,
            gainmap_quality: 50,
        };
        let low_quality_size = estimate_size(&img, &low_quality_options);

        // Confirm that lower quality yields a smaller file size.
        assert!(low_quality_size < high_quality_size);

        // Confirm the minimum size guarantee.
        assert!(high_quality_size >= 2048);
        assert!(low_quality_size >= 2048);
    }
}
