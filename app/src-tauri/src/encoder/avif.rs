use super::common::{
    EncodingAnalysis, get_encoding_recommendations, log_encoding_analysis,
    provide_icc_recommendations,
};
use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use serde::{Deserialize, Serialize};
use std::ptr;

/// AVIF format encoding options
/// quality: 0-100 (higher values mean better quality)
/// bit_depth: Bit depth (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten, BitDepth::Twelve)
/// alpha_quality: Alpha channel quality (1-100, higher values mean better quality)
/// speed: Encoding speed (0-10). 0 is highest quality but slowest, 10 is fastest
/// color_model: Color model (ColorModel::YCbCr, ColorModel::RGB)
/// threads: Number of threads to use (None for automatic)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvifOptions {
    pub quality: f32,
    pub bit_depth: BitDepth,
    pub alpha_quality: f32,
    pub speed: u8,
    pub color_model: ColorModel,
    pub threads: Option<usize>,
}

/// Bit depth enumeration
/// - Auto: Automatically determined based on input image bit depth and HDR content
/// - Eight: 8-bit (SDR only)
/// - Ten: 10-bit (HDR capable)
/// - Twelve: 12-bit (Maximum HDR precision)
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Auto,
    Eight,
    Ten,
    Twelve,
}

/// Color model enumeration
/// - YCbCr: YCbCr color model (better compression)
/// - RGB: RGB color model (better color accuracy)
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    YCbCr,
    RGB,
}

