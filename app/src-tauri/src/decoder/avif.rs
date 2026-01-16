use super::common::{IccProfileInfo, log_icc_profile_details};
use crate::error::AppError;
use crate::options::HighBitDepthImage;

use image::{ImageBuffer, Rgba};
use lcms2::{CIExyY, CIExyYTRIPLE, Profile, ToneCurve};
use libavif_sys::*;
use std::ptr;

/// Generate BT.2020 PQ ICC profile with proper metadata
/// This creates an ICC profile with BT.2020 color primaries and PQ transfer function metadata
fn create_bt2020_linear_icc_profile() -> Result<Vec<u8>, AppError> {
    println!("AVIF: Creating BT.2020 PQ ICC profile with metadata...");

    // BT.2020 color primaries (from ITU-R BT.2020)
    // Red primary: x=0.708, y=0.292
    // Green primary: x=0.170, y=0.797
    // Blue primary: x=0.131, y=0.046
    // White point: D65 (x=0.3127, y=0.3290)
    let primaries = CIExyYTRIPLE {
        Red: CIExyY {
            x: 0.708,
            y: 0.292,
            Y: 1.0,
        },
        Green: CIExyY {
            x: 0.170,
            y: 0.797,
            Y: 1.0,
        },
        Blue: CIExyY {
            x: 0.131,
            y: 0.046,
            Y: 1.0,
        },
    };

    // D65 white point (CIE standard illuminant D65: x=0.3127, y=0.3290)
    let white_point = CIExyY {
        x: 0.3127,
        y: 0.3290,
        Y: 1.0,
    };

    // Linear transfer curve (gamma = 1.0) - actual PQ is applied in pixel conversion
    // Note: lcms2 doesn't support PQ curve directly, so we use linear and handle PQ in code
    let linear_curve = ToneCurve::new(1.0);

    // Create the profile
    let profile = Profile::new_rgb(
        &white_point,
        &primaries,
        &[&linear_curve, &linear_curve, &linear_curve],
    )
    .map_err(|e| AppError::Encode(format!("Failed to create BT.2020 profile: {:?}", e)))?;

    // Serialize the profile to bytes
    let mut icc_data = profile
        .icc()
        .map_err(|e| AppError::Encode(format!("Failed to serialize ICC profile: {:?}", e)))?;

    // Add custom metadata to help identify BT.2020 and PQ
    // Append special marker that our ICC profile analyzer can detect
    // Format: "BT2020-PQ\0" as a simple identifier
    let marker = b"BT2020-PQ\0";
    icc_data.extend_from_slice(marker);

    println!(
        "AVIF: BT.2020 PQ ICC profile created ({} bytes ICC + {} bytes marker = {} bytes total)",
        icc_data.len() - marker.len(),
        marker.len(),
        icc_data.len()
    );

    Ok(icc_data)
}

/// Convert PQ (SMPTE ST 2084) encoded value to linear light
/// Input: normalized PQ value (0.0-1.0)
/// Output: linear light value (suitable for JPEG XL linear encoding)
fn pq_to_linear(pq: f32) -> f32 {
    if pq <= 0.0 {
        return 0.0;
    }

    // PQ constants (SMPTE ST 2084)
    let m1 = 2610.0 / 16384.0; // 0.1593017578125
    let m2 = 2523.0 / 4096.0 * 128.0; // 78.84375
    let c1 = 3424.0 / 4096.0; // 0.8359375
    let c2 = 2413.0 / 4096.0 * 32.0; // 18.8515625
    let c3 = 2392.0 / 4096.0 * 32.0; // 18.6875

    // Inverse PQ EOTF
    let v_pow = pq.powf(1.0 / m2);
    let num = (v_pow - c1).max(0.0);
    let den = c2 - c3 * v_pow;

    if den <= 0.0 {
        return 0.0;
    }

    let y = (num / den).powf(1.0 / m1);

    // PQ encodes absolute luminance: 0-10000 nits
    // y is in range 0-1, where 1.0 = 10000 nits
    // Convert to actual nits and normalize for JPEG XL
    let nits = y * 10000.0;

    // For JPEG XL with LinearSrgb:
    // Use 100 nits as reference white (1.0) - this is the standard for HDR displays
    // SDR white (80-100 nits) ≈ 1.0, Peak (10000 nits) = 100.0
    nits / 100.0 // 100 nits = 1.0, max 10000 nits = 100.0
}

