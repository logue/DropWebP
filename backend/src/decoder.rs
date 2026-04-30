mod avif;
mod common;
mod heic;
mod jpeg2k;
mod jxl;

// Re-export IccProfileInfo and TransferFunction for use in other modules
pub use common::{IccProfileInfo, TransferFunction};

use crate::error::AppError;
use crate::options::HighBitDepthImage;
use image::{self, DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;
use std::path::Path;

/// Decode an image from a file path and return it as a `HighBitDepthImage`.
///
/// HEIC inputs are decoded with the OS-native API for HDR-capable 16-bit output.
///
/// # Arguments
/// - `path`: filesystem path to the source image.
///
/// # Returns
/// Tuple of decoded image and optional ICC profile bytes.
///
/// # Errors
/// Returns `AppError` when the file cannot be read or the format is unsupported.
// Used by the binary crate via `crate::decoder::decode_from_path`.
#[allow(dead_code)]
pub fn decode_from_path<P: AsRef<Path>>(
    path: P,
) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    let decode_start = std::time::Instant::now();

    // Read the file from disk.
    let data = std::fs::read(path.as_ref()).map_err(AppError::IoError)?;

    // Detect the image format.
    let format = detect_format(&data)
        .ok_or_else(|| AppError::Decode("Unsupported or unknown image format".to_string()))?;

    // For HEIC, use the OS-native API (HDR-capable 16-bit).
    if matches!(format, DetectedFormat::Heic) {
        println!("Decoder: Using OS-native HEIC decoder (HDR-capable)...");
        let path_ref = path.as_ref();
        let (mut img, icc_profile) = heic::decode_heic(path_ref)?;

        // Apply EXIF Orientation by rotating/flipping the image.
        if let Ok(exif_data) = std::fs::read(path_ref)
            && let Ok(exif_reader) =
                exif::Reader::new().read_from_container(&mut std::io::Cursor::new(&exif_data))
            && let Some(orientation_field) =
                exif_reader.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
            && let Some(orientation_value) = orientation_field.value.get_uint(0)
        {
            println!("HEIC: EXIF Orientation detected: {}", orientation_value);
            img = match orientation_value {
                1 => img,                     // Normal
                2 => img.fliph(),             // Flip horizontal
                3 => img.rotate180(),         // Rotate 180
                4 => img.flipv(),             // Flip vertical
                5 => img.rotate90().fliph(),  // Rotate 90 CW + flip horizontal
                6 => img.rotate90(),          // Rotate 90 CW
                7 => img.rotate270().fliph(), // Rotate 270 CW + flip horizontal
                8 => img.rotate270(),         // Rotate 270 CW
                _ => img,
            };
        }

        // HEIC is decoded into 16-bit RGBA (Rgba16); the original color space
        // (Display P3, PQ transfer function, etc.) is preserved as-is.
        let high_bit_img = match img {
            DynamicImage::ImageRgba16(rgba16) => {
                println!("HEIC: 16-bit image detected, preserving original color space");
                // Normalize 16-bit u16 values to f32 (0-65535 -> 0.0-1.0).
                // No color space conversion is performed; values are kept as-is.
                HighBitDepthImage::Rgba(DynamicImage::ImageRgba16(rgba16).to_rgba32f())
            }
            DynamicImage::ImageRgba8(rgba) => {
                // Fallback: 8-bit RGBA (SDR).
                println!("HEIC: 8-bit image, converting to f32");
                HighBitDepthImage::Rgba(image::DynamicImage::ImageRgba8(rgba).to_rgba32f())
            }
            _ => {
                println!("HEIC: Converting other format to f32");
                HighBitDepthImage::Rgba(img.to_rgba32f())
            }
        };

        println!(
            "Decoder: HEIC decoded in {:.2}s",
            decode_start.elapsed().as_secs_f64()
        );

        if let Some(ref profile) = icc_profile {
            println!("Decoder: Returning ICC profile ({} bytes)", profile.len());
        } else {
            println!("Decoder: No ICC profile available");
        }

        return Ok((high_bit_img, icc_profile));
    }

    // Other formats fall back to the regular `decode` function.
    let result = decode(&data);
    println!(
        "Decoder: Image decoded in {:.2}s",
        decode_start.elapsed().as_secs_f64()
    );
    result
}