/// Encode HighBitDepthImage to AVIF format using libavif-sys with HDR support
///
/// # Arguments
/// * `pixel_data` - Source HighBitDepthImage to encode
/// * `icc_profile` - ICC profile for color management (optional)
/// * `options` - AVIF encoding options
///
/// # Returns
/// * Success: AVIF format byte data as Vec<u8>
/// * Failure: AppError
///
/// # HDR Support
/// * Automatically detects HDR content (luminance > 1.0)
/// * Uses 10-bit or 12-bit encoding for HDR
/// * Sets PQ (ST.2084) transfer characteristics for HDR
/// * Sets BT.2020 color primaries for wide gamut
/// * Embeds cICP metadata for proper HDR signaling
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &AvifOptions,
) -> Result<Vec<u8>, AppError> {
    println!("AVIF: Starting AVIF encoding process with libavif-sys...");

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
    let (pixels_f32, has_alpha) = extract_pixel_data(&pixel_data);

    println!(
        "AVIF: Image properties - {}x{}, {} channels",
        width,
        height,
        if has_alpha { 4 } else { 3 }
    );

    // Determine bit depth based on content and settings
    let target_depth = match options.bit_depth {
        BitDepth::Auto => {
            if analysis.has_hdr_content {
                println!("AVIF: Auto-selecting 10-bit for HDR content");
                10
            } else {
                println!("AVIF: Auto-selecting 8-bit for SDR content");
                8
            }
        }
        BitDepth::Eight => 8,
        BitDepth::Ten => 10,
        BitDepth::Twelve => 12,
    };

    println!(
        "AVIF: Encoding settings - Quality: {}, Bit depth: {}, Color model: {:?}",
        options.quality, target_depth, options.color_model
    );

    // HDR detection and settings
    let is_hdr = analysis.has_hdr_content && target_depth >= 10;
    if is_hdr {
        println!(
            "AVIF: HDR mode enabled (max luminance: {:.3})",
            analysis.max_luminance
        );
    }

    unsafe {
        // Create encoder
        let encoder = libavif_sys::avifEncoderCreate();
        if encoder.is_null() {
            return Err(AppError::Avif("Failed to create AVIF encoder".to_string()));
        }

        // Set encoder options
        (*encoder).maxThreads = options.threads.unwrap_or(0) as i32;
        (*encoder).speed = options.speed as i32;

        println!(
            "AVIF: Encoder settings - speed={}, maxThreads={} (0=auto)",
            options.speed,
            (*encoder).maxThreads
        );

        // Quality settings (0-100 to 0-63 for minQuantizer/maxQuantizer)
        // Lower quantizer = higher quality
        let quantizer = ((100.0 - options.quality) * 63.0 / 100.0) as i32;
        (*encoder).minQuantizer = quantizer;
        (*encoder).maxQuantizer = quantizer;

        let alpha_quantizer = ((100.0 - options.alpha_quality) * 63.0 / 100.0) as i32;
        (*encoder).minQuantizerAlpha = alpha_quantizer;
        (*encoder).maxQuantizerAlpha = alpha_quantizer;

        // Create image
        let image = libavif_sys::avifImageCreate(
            width,
            height,
            target_depth,
            if has_alpha {
                libavif_sys::AVIF_PIXEL_FORMAT_YUV444
            } else {
                libavif_sys::AVIF_PIXEL_FORMAT_YUV444
            },
        );

        if image.is_null() {
            libavif_sys::avifEncoderDestroy(encoder);
            return Err(AppError::Avif("Failed to create AVIF image".to_string()));
        }

        // Set color properties
        if is_hdr {
            // HDR: Use BT.2020 with PQ transfer
            (*image).colorPrimaries = libavif_sys::AVIF_COLOR_PRIMARIES_BT2020 as u16;
            (*image).transferCharacteristics =
                libavif_sys::AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084 as u16; // PQ
            (*image).matrixCoefficients = libavif_sys::AVIF_MATRIX_COEFFICIENTS_BT2020_NCL as u16;
            println!("AVIF: Using BT.2020 color primaries with PQ (ST.2084) transfer");
        } else {
            // SDR: Use BT.709/sRGB
            (*image).colorPrimaries = libavif_sys::AVIF_COLOR_PRIMARIES_BT709 as u16;
            (*image).transferCharacteristics =
                libavif_sys::AVIF_TRANSFER_CHARACTERISTICS_SRGB as u16;
            (*image).matrixCoefficients = libavif_sys::AVIF_MATRIX_COEFFICIENTS_BT709 as u16;
        }

        (*image).yuvRange = libavif_sys::AVIF_RANGE_FULL;

        // Convert f32 pixels to RGB based on bit depth
        let rgb_result = if target_depth == 8 {
            convert_to_rgb8(
                &pixels_f32,
                width,
                height,
                has_alpha,
                is_hdr,
                analysis.max_luminance,
            )
        } else {
            convert_to_rgb16(
                &pixels_f32,
                width,
                height,
                has_alpha,
                target_depth,
                is_hdr,
                analysis.max_luminance,
            )
        };

        let (rgb_pixels, rgb_format, rgb_depth) = match rgb_result {
            Ok(data) => data,
            Err(e) => {
                libavif_sys::avifImageDestroy(image);
                libavif_sys::avifEncoderDestroy(encoder);
                return Err(e);
            }
        };

        // Calculate row stride
        let channels = if has_alpha { 4 } else { 3 };
        let bytes_per_channel = if rgb_depth == 8 { 1 } else { 2 };
        let row_bytes = width * channels * bytes_per_channel;
        let expected_size = row_bytes * height;

        println!(
            "AVIF: RGB image setup - width={}, height={}, depth={}, channels={}, rowBytes={}, expected_size={}, actual_size={}",
            width,
            height,
            rgb_depth,
            channels,
            row_bytes,
            expected_size,
            rgb_pixels.len()
        );

        if rgb_pixels.len() != expected_size as usize {
            libavif_sys::avifImageDestroy(image);
            libavif_sys::avifEncoderDestroy(encoder);
            return Err(AppError::Avif(format!(
                "RGB pixel data size mismatch: expected {} bytes, got {} bytes",
                expected_size,
                rgb_pixels.len()
            )));
        }

        // Create RGB image
        let mut rgb_image = libavif_sys::avifRGBImage {
            width: width as u32,
            height: height as u32,
            depth: rgb_depth as u32,
            format: rgb_format,
            chromaUpsampling: libavif_sys::AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
            chromaDownsampling: libavif_sys::AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
            avoidLibYUV: 0,
            ignoreAlpha: if has_alpha { 0 } else { 1 },
            alphaPremultiplied: 0,
            isFloat: 0,
            maxThreads: options.threads.unwrap_or(0) as i32,
            pixels: rgb_pixels.as_ptr() as *mut u8,
            rowBytes: row_bytes as u32,
        };

        // Convert RGB to YUV
        println!("AVIF: Converting RGB to YUV...");
        let result = libavif_sys::avifImageRGBToYUV(image, &rgb_image);
        if result != libavif_sys::AVIF_RESULT_OK {
            let error_msg = match result {
                libavif_sys::AVIF_RESULT_UNKNOWN_ERROR => "Unknown error",
                libavif_sys::AVIF_RESULT_INVALID_FTYP => "Invalid ftyp",
                libavif_sys::AVIF_RESULT_NO_CONTENT => "No content",
                libavif_sys::AVIF_RESULT_NO_YUV_FORMAT_SELECTED => "No YUV format selected",
                libavif_sys::AVIF_RESULT_REFORMAT_FAILED => "Reformat failed",
                libavif_sys::AVIF_RESULT_UNSUPPORTED_DEPTH => "Unsupported depth",
                libavif_sys::AVIF_RESULT_ENCODE_COLOR_FAILED => "Encode color failed",
                libavif_sys::AVIF_RESULT_ENCODE_ALPHA_FAILED => "Encode alpha failed",
                libavif_sys::AVIF_RESULT_BMFF_PARSE_FAILED => "BMFF parse failed",
                libavif_sys::AVIF_RESULT_MISSING_IMAGE_ITEM => "Missing image item",
                libavif_sys::AVIF_RESULT_DECODE_COLOR_FAILED => "Decode color failed",
                libavif_sys::AVIF_RESULT_DECODE_ALPHA_FAILED => "Decode alpha failed",
                libavif_sys::AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH => "Color alpha size mismatch",
                libavif_sys::AVIF_RESULT_ISPE_SIZE_MISMATCH => "ISPE size mismatch",
                libavif_sys::AVIF_RESULT_NO_CODEC_AVAILABLE => "No codec available",
                libavif_sys::AVIF_RESULT_NO_IMAGES_REMAINING => "No images remaining",
                libavif_sys::AVIF_RESULT_INVALID_EXIF_PAYLOAD => "Invalid EXIF payload",
                libavif_sys::AVIF_RESULT_INVALID_IMAGE_GRID => "Invalid image grid",
                libavif_sys::AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION => {
                    "Invalid codec specific option"
                }
                libavif_sys::AVIF_RESULT_TRUNCATED_DATA => "Truncated data",
                libavif_sys::AVIF_RESULT_IO_NOT_SET => "IO not set",
                libavif_sys::AVIF_RESULT_IO_ERROR => "IO error",
                libavif_sys::AVIF_RESULT_WAITING_ON_IO => "Waiting on IO",
                libavif_sys::AVIF_RESULT_INVALID_ARGUMENT => "Invalid argument",
                libavif_sys::AVIF_RESULT_NOT_IMPLEMENTED => "Not implemented",
                _ => "Unknown error code",
            };
            libavif_sys::avifImageDestroy(image);
            libavif_sys::avifEncoderDestroy(encoder);
            return Err(AppError::Avif(format!(
                "Failed to convert RGB to YUV: {} (error code {})",
                error_msg, result
            )));
        }
        println!("AVIF: RGB to YUV conversion successful");

        // Add ICC profile if provided
        if let Some(ref icc_data) = icc_profile {
            libavif_sys::avifImageSetProfileICC(image, icc_data.as_ptr(), icc_data.len());
            println!("AVIF: Embedded ICC profile ({} bytes)", icc_data.len());
        }

        // Encode image
        println!(
            "AVIF: Adding image to encoder (this may take a while for large images with rav1e codec)..."
        );
        let start_time = std::time::Instant::now();
        let result = libavif_sys::avifEncoderAddImage(
            encoder,
            image,
            1,
            libavif_sys::AVIF_ADD_IMAGE_FLAG_SINGLE as u32,
        );
        println!(
            "AVIF: avifEncoderAddImage took {:.2}s",
            start_time.elapsed().as_secs_f64()
        );

        if result != libavif_sys::AVIF_RESULT_OK {
            libavif_sys::avifImageDestroy(image);
            libavif_sys::avifEncoderDestroy(encoder);
            return Err(AppError::Avif(format!(
                "Failed to add image to encoder: error code {}",
                result
            )));
        }

        // Finish encoding
        println!("AVIF: Finishing encoding...");
        let start_time = std::time::Instant::now();
        let mut output = libavif_sys::avifRWData {
            data: ptr::null_mut(),
            size: 0,
        };

        let result = libavif_sys::avifEncoderFinish(encoder, &mut output);
        println!(
            "AVIF: avifEncoderFinish took {:.2}s",
            start_time.elapsed().as_secs_f64()
        );
        if result != libavif_sys::AVIF_RESULT_OK {
            libavif_sys::avifImageDestroy(image);
            libavif_sys::avifEncoderDestroy(encoder);
            return Err(AppError::Avif(format!(
                "Failed to finish encoding: error code {}",
                result
            )));
        }

        // Copy output data
        let encoded_data = if !output.data.is_null() && output.size > 0 {
            let slice = std::slice::from_raw_parts(output.data, output.size);
            slice.to_vec()
        } else {
            Vec::new()
        };

        // Cleanup
        libavif_sys::avifRWDataFree(&mut output);
        libavif_sys::avifImageDestroy(image);
        libavif_sys::avifEncoderDestroy(encoder);

        if encoded_data.is_empty() {
            return Err(AppError::Avif("Encoded data is empty".to_string()));
        }

        println!(
            "AVIF: Encoding completed successfully ({} bytes)",
            encoded_data.len()
        );

        // Provide format-specific ICC recommendations
        provide_icc_recommendations("AVIF", analysis.has_wide_gamut, analysis.has_hdr_content);

        Ok(encoded_data)
    }
}