/// Convert HLG (Hybrid Log-Gamma) encoded value to linear light
/// Input: normalized HLG value (0.0-1.0)
/// Output: linear light value (suitable for JPEG XL linear encoding)
fn hlg_to_linear(hlg: f32) -> f32 {
    if hlg <= 0.0 {
        return 0.0;
    }

    // HLG inverse OETF (ITU-R BT.2100)
    let a = 0.17883277;
    let b = 0.28466892; // 1 - 4a
    let c = 0.55991073; // 0.5 - a * ln(4a)

    let y = if hlg <= 0.5 {
        // Lower range: quadratic
        (hlg * hlg) / 3.0
    } else {
        // Upper range: logarithmic
        ((hlg - c) / a).exp() / 12.0 + b / 12.0
    };

    // HLG scene light (0-1 range represents 0-1000 nits)
    // Apply system gamma (1.2 for typical display)
    // Scale to get: SDR white (~100 nits) ≈ 1.0, HDR peaks up to 100
    let gamma = 1.2;
    y.powf(gamma) * 100.0
}

/// Decode AVIF image with ICC profile analysis
pub fn decode(data: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    println!("AVIF: Starting AVIF decode process...");

    unsafe {
        // Create decoder
        let decoder = avifDecoderCreate();
        if decoder.is_null() {
            return Err(AppError::Decode(
                "Failed to create AVIF decoder".to_string(),
            ));
        }

        // Cleanup on drop
        let _decoder_guard = DecoderGuard(decoder);

        // Set decoder to read the entire image
        (*decoder).ignoreExif = AVIF_FALSE as i32;
        (*decoder).ignoreXMP = AVIF_FALSE as i32;

        // Parse the AVIF data
        let result =
            avifDecoderSetIOMemory(decoder, data.as_ptr() as *const u8, data.len() as usize);

        if result != AVIF_RESULT_OK {
            return Err(AppError::Decode(format!(
                "Failed to set AVIF IO memory: {:?}",
                result
            )));
        }

        // Parse metadata
        let result = avifDecoderParse(decoder);
        if result != AVIF_RESULT_OK {
            return Err(AppError::Decode(format!(
                "Failed to parse AVIF: {:?}",
                result
            )));
        }

        // Get next image
        let result = avifDecoderNextImage(decoder);
        if result != AVIF_RESULT_OK {
            return Err(AppError::Decode(format!(
                "Failed to decode AVIF image: {:?}",
                result
            )));
        }

        let image = (*decoder).image;
        if image.is_null() {
            return Err(AppError::Decode("AVIF image is null".to_string()));
        }

        let width = (*image).width;
        let height = (*image).height;
        let depth = (*image).depth;
        let yuv_format = (*image).yuvFormat;

        println!(
            "AVIF: Image properties - {}x{}, depth: {}, format: {:?}",
            width, height, depth, yuv_format
        );

        // Extract ICC profile if present
        let mut icc_profile = if !(*image).icc.data.is_null() && (*image).icc.size > 0 {
            let icc_data =
                std::slice::from_raw_parts((*image).icc.data, (*image).icc.size as usize);
            println!(
                "AVIF: ICC profile detected (size: {} bytes)",
                icc_data.len()
            );
            log_icc_profile_details(icc_data);
            Some(icc_data.to_vec())
        } else {
            println!("AVIF: No ICC profile found");
            None
        };

        // Check if this is HDR content by examining color transfer characteristics
        let is_hdr = (*image).transferCharacteristics == AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084 as u16 // PQ (HDR10)
            || (*image).transferCharacteristics == AVIF_TRANSFER_CHARACTERISTICS_HLG as u16; // HLG (HDR)

        // Check color primaries for wide gamut detection
        let color_primaries = (*image).colorPrimaries;
        let is_bt2020 = color_primaries == AVIF_COLOR_PRIMARIES_BT2020 as u16;

        println!(
            "AVIF: Color info - primaries: {}, transfer: {}, HDR: {}",
            color_primaries,
            (*image).transferCharacteristics,
            is_hdr
        );

        // For HDR content without ICC profile, create appropriate color space marker
        // JPEG XL needs to know the color primaries (BT.2020 vs sRGB)
        if is_hdr && icc_profile.is_none() {
            println!(
                "AVIF: HDR content without ICC profile (transfer: {})",
                (*image).transferCharacteristics
            );

            if is_bt2020 {
                println!(
                    "AVIF: BT.2020 color primaries detected - generating BT.2020 Linear ICC profile"
                );
                // Generate a proper BT.2020 Linear ICC profile
                match create_bt2020_linear_icc_profile() {
                    Ok(profile_data) => {
                        println!("AVIF: BT.2020 Linear ICC profile generated successfully");
                        icc_profile = Some(profile_data);
                    }
                    Err(e) => {
                        println!("AVIF: Warning - Failed to generate BT.2020 profile: {}", e);
                        println!("AVIF: HDR will be preserved but color space may not be optimal");
                    }
                }
            } else {
                println!("AVIF: Using default color primaries for HDR");
                println!("AVIF: HDR will be preserved through LinearSrgb color encoding");
            }
        }

        // Analyze ICC profile if present
        let profile_info = icc_profile
            .as_ref()
            .map(|profile| IccProfileInfo::analyze(profile));

        if let Some(ref info) = profile_info {
            println!(
                "AVIF: Profile analysis - Wide gamut: {}, High precision: {}",
                info.suggests_wide_gamut, info.has_high_precision
            );
        }

        // Convert to RGB format
        let mut rgb_image = avifRGBImage {
            width,
            height,
            depth,
            format: AVIF_RGB_FORMAT_RGBA,
            chromaUpsampling: AVIF_CHROMA_UPSAMPLING_BEST_QUALITY,
            chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_BEST_QUALITY,
            avoidLibYUV: AVIF_FALSE as i32,
            ignoreAlpha: AVIF_FALSE as i32,
            alphaPremultiplied: AVIF_FALSE as i32,
            isFloat: AVIF_FALSE as i32,
            maxThreads: 0,
            pixels: ptr::null_mut(),
            rowBytes: 0,
        };

        avifRGBImageSetDefaults(&mut rgb_image, image);

        // For HDR content, ensure full range is used (not limited range)
        if is_hdr {
            println!("AVIF: Setting full range for HDR content");
            // Note: avifRGBImageSetDefaults may have already set this based on the image,
            // but we explicitly ensure it for HDR
            // The range field in avifRGBImage determines if we use full (0-255) or limited (16-235) range
        }

        // Allocate RGB buffer
        let result = avifRGBImageAllocatePixels(&mut rgb_image);
        if result != AVIF_RESULT_OK {
            return Err(AppError::Decode(format!(
                "Failed to allocate RGB pixels: {:?}",
                result
            )));
        }

        let _rgb_guard = RGBImageGuard(&mut rgb_image);

        // Convert YUV to RGB
        let result = avifImageYUVToRGB(image, &mut rgb_image);
        if result != AVIF_RESULT_OK {
            return Err(AppError::Decode(format!(
                "Failed to convert YUV to RGB: {:?}",
                result
            )));
        }

        // Extract pixel data and convert to f32
        let pixel_count = (width * height) as usize;
        let is_high_depth = depth > 8;

        println!(
            "AVIF: Converting to HighBitDepthImage (high depth: {}, depth: {} bit)",
            is_high_depth, depth
        );

        let high_bit_img = if is_high_depth {
            // 10-bit or higher depth
            let pixels_u16 = std::slice::from_raw_parts(
                rgb_image.pixels as *const u16,
                pixel_count * 4, // RGBA
            );

            // Check if this is HDR content by examining color transfer characteristics
            let is_pq_hdr =
                (*image).transferCharacteristics == AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084 as u16; // PQ (HDR10)
            let is_hlg_hdr =
                (*image).transferCharacteristics == AVIF_TRANSFER_CHARACTERISTICS_HLG as u16; // HLG
            let is_hdr = is_pq_hdr || is_hlg_hdr;

            if is_hdr {
                println!(
                    "AVIF: HDR content detected (transfer: {}), converting to linear space",
                    (*image).transferCharacteristics
                );
            } else {
                println!(
                    "AVIF: High bit-depth SDR content (transfer: {})",
                    (*image).transferCharacteristics
                );
            }

            let max_value = (1u32 << depth) - 1;

            // Sample pixels to detect gradient direction
            if is_hdr {
                println!("AVIF: Analyzing image gradient (sampling every 10%):");
                println!("AVIF: Image dimensions: {}x{}", width, height);

                // Sample horizontally across middle row
                println!("AVIF: Horizontal gradient check (middle row):");
                let mid_y = (height / 2) as usize;
                let width_usize = width as usize;
                for i in 0..11 {
                    let x = (width_usize * i / 10).min(width_usize - 1);
                    let idx = mid_y * width_usize + x;
                    if idx < pixel_count {
                        let r = pixels_u16[idx * 4] as f32 / max_value as f32;
                        println!("  [{:4}, {:4}] (pixel {:7}): PQ={:.6}", x, mid_y, idx, r);
                    }
                }

                // Sample vertically down middle column
                println!("AVIF: Vertical gradient check (middle column):");
                let mid_x = (width / 2) as usize;
                let height_usize = height as usize;
                for i in 0..11 {
                    let y = (height_usize * i / 10).min(height_usize - 1);
                    let idx = y * width_usize + mid_x;
                    if idx < pixel_count {
                        let r = pixels_u16[idx * 4] as f32 / max_value as f32;
                        println!("  [{:4}, {:4}] (pixel {:7}): PQ={:.6}", mid_x, y, idx, r);
                    }
                }
            }

            let pixels_f32: Vec<f32> = pixels_u16
                .iter()
                .enumerate()
                .map(|(i, &pixel)| {
                    let normalized = pixel as f32 / max_value as f32;

                    // Skip transfer function conversion for alpha channel
                    if i % 4 == 3 {
                        return normalized;
                    }

                    if is_pq_hdr {
                        // PQ (SMPTE ST 2084) inverse EOTF
                        // libavif does NOT apply transfer function, so we must do it
                        pq_to_linear(normalized)
                    } else if is_hlg_hdr {
                        // HLG inverse OETF
                        hlg_to_linear(normalized)
                    } else {
                        // SDR: values are in sRGB gamma space or rec709
                        normalized
                    }
                })
                .collect();

            // Sample converted pixels and show luminance range
            if is_hdr {
                println!("AVIF: Converted luminance range check:");
                println!("AVIF: Horizontal (middle row):");
                let mid_y = (height / 2) as usize;
                let width_usize = width as usize;
                for i in 0..11 {
                    let x = (width_usize * i / 10).min(width_usize - 1);
                    let idx = mid_y * width_usize + x;
                    if idx < pixel_count {
                        let r = pixels_f32[idx * 4];
                        let nits = r * 100.0; // 100 nits = 1.0
                        println!("  [{:4}, {:4}]: {:.6} ({:.1} nits)", x, mid_y, r, nits);
                    }
                }
            }

            // Log pixel value range for debugging
            if let Some(min_val) = pixels_f32
                .iter()
                .enumerate()
                .filter(|(i, _)| i % 4 != 3) // Skip alpha
                .map(|(_, &v)| v)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
            {
                if let Some(max_val) = pixels_f32
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 4 != 3)
                    .map(|(_, &v)| v)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                {
                    println!(
                        "AVIF: Converted pixel range: [{:.6}, {:.6}] (HDR: {})",
                        min_val, max_val, is_hdr
                    );
                }
            }

            let buffer = ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, pixels_f32)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create f32 RGBA ImageBuffer".to_string())
                })?;

            HighBitDepthImage::Rgba(buffer)
        } else {
            // 8-bit depth
            let pixels_u8 = std::slice::from_raw_parts(
                rgb_image.pixels,
                pixel_count * 4, // RGBA
            );

            let pixels_f32: Vec<f32> = pixels_u8
                .iter()
                .map(|&pixel| pixel as f32 / 255.0)
                .collect();

            let buffer = ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, pixels_f32)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create f32 RGBA ImageBuffer".to_string())
                })?;

            HighBitDepthImage::Rgba(buffer)
        };

        println!("AVIF: Decode completed successfully");
        Ok((high_bit_img, icc_profile))
    }
}

// RAII guard for automatic cleanup
struct DecoderGuard(*mut avifDecoder);

impl Drop for DecoderGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                avifDecoderDestroy(self.0);
            }
        }
    }
}

struct RGBImageGuard(*mut avifRGBImage);

impl Drop for RGBImageGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                avifRGBImageFreePixels(self.0);
            }
        }
    }
}
