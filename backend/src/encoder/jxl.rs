use crate::{encoder::HighBitDepthImage, error::AppError};
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr;

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
    pub fn to_jxl_speed(self) -> i32 {
        match self {
            Self::Lightning => 1,
            Self::Thunder => 2,
            Self::Falcon => 3,
            Self::Cheetah => 4,
            Self::Hare => 5,
            Self::Wombat => 6,
            Self::Squirrel => 7,
            Self::Kitten => 8,
            Self::Tortoise => 9,
            Self::Glacier => 10,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorEncoding {
    Srgb,
    LinearSrgb,
    SrgbLuma,
    LinearSrgbLuma,
}

struct EncoderGuard(*mut jxl_sys::JxlEncoder);

impl Drop for EncoderGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                jxl_sys::JxlEncoderDestroy(self.0);
            }
        }
    }
}

pub fn encode(
    img: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &JxlOptions,
) -> Result<Vec<u8>, AppError> {
    println!("JXL: Starting encode (jxl-sys)...");
    println!(
        "JXL: Options - lossless: {}, quality: {}, speed: {:?}",
        options.lossless, options.quality, options.speed
    );

    let (width, height, channels) = match img {
        HighBitDepthImage::Rgb(buf) => (buf.width(), buf.height(), 3),
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
            (buf.width(), buf.height(), 4)
        }
    };

    println!(
        "JXL: Image size: {}x{}, channels: {}",
        width, height, channels
    );

    // Range check for HDR content.
    let (max_value, min_value) = match img {
        HighBitDepthImage::Rgb(buf) => {
            let pixels = buf.as_raw();
            let max = pixels.iter().fold(0.0f32, |max, &v| max.max(v));
            let min = pixels.iter().fold(f32::MAX, |min, &v| min.min(v));
            (max, min)
        }
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
            let pixels = buf.as_raw();
            let max = pixels.iter().fold(0.0f32, |max, &v| max.max(v));
            let min = pixels.iter().fold(f32::MAX, |min, &v| min.min(v));
            (max, min)
        }
    };
    println!(
        "JXL: Input pixel value range: [{:.6}, {:.6}]",
        min_value, max_value
    );
    if max_value > 1.0 {
        println!(
            "JXL: HDR content detected! Max value: {:.3} ({}x SDR white)",
            max_value, max_value
        );
    }

    if let Some(ref profile) = icc_profile {
        println!("JXL: ICC profile available ({} bytes)", profile.len());

        // Check for the BT2020-PQ marker.
        if profile.len() >= 10 {
            let marker = &profile[profile.len() - 10..];
            if marker == b"\0BT2020-PQ\0" {
                println!("JXL: BT2020-PQ marker detected in ICC profile");
            }
        }
    }

    unsafe {
        let enc = jxl_sys::JxlEncoderCreate(ptr::null());
        if enc.is_null() {
            return Err(AppError::Jxr("Failed to create encoder".to_string()));
        }
        let _guard = EncoderGuard(enc);

        // 1. Set BasicInfo first (before the ICC profile).
        let mut basic_info = MaybeUninit::<jxl_sys::JxlBasicInfo>::uninit();
        jxl_sys::JxlEncoderInitBasicInfo(basic_info.as_mut_ptr());
        let mut basic_info = basic_info.assume_init();

        basic_info.xsize = width;
        basic_info.ysize = height;
        basic_info.bits_per_sample = 32;
        basic_info.exponent_bits_per_sample = 8;
        basic_info.alpha_bits = if channels == 4 { 32 } else { 0 };
        basic_info.alpha_exponent_bits = if channels == 4 { 8 } else { 0 };
        basic_info.num_extra_channels = if channels == 4 { 1 } else { 0 };
        // Force uses_original_profile to 1 when an ICC profile is provided.
        basic_info.uses_original_profile = if icc_profile.is_some() { 1 } else { 0 };

        println!(
            "JXL: Setting BasicInfo (uses_original_profile: {})",
            basic_info.uses_original_profile
        );
        let status = jxl_sys::JxlEncoderSetBasicInfo(enc, &basic_info);
        if status != jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS {
            return Err(AppError::Jxr(format!(
                "Failed to set basic info: {:?}",
                status
            )));
        }

        // 2. Set the ICC profile (after BasicInfo).
        if let Some(ref profile) = icc_profile {
            println!(
                "JXL: Setting ICC profile ({} bytes) AFTER BasicInfo",
                profile.len()
            );
            let status = jxl_sys::JxlEncoderSetICCProfile(enc, profile.as_ptr(), profile.len());
            if status == jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS {
                println!("JXL: ICC profile set successfully");
            } else {
                return Err(AppError::Jxr(format!(
                    "Failed to set ICC profile: {:?}",
                    status
                )));
            }
        }

        // 3. Set the color encoding (only when no ICC profile is provided).
        if icc_profile.is_none() {
            let mut color_encoding = MaybeUninit::<jxl_sys::JxlColorEncoding>::uninit();
            jxl_sys::JxlColorEncodingSetToSRGB(
                color_encoding.as_mut_ptr(),
                if channels == 3 { 0 } else { 1 },
            );
            let color_encoding = color_encoding.assume_init();

            let status = jxl_sys::JxlEncoderSetColorEncoding(enc, &color_encoding);
            if status != jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS {
                println!("JXL: Warning - failed to set color encoding: {:?}", status);
            }
        } else {
            println!("JXL: Using ICC profile for color encoding (uses_original_profile=1)");
        }

        let frame_settings = jxl_sys::JxlEncoderFrameSettingsCreate(enc, ptr::null());
        if frame_settings.is_null() {
            return Err(AppError::Jxr("Failed to create frame settings".to_string()));
        }

        if options.lossless {
            jxl_sys::JxlEncoderSetFrameLossless(frame_settings, 1);
            println!("JXL: Using lossless mode");
        } else {
            jxl_sys::JxlEncoderSetFrameDistance(frame_settings, options.quality);
            println!("JXL: Using lossy mode with quality: {}", options.quality);
        }

        jxl_sys::JxlEncoderFrameSettingsSetOption(
            frame_settings,
            jxl_sys::JxlEncoderFrameSettingId::JXL_ENC_FRAME_SETTING_EFFORT,
            options.speed.to_jxl_speed() as i64,
        );

        jxl_sys::JxlEncoderFrameSettingsSetOption(
            frame_settings,
            jxl_sys::JxlEncoderFrameSettingId::JXL_ENC_FRAME_SETTING_DECODING_SPEED,
            options.decoding_speed,
        );

        jxl_sys::JxlEncoderUseContainer(enc, if options.use_container { 1 } else { 0 });

        let pixel_format = jxl_sys::JxlPixelFormat {
            num_channels: channels,
            data_type: jxl_sys::JxlDataType::JXL_TYPE_FLOAT,
            endianness: jxl_sys::JxlEndianness::JXL_NATIVE_ENDIAN,
            align: 0,
        };

        let (data_ptr, data_size) = match img {
            HighBitDepthImage::Rgb(buf) => {
                let pixels = buf.as_raw();
                (pixels.as_ptr() as *const c_void, pixels.len() * 4)
            }
            HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
                let pixels = buf.as_raw();
                (pixels.as_ptr() as *const c_void, pixels.len() * 4)
            }
        };

        let status =
            jxl_sys::JxlEncoderAddImageFrame(frame_settings, &pixel_format, data_ptr, data_size);
        if status != jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS {
            return Err(AppError::Jxr(format!(
                "Failed to add image frame: {:?}",
                status
            )));
        }

        jxl_sys::JxlEncoderCloseInput(enc);

        // Compute a reasonable initial buffer size.
        let estimated_size = estimate_size(width, height, channels, options.quality);
        println!(
            "JXL: Estimated output size: {} bytes ({:.1} KB)",
            estimated_size,
            estimated_size as f32 / 1024.0
        );
        let mut output = Vec::with_capacity(estimated_size);
        let mut next_out = output.as_mut_ptr();
        let mut avail_out = output.capacity();

        loop {
            let status = jxl_sys::JxlEncoderProcessOutput(enc, &mut next_out, &mut avail_out);

            match status {
                jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS => {
                    let encoded_size = output.capacity() - avail_out;
                    output.set_len(encoded_size);
                    println!("JXL: Successfully encoded {} bytes", encoded_size);
                    break;
                }
                jxl_sys::JxlEncoderStatus::JXL_ENC_NEED_MORE_OUTPUT => {
                    let offset = output.capacity() - avail_out;
                    // Reflect the bytes already written into len.
                    output.set_len(offset);

                    // Double the capacity (add at least 64KB).
                    let additional = output.capacity().max(64 * 1024);
                    output.reserve(additional);

                    // Refresh the pointer and remaining capacity.
                    next_out = output.as_mut_ptr().add(offset);
                    avail_out = output.capacity() - offset;
                    println!(
                        "JXL: Need more output buffer (written: {} bytes, new capacity: {} bytes)",
                        offset,
                        output.capacity()
                    );
                }
                _ => {
                    return Err(AppError::Jxr(format!("Encoding failed: {:?}", status)));
                }
            }
        }

        Ok(output)
    }
}