/// Decode image bytes and return them as a `HighBitDepthImage`.
///
/// Supported formats: JPEG 2000, JPEG XL, and any format the `image` crate
/// can recognize.
///
/// # Arguments
/// - `image_bytes`: raw image bytes.
///
/// # Returns
/// Tuple of decoded image and optional ICC profile bytes.
///
/// # Errors
/// Returns `AppError::Decode` for unsupported, malformed, or HEIC inputs
/// (HEIC requires `decode_from_path`).
///
/// # Notes
/// - JPEG 2000 decoding uses the `jpeg2k` crate, which does not handle every
///   JPEG 2000 file; some inputs may fail.
/// - HEIC/HEIF decoding uses OS-native APIs:
///   - Windows: Windows Imaging Component (WIC)
///   - macOS: ImageIO framework
///   - Linux: `heif-convert` command (requires the `libheif-tools` package)
pub fn decode(image_bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    // First, identify the image format from the byte data.
    let format = detect_format(image_bytes)
        .ok_or_else(|| AppError::Decode("Unsupported or unknown image format".to_string()))?;

    // Dispatch to the appropriate decoder based on the detected format.
    match format {
        DetectedFormat::Avif => {
            println!("Decoder: Using AVIF decoder...");
            avif::decode(image_bytes)
        }
        DetectedFormat::Heic => {
            // HEIC requires a file path, so the caller must use `decode_from_path`.
            Err(AppError::Decode(
                "HEIC format requires file path. Use decode_from_path() instead.".to_string(),
            ))
        }
        DetectedFormat::Jpeg2000 => {
            println!("Decoder: Using Jpeg2k decoder...");
            jpeg2k::decode(image_bytes)
        }
        DetectedFormat::Jxl => {
            println!("Decoder: Using JPEG XL decoder...");
            jxl::decode(image_bytes)
        }
        DetectedFormat::Standard(_image_format) => {
            println!("Decoder: Using image decoder...");
            let icc_profile = extract_icc_profile(image_bytes);

            // 1. Load the bytes into a DynamicImage from memory.
            let img: DynamicImage = image::load_from_memory(image_bytes)
                .map_err(|e| AppError::Decode(e.to_string()))?;

            // 2. Inspect color type and bit depth.
            let color_type = img.color();
            let (width, height) = img.dimensions();

            println!("PNG: {}x{} - {:?}", width, height, color_type);

            // Detect wide-gamut content via ICC profile analysis.
            let has_wide_gamut_profile = if let Some(ref profile) = icc_profile {
                println!("PNG: ICC profile detected - size: {} bytes", profile.len());
                // Larger ICC profiles (Display P3, Rec2020, etc.) likely indicate wide gamut.
                profile.len() > 400 && profile.len() < 1000
            } else {
                false
            };

            // Bit depth decision: 16-bit formats, or 8-bit with a wide-gamut ICC profile.
            let requires_high_precision = match color_type {
                image::ColorType::L16 | image::ColorType::Rgb16 | image::ColorType::Rgba16 => {
                    println!("PNG: 16-bit image, using high-precision processing");
                    true
                }
                _ if has_wide_gamut_profile => {
                    println!(
                        "PNG: 8-bit image with wide-gamut ICC profile - using high-precision processing"
                    );
                    true
                }
                _ => {
                    println!("PNG: standard 8-bit image processing");
                    false
                }
            };

            // 3. Convert appropriately based on bit depth.
            if requires_high_precision {
                println!("PNG: running high-precision f32 conversion");
                match color_type {
                    // Formats without an alpha channel.
                    image::ColorType::L8
                    | image::ColorType::L16
                    | image::ColorType::Rgb8
                    | image::ColorType::Rgb16 => {
                        Ok((HighBitDepthImage::Rgb(img.to_rgb32f()), icc_profile))
                    }
                    // Formats that include an alpha channel.
                    _ => Ok((HighBitDepthImage::Rgba(img.to_rgba32f()), icc_profile)),
                }
            } else {
                println!("PNG: standard-precision processing (8-bit fast path)");
                // Standard 8-bit images: efficient conversion (avoid unnecessary high-precision).
                match color_type {
                    // Formats without an alpha channel.
                    image::ColorType::L8 | image::ColorType::Rgb8 => {
                        // Convert 8-bit data to f32 efficiently (range 0-1).
                        let rgb8_img = img.to_rgb8();
                        let pixels_f32: Vec<f32> = rgb8_img
                            .pixels()
                            .flat_map(|p| p.0.iter())
                            .map(|&x| x as f32 / 255.0)
                            .collect();

                        let buffer = image::ImageBuffer::<image::Rgb<f32>, _>::from_raw(
                            width, height, pixels_f32,
                        )
                        .ok_or_else(|| {
                            AppError::Decode("Failed to create RGB f32 buffer".to_string())
                        })?;

                        Ok((HighBitDepthImage::Rgb(buffer), icc_profile))
                    }
                    // 8-bit format with alpha channel.
                    image::ColorType::La8 | image::ColorType::Rgba8 => {
                        println!("PNG: efficient conversion for 8-bit RGBA image");
                        // Convert 8-bit data to f32 efficiently (range 0-1).
                        let rgba8_img = img.to_rgba8();
                        let pixels_f32: Vec<f32> = rgba8_img
                            .pixels()
                            .flat_map(|p| p.0.iter())
                            .map(|&x| x as f32 / 255.0)
                            .collect();

                        let buffer = image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(
                            width, height, pixels_f32,
                        )
                        .ok_or_else(|| {
                            AppError::Decode("Failed to create RGBA f32 buffer".to_string())
                        })?;

                        Ok((HighBitDepthImage::Rgba(buffer), icc_profile))
                    }
                    // Other types (16-bit, etc.) fall through to the general path.
                    _ => {
                        println!("PNG: fallback - high-precision conversion");
                        match color_type {
                            image::ColorType::L8
                            | image::ColorType::L16
                            | image::ColorType::Rgb8
                            | image::ColorType::Rgb16 => {
                                Ok((HighBitDepthImage::Rgb(img.to_rgb32f()), icc_profile))
                            }
                            _ => Ok((HighBitDepthImage::Rgba(img.to_rgba32f()), icc_profile)),
                        }
                    }
                }
            }
        }
    }
}