/// Convert f32 pixels to 8-bit RGB/RGBA
fn convert_to_rgb8(
    pixels_f32: &[f32],
    width: u32,
    height: u32,
    has_alpha: bool,
    is_hdr: bool,
    _max_luminance: f32,
) -> Result<(Vec<u8>, libavif_sys::avifRGBFormat, i32), AppError> {
    let channels = if has_alpha { 4 } else { 3 };
    let pixel_count = (width * height) as usize;
    let expected_len = pixel_count * channels;

    println!(
        "AVIF: convert_to_rgb8 - width={}, height={}, has_alpha={}, channels={}, pixel_count={}, expected_len={}, input_len={}",
        width,
        height,
        has_alpha,
        channels,
        pixel_count,
        expected_len,
        pixels_f32.len()
    );

    if pixels_f32.len() != expected_len {
        return Err(AppError::Encode(format!(
            "Pixel data length mismatch: expected {}, got {}",
            expected_len,
            pixels_f32.len()
        )));
    }

    let mut rgb_pixels = Vec::with_capacity(expected_len);

    if is_hdr {
        // HDR to SDR tone mapping (simple Reinhard)
        println!(
            "AVIF: Applying tone mapping for 8-bit output (max luminance: {:.3})",
            _max_luminance
        );
        for i in 0..pixel_count {
            let base = i * channels;
            let r = pixels_f32[base];
            let g = pixels_f32[base + 1];
            let b = pixels_f32[base + 2];

            // Reinhard tone mapping
            let r_mapped = r / (1.0 + r);
            let g_mapped = g / (1.0 + g);
            let b_mapped = b / (1.0 + b);

            rgb_pixels.push((r_mapped.clamp(0.0, 1.0) * 255.0) as u8);
            rgb_pixels.push((g_mapped.clamp(0.0, 1.0) * 255.0) as u8);
            rgb_pixels.push((b_mapped.clamp(0.0, 1.0) * 255.0) as u8);

            if has_alpha {
                let a = pixels_f32[base + 3];
                rgb_pixels.push((a.clamp(0.0, 1.0) * 255.0) as u8);
            }
        }
    } else {
        // SDR: Direct conversion (input is already in sRGB gamma space)
        for i in 0..pixel_count {
            let base = i * channels;
            let r = pixels_f32[base];
            let g = pixels_f32[base + 1];
            let b = pixels_f32[base + 2];

            rgb_pixels.push((r.clamp(0.0, 1.0) * 255.0) as u8);
            rgb_pixels.push((g.clamp(0.0, 1.0) * 255.0) as u8);
            rgb_pixels.push((b.clamp(0.0, 1.0) * 255.0) as u8);

            if has_alpha {
                let a = pixels_f32[base + 3];
                rgb_pixels.push((a.clamp(0.0, 1.0) * 255.0) as u8);
            }
        }
    }

    // Debug: Print first few pixels
    if rgb_pixels.len() >= 12 {
        println!(
            "AVIF: First 4 pixels (RGB): [{}, {}, {}] [{}, {}, {}] [{}, {}, {}] [{}, {}, {}]",
            rgb_pixels[0],
            rgb_pixels[1],
            rgb_pixels[2],
            rgb_pixels[3],
            rgb_pixels[4],
            rgb_pixels[5],
            rgb_pixels[6],
            rgb_pixels[7],
            rgb_pixels[8],
            rgb_pixels[9],
            rgb_pixels[10],
            rgb_pixels[11]
        );
    }

    let format = if has_alpha {
        libavif_sys::AVIF_RGB_FORMAT_RGBA
    } else {
        libavif_sys::AVIF_RGB_FORMAT_RGB
    };

    Ok((rgb_pixels, format, 8))
}