// Used by the binary crate via `crate::encoder::jxl::transcode`.
#[allow(dead_code)]
pub fn transcode(jpeg_data: &[u8]) -> Result<Vec<u8>, AppError> {
    println!("JXL: Starting JPEG transcode...");

    unsafe {
        let enc = jxl_sys::JxlEncoderCreate(ptr::null());
        if enc.is_null() {
            return Err(AppError::Jxr("Failed to create encoder".to_string()));
        }
        let _guard = EncoderGuard(enc);

        jxl_sys::JxlEncoderUseContainer(enc, 1);
        jxl_sys::JxlEncoderStoreJPEGMetadata(enc, 1);

        let frame_settings = jxl_sys::JxlEncoderFrameSettingsCreate(enc, ptr::null());
        if frame_settings.is_null() {
            return Err(AppError::Jxr("Failed to create frame settings".to_string()));
        }

        let status =
            jxl_sys::JxlEncoderAddJPEGFrame(frame_settings, jpeg_data.as_ptr(), jpeg_data.len());
        if status != jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS {
            return Err(AppError::Jxr(format!(
                "Failed to add JPEG frame: {:?}",
                status
            )));
        }

        jxl_sys::JxlEncoderCloseInput(enc);

        let mut output = Vec::with_capacity(jpeg_data.len());
        let mut next_out = output.as_mut_ptr();
        let mut avail_out = output.capacity();

        loop {
            let status = jxl_sys::JxlEncoderProcessOutput(enc, &mut next_out, &mut avail_out);

            match status {
                jxl_sys::JxlEncoderStatus::JXL_ENC_SUCCESS => {
                    let encoded_size = output.capacity() - avail_out;
                    output.set_len(encoded_size);
                    println!("JXL: Successfully transcoded {} bytes", encoded_size);
                    break;
                }
                jxl_sys::JxlEncoderStatus::JXL_ENC_NEED_MORE_OUTPUT => {
                    let offset = output.capacity() - avail_out;
                    // Reflect the bytes already written into len.
                    output.set_len(offset);

                    // Double the capacity (add at least 64KB).
                    let additional = output.capacity().max(64 * 1024);
                    output.reserve(additional);

                    // Refresh the pointer and remaining capacity.
                    next_out = output.as_mut_ptr().add(offset);
                    avail_out = output.capacity() - offset;
                }
                _ => {
                    return Err(AppError::Jxr(format!("Transcode failed: {:?}", status)));
                }
            }
        }

        Ok(output)
    }
}

pub fn estimate_size(width: u32, height: u32, channels: u32, quality: f32) -> usize {
    let pixels = (width * height) as usize;
    let base_size = pixels * channels as usize;

    // JXL distance: smaller values mean higher quality (and larger files).
    // distance 0.0 = lossless
    // distance 1.0 = visually lossless (~35-40% of base)
    // distance 3.0 = high quality (~70% of base)
    // distance 7.0 = standard quality (~15% of base)

    let estimated_ratio = if quality < 0.5 {
        // Lossless approximation: 50-80%.
        0.65
    } else if quality < 1.5 {
        // Visually lossless: 30-45%
        0.38
    } else if quality < 4.0 {
        // High quality: 40-70%
        0.55
    } else {
        // Standard quality: 10-20%
        0.15
    };

    // Add a 1.5x safety factor.
    let estimated = (base_size as f32 * estimated_ratio * 1.5) as usize;

    // Clamp between 256KB and 16MB.
    estimated.clamp(256 * 1024, 16 * 1024 * 1024)
}
