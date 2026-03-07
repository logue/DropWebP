use super::common::{IccProfileInfo, log_icc_profile_details};
use crate::error::AppError;
use crate::options::HighBitDepthImage;

use image::{ImageBuffer, Rgb, Rgba};
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr;

struct DecoderGuard(*mut jxl_sys::JxlDecoder);

impl Drop for DecoderGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                jxl_sys::JxlDecoderDestroy(self.0);
            }
        }
    }
}

/// Decode JPEG XL image with ICC profile analysis
pub fn decode(data: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    println!("JXL: Starting JPEG XL decode process (jxl-sys)...");

    unsafe {
        let dec = jxl_sys::JxlDecoderCreate(ptr::null());
        if dec.is_null() {
            return Err(AppError::Decode("Failed to create JXL decoder".to_string()));
        }
        let _guard = DecoderGuard(dec);

        let events = jxl_sys::JxlDecoderStatus::JXL_DEC_BASIC_INFO as i32
            | jxl_sys::JxlDecoderStatus::JXL_DEC_COLOR_ENCODING as i32
            | jxl_sys::JxlDecoderStatus::JXL_DEC_FULL_IMAGE as i32;

        let status = jxl_sys::JxlDecoderSubscribeEvents(dec, events);
        if status != jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS {
            return Err(AppError::Decode(format!(
                "Failed to subscribe to events: {:?}",
                status
            )));
        }

        jxl_sys::JxlDecoderSetInput(dec, data.as_ptr(), data.len());
        jxl_sys::JxlDecoderCloseInput(dec);

        let mut basic_info = MaybeUninit::<jxl_sys::JxlBasicInfo>::uninit();
        let mut icc_profile: Option<Vec<u8>> = None;
        let mut pixel_data: Option<Vec<f32>> = None;
        let mut width = 0u32;
        let mut height = 0u32;
        let mut num_channels = 0u32;

        loop {
            let status = jxl_sys::JxlDecoderProcessInput(dec);

            match status {
                jxl_sys::JxlDecoderStatus::JXL_DEC_BASIC_INFO => {
                    let info_status = jxl_sys::JxlDecoderGetBasicInfo(dec, basic_info.as_mut_ptr());
                    if info_status != jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS {
                        return Err(AppError::Decode(format!(
                            "Failed to get basic info: {:?}",
                            info_status
                        )));
                    }

                    let basic_info = basic_info.assume_init();
                    width = basic_info.xsize;
                    height = basic_info.ysize;
                    num_channels = if basic_info.num_extra_channels > 0 {
                        4
                    } else {
                        3
                    };

                    println!(
                        "JXL: Image size: {}x{}, channels: {}",
                        width, height, num_channels
                    );
                }

                jxl_sys::JxlDecoderStatus::JXL_DEC_COLOR_ENCODING => {
                    let mut icc_size: usize = 0;
                    let size_status = jxl_sys::JxlDecoderGetICCProfileSize(
                        dec,
                        jxl_sys::JxlColorProfileTarget::JXL_COLOR_PROFILE_TARGET_DATA,
                        &mut icc_size,
                    );

                    if size_status == jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS && icc_size > 0 {
                        let mut icc_buffer = vec![0u8; icc_size];
                        let get_status = jxl_sys::JxlDecoderGetColorAsICCProfile(
                            dec,
                            jxl_sys::JxlColorProfileTarget::JXL_COLOR_PROFILE_TARGET_DATA,
                            icc_buffer.as_mut_ptr(),
                            icc_size,
                        );

                        if get_status == jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS {
                            println!("JXL: ICC profile extracted ({} bytes)", icc_size);
                            log_icc_profile_details(&icc_buffer);
                            let profile_info = IccProfileInfo::analyze(&icc_buffer);
                            println!(
                                "JXL: Profile analysis - Wide gamut: {}, High precision: {}, BT.2020: {}",
                                profile_info.suggests_wide_gamut,
                                profile_info.has_high_precision,
                                profile_info.is_bt2020()
                            );
                            icc_profile = Some(icc_buffer);
                        }
                    }
                }

                jxl_sys::JxlDecoderStatus::JXL_DEC_NEED_IMAGE_OUT_BUFFER => {
                    let pixel_format = jxl_sys::JxlPixelFormat {
                        num_channels,
                        data_type: jxl_sys::JxlDataType::JXL_TYPE_FLOAT,
                        endianness: jxl_sys::JxlEndianness::JXL_NATIVE_ENDIAN,
                        align: 0,
                    };

                    let mut buffer_size: usize = 0;
                    let size_status =
                        jxl_sys::JxlDecoderImageOutBufferSize(dec, &pixel_format, &mut buffer_size);
                    if size_status != jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS {
                        return Err(AppError::Decode(format!(
                            "Failed to get buffer size: {:?}",
                            size_status
                        )));
                    }

                    let num_floats = buffer_size / std::mem::size_of::<f32>();
                    let mut buffer = vec![0.0f32; num_floats];

                    let set_status = jxl_sys::JxlDecoderSetImageOutBuffer(
                        dec,
                        &pixel_format,
                        buffer.as_mut_ptr() as *mut c_void,
                        buffer_size,
                    );
                    if set_status != jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS {
                        return Err(AppError::Decode(format!(
                            "Failed to set output buffer: {:?}",
                            set_status
                        )));
                    }

                    pixel_data = Some(buffer);
                }

                jxl_sys::JxlDecoderStatus::JXL_DEC_FULL_IMAGE => {
                    println!("JXL: Full image decoded");
                }

                jxl_sys::JxlDecoderStatus::JXL_DEC_SUCCESS => {
                    println!("JXL: Decoding complete");
                    break;
                }

                _ => {
                    return Err(AppError::Decode(format!("Decoder error: {:?}", status)));
                }
            }
        }

        let buffer_f32 = pixel_data
            .ok_or_else(|| AppError::Decode("No pixel data received from decoder".to_string()))?;

        let max_value = buffer_f32.iter().fold(0.0f32, |max, &v| max.max(v));
        let min_value = buffer_f32.iter().fold(f32::MAX, |min, &v| min.min(v));
        println!(
            "JXL: Decoded pixel value range: [{:.3}, {:.3}]",
            min_value, max_value
        );
        if max_value > 1.0 {
            println!(
                "JXL: HDR content detected in decoded data (max: {:.3})",
                max_value
            );
        }

        let image_buffer = match num_channels {
            3 => {
                let buffer = ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                    .ok_or_else(|| {
                        AppError::Decode("Failed to create f32 RGB ImageBuffer".to_string())
                    })?;
                HighBitDepthImage::Rgb(buffer)
            }
            4 => {
                let buffer =
                    ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                        .ok_or_else(|| {
                            AppError::Decode("Failed to create f32 RGBA ImageBuffer".to_string())
                        })?;
                HighBitDepthImage::Rgba(buffer)
            }
            _ => {
                return Err(AppError::Decode(format!(
                    "Unsupported channel count: {}",
                    num_channels
                )));
            }
        };

        Ok((image_buffer, icc_profile))
    }
}