/// Convert f32 pixels to 10-bit or 12-bit RGB/RGBA
fn convert_to_rgb16(
    pixels_f32: &[f32],
    width: u32,
    height: u32,
    has_alpha: bool,
    target_depth: u32,
    is_hdr: bool,
    _max_luminance: f32,
) -> Result<(Vec<u8>, libavif_sys::avifRGBFormat, i32), AppError> {
    let channels = if has_alpha { 4 } else { 3 };
    let pixel_count = (width * height) as usize;
    let expected_len = pixel_count * channels;

    if pixels_f32.len() != expected_len {
        return Err(AppError::Encode(format!(
            "Pixel data length mismatch: expected {}, got {}",
            expected_len,
            pixels_f32.len()
        )));
    }

    let max_value = ((1 << target_depth) - 1) as f32;
    // Use Vec<u16> for proper alignment and native byte order
    let mut rgb_u16 = Vec::with_capacity(expected_len);

    if is_hdr {
        // HDR: Preserve extended range, apply PQ encoding
        println!(
            "AVIF: Preserving HDR with PQ encoding ({}-bit)",
            target_depth
        );
        for i in 0..pixel_count {
            let base = i * channels;
            let r = pixels_f32[base];
            let g = pixels_f32[base + 1];
            let b = pixels_f32[base + 2];

            // Apply PQ (ST.2084) EOTF inverse
            let r_pq = apply_pq_eotf_inverse(r);
            let g_pq = apply_pq_eotf_inverse(g);
            let b_pq = apply_pq_eotf_inverse(b);

            let r_u16 = (r_pq * max_value) as u16;
            let g_u16 = (g_pq * max_value) as u16;
            let b_u16 = (b_pq * max_value) as u16;

            rgb_u16.push(r_u16);
            rgb_u16.push(g_u16);
            rgb_u16.push(b_u16);

            if has_alpha {
                let a = pixels_f32[base + 3];
                let a_u16 = (a.clamp(0.0, 1.0) * max_value) as u16;
                rgb_u16.push(a_u16);
            }
        }
    } else {
        // SDR: Direct conversion (input is already in sRGB gamma space)
        for i in 0..pixel_count {
            let base = i * channels;
            let r = pixels_f32[base];
            let g = pixels_f32[base + 1];
            let b = pixels_f32[base + 2];

            let r_u16 = (r.clamp(0.0, 1.0) * max_value) as u16;
            let g_u16 = (g.clamp(0.0, 1.0) * max_value) as u16;
            let b_u16 = (b.clamp(0.0, 1.0) * max_value) as u16;

            rgb_u16.push(r_u16);
            rgb_u16.push(g_u16);
            rgb_u16.push(b_u16);

            if has_alpha {
                let a = pixels_f32[base + 3];
                let a_u16 = (a.clamp(0.0, 1.0) * max_value) as u16;
                rgb_u16.push(a_u16);
            }
        }
    }

    // Convert Vec<u16> to Vec<u8> for C API (keeps native byte order)
    let rgb_pixels = unsafe {
        let len = rgb_u16.len() * 2;
        let capacity = rgb_u16.capacity() * 2;
        let ptr = rgb_u16.as_mut_ptr() as *mut u8;
        std::mem::forget(rgb_u16);
        Vec::from_raw_parts(ptr, len, capacity)
    };

    let format = if has_alpha {
        libavif_sys::AVIF_RGB_FORMAT_RGBA
    } else {
        libavif_sys::AVIF_RGB_FORMAT_RGB
    };

    Ok((rgb_pixels, format, target_depth as i32))
}