// Internal enumeration for the formats this decoder explicitly handles.
enum DetectedFormat {
    Avif,
    Heic,
    Jpeg2000,
    Jxl,
    // Any other format supported by the `image` crate.
    Standard(ImageFormat),
}

/// Detect the image format from the magic-number prefix of the byte data.
fn detect_format(bytes: &[u8]) -> Option<DetectedFormat> {
    // Check for HEIC/AVIF (ISOBMFF container).
    // Look for an ftyp box containing "heic", "heix", "avif", etc.
    if bytes.len() > 12 && &bytes[4..8] == b"ftyp" {
        let ftyp = &bytes[8..12];
        if ftyp == b"heic" || ftyp == b"heix" || ftyp == b"hevc" || ftyp == b"heim" {
            return Some(DetectedFormat::Heic);
        }
        // Detect AVIF.
        if ftyp == b"avif" || ftyp == b"avis" {
            return Some(DetectedFormat::Avif);
        }
    }

    // Check for JPEG 2000.
    if bytes.starts_with(b"\x00\x00\x00\x0CjP  \r\n\x87\n") {
        return Some(DetectedFormat::Jpeg2000);
    }

    // Check for JPEG XL.
    if bytes.starts_with(b"\xFF\x0A") || bytes.starts_with(b"\x00\x00\x00\x0CJXL ") {
        return Some(DetectedFormat::Jxl);
    }

    // Otherwise, defer to the `image` crate's format detection.
    if let Ok(format) = image::guess_format(bytes) {
        return Some(DetectedFormat::Standard(format));
    }

    None
}

/// Extract an embedded ICC profile from PNG or JPEG byte data.
///
/// # Arguments
/// - `bytes`: raw image bytes.
///
/// # Returns
/// `Some(profile)` when an ICC profile could be extracted, otherwise `None`.
pub fn extract_icc_profile(bytes: &[u8]) -> Option<Vec<u8>> {
    // --- PNG branch ---
    // Verify the PNG magic number (89 50 4E 47 0D 0A 1A 0A).
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        let decoder = png::Decoder::new(Cursor::new(bytes));
        if let Ok(reader) = decoder.read_info()
            && let Some(profile) = &reader.info().icc_profile
        {
            // Return the profile data from the iCCP chunk.
            return Some(profile.to_vec());
        }
        return None;
    }

    // --- JPEG branch ---
    // Verify the JPEG magic number (FF D8).
    if bytes.starts_with(&[0xFF, 0xD8]) {
        let mut icc_chunks = std::collections::BTreeMap::new();
        let mut pos = 2; // Begin scanning right after the SOI marker.

        while pos < bytes.len() - 4 {
            // Look for marker bytes (those starting with 0xFF).
            if bytes[pos] != 0xFF {
                pos += 1;
                continue;
            }

            let marker = bytes[pos + 1];

            // Check whether this is an APP2 marker (FF E2).
            if marker == 0xE2 {
                let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
                let segment_data = &bytes[pos + 4..pos + 2 + length];

                // Check for the "ICC_PROFILE" identifier.
                if segment_data.starts_with(b"ICC_PROFILE\0") {
                    // Parse the chunk metadata.
                    let chunk_index = segment_data[12];
                    let total_chunks = segment_data[13];
                    let profile_part = &segment_data[14..];

                    icc_chunks.insert(chunk_index, profile_part);

                    // Check whether all chunks have been collected.
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
            // Advance to the next marker.
            let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
            pos += 2 + length;
        }

        return None;
    }

    // Unsupported container.
    None
}