/// Apply PQ (ST.2084) inverse EOTF for HDR encoding
/// Input: Linear light (0.0-10000.0 cd/m²), Output: PQ signal (0.0-1.0)
fn apply_pq_eotf_inverse(linear: f32) -> f32 {
    // Normalize to 0-1 range (assuming 10000 nits max)
    let normalized = (linear / 10000.0).clamp(0.0, 1.0);

    // PQ constants
    let m1 = 2610.0 / 16384.0;
    let m2 = 2523.0 / 4096.0 * 128.0;
    let c1 = 3424.0 / 4096.0;
    let c2 = 2413.0 / 4096.0 * 32.0;
    let c3 = 2392.0 / 4096.0 * 32.0;

    let y_m1 = normalized.powf(m1);
    let pq = ((c1 + c2 * y_m1) / (1.0 + c3 * y_m1)).powf(m2);

    pq.clamp(0.0, 1.0)
}

/// Convert linear RGB to sRGB gamma
fn linear_to_srgb(linear: f32) -> f32 {
    let clamped = linear.clamp(0.0, 1.0);
    if clamped <= 0.0031308 {
        clamped * 12.92
    } else {
        1.055 * clamped.powf(1.0 / 2.4) - 0.055
    }
}

/// Estimate AVIF file size
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

    // AVIF compression ratio estimation
    let quality_factor = options.quality / 100.0;
    let compression_ratio = 0.05 + (quality_factor * 0.15); // 5%-20% range

    (uncompressed_size as f64 * compression_ratio as f64) as usize
}
