/// HEIC/HEIF デコーダー（OS標準API使用、HDR対応）
/// - Windows: Windows Imaging Component (WIC) - 64bppRGBA
/// - macOS: ImageIO framework - 16-bit per channel
/// - Linux: heif-convert コマンド - 16-bit PNG
use crate::error::AppError;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::path::Path;

#[cfg(target_os = "windows")]
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<DynamicImage, AppError> {
    use std::ptr;
    use windows::{Win32::Graphics::Imaging::*, Win32::System::Com::*, core::*};

    unsafe {
        // COMの初期化
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        // WICファクトリーの作成
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;

        // デコーダーの作成
        let path_str = path.as_ref().to_str().ok_or(AppError::PathConversion)?;
        let wide_path: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();

        let decoder = factory.CreateDecoderFromFilename(
            PCWSTR(wide_path.as_ptr()),
            ptr::null(),
            GENERIC_READ.0,
            WICDecodeMetadataCacheOnDemand,
        )?;

        // 最初のフレームを取得
        let frame = decoder.GetFrame(0)?;

        // サイズを取得
        let mut width = 0u32;
        let mut height = 0u32;
        frame.GetSize(&mut width, &mut height)?;

        // RGBA64ビット（16-bit/ch）に変換するためのコンバーターを作成（HDR対応）
        let converter: IWICFormatConverter = factory.CreateFormatConverter()?;
        converter.Initialize(
            &frame,
            &GUID_WICPixelFormat64bppRGBA,
            WICBitmapDitherTypeNone,
            None,
            0.0,
            WICBitmapPaletteTypeCustom,
        )?;

        // ピクセルデータを読み込む
        let stride = width * 8; // 8 bytes per pixel (RGBA 16-bit/ch)
        let buffer_size = (stride * height) as usize;
        let mut buffer = vec![0u8; buffer_size];

        converter.CopyPixels(ptr::null(), stride, &mut buffer)?;

        // COMのクリーンアップ
        CoUninitialize();

        // u8バッファをu16に変換（リトルエンディアン）
        let mut rgba16_buffer = Vec::with_capacity((width * height * 4) as usize);
        for chunk in buffer.chunks_exact(2) {
            let value = u16::from_le_bytes([chunk[0], chunk[1]]);
            rgba16_buffer.push(value);
        }

        // image::DynamicImageに変換（16-bit）
        let img_buffer = ImageBuffer::<Rgba<u16>, Vec<u16>>::from_raw(width, height, rgba16_buffer)
            .ok_or(AppError::ImageConversion)?;

        Ok(DynamicImage::ImageRgba16(img_buffer))
    }
}

#[cfg(target_os = "macos")]
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<DynamicImage, AppError> {
    use core_foundation::base::TCFType;
    use core_foundation::data::CFData;
    use std::os::raw::c_void;
    use std::ptr;

    // C型定義
    type CGImageSourceRef = *const c_void;
    type CGImageRef = *const c_void;
    type CGDataProviderRef = *const c_void;

    unsafe extern "C" {
        fn CGImageSourceCreateWithURL(
            url: core_foundation::url::CFURLRef,
            options: core_foundation::dictionary::CFDictionaryRef,
        ) -> CGImageSourceRef;
        fn CGImageSourceCreateImageAtIndex(
            isrc: CGImageSourceRef,
            index: usize,
            options: core_foundation::dictionary::CFDictionaryRef,
        ) -> CGImageRef;
        fn CGImageGetWidth(image: CGImageRef) -> usize;
        fn CGImageGetHeight(image: CGImageRef) -> usize;
        fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
        fn CGImageGetBitsPerPixel(image: CGImageRef) -> usize;
        fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
        fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
        fn CGDataProviderCopyData(provider: CGDataProviderRef) -> core_foundation::data::CFDataRef;
        fn CFRelease(cf: *const c_void);
    }

    unsafe {
        // ファイルパスからCFURLを作成
        let path_str = path.as_ref().to_str().ok_or(AppError::PathConversion)?;
        let cf_url = core_foundation::url::CFURL::from_path(path_str, false)
            .ok_or(AppError::PathConversion)?;

        // CGImageSourceを作成
        let image_source_ref = CGImageSourceCreateWithURL(cf_url.as_concrete_TypeRef(), ptr::null());

        if image_source_ref.is_null() {
            return Err(AppError::ImageDecoding);
        }

        // 最初の画像を取得
        let cg_image_ref = CGImageSourceCreateImageAtIndex(image_source_ref, 0, ptr::null());

        if cg_image_ref.is_null() {
            CFRelease(image_source_ref);
            return Err(AppError::ImageDecoding);
        }

        // CGImageの情報を取得
        let width = CGImageGetWidth(cg_image_ref);
        let height = CGImageGetHeight(cg_image_ref);
        let bits_per_component = CGImageGetBitsPerComponent(cg_image_ref);
        let bits_per_pixel = CGImageGetBitsPerPixel(cg_image_ref);
        let bytes_per_row = CGImageGetBytesPerRow(cg_image_ref);
        let data_provider_ref = CGImageGetDataProvider(cg_image_ref);
        let cf_data_ref = CGDataProviderCopyData(data_provider_ref);

        let data = CFData::wrap_under_create_rule(cf_data_ref);
        let data_slice = data.bytes();

        // 16-bit/chの場合はRGBA16、8-bit/chの場合は8→16変換
        let is_16bit = bits_per_component == 16;
        let mut rgba16_buffer = Vec::with_capacity((width * height * 4) as usize);

        if is_16bit {
            // 16-bit/ch の場合（HDR対応）
            match bits_per_pixel {
                64 => {
                    // RGBA16 または BGRA16
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * bytes_per_row + x * 8) as usize;
                            if offset + 7 < data_slice.len() {
                                // macOSはBGRA16なのでRGBA16に変換（ビッグエンディアン）
                                let b = u16::from_be_bytes([data_slice[offset], data_slice[offset + 1]]);
                                let g = u16::from_be_bytes([data_slice[offset + 2], data_slice[offset + 3]]);
                                let r = u16::from_be_bytes([data_slice[offset + 4], data_slice[offset + 5]]);
                                let a = u16::from_be_bytes([data_slice[offset + 6], data_slice[offset + 7]]);
                                rgba16_buffer.push(r);
                                rgba16_buffer.push(g);
                                rgba16_buffer.push(b);
                                rgba16_buffer.push(a);
                            }
                        }
                    }
                }
                48 => {
                    // RGB16
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * bytes_per_row + x * 6) as usize;
                            if offset + 5 < data_slice.len() {
                                let r = u16::from_be_bytes([data_slice[offset], data_slice[offset + 1]]);
                                let g = u16::from_be_bytes([data_slice[offset + 2], data_slice[offset + 3]]);
                                let b = u16::from_be_bytes([data_slice[offset + 4], data_slice[offset + 5]]);
                                rgba16_buffer.push(r);
                                rgba16_buffer.push(g);
                                rgba16_buffer.push(b);
                                rgba16_buffer.push(0xFFFF);
                            }
                        }
                    }
                }
                _ => {
                    CFRelease(cg_image_ref);
                    CFRelease(image_source_ref);
                    return Err(AppError::UnsupportedFormat(format!(
                        "Unsupported 16-bit format: {} bits per pixel",
                        bits_per_pixel
                    )));
                }
            }
        } else {
            // 8-bit/ch の場合 → 16-bitに変換（0-255 → 0-65535）
            match bits_per_pixel {
                32 => {
                    // RGBA8 または BGRA8
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * bytes_per_row + x * 4) as usize;
                            if offset + 3 < data_slice.len() {
                                // macOSはBGRA8なのでRGBA16に変換（8bit→16bit拡張）
                                let r = (data_slice[offset + 2] as u16) * 257;
                                let g = (data_slice[offset + 1] as u16) * 257;
                                let b = (data_slice[offset] as u16) * 257;
                                let a = (data_slice[offset + 3] as u16) * 257;
                                rgba16_buffer.push(r);
                                rgba16_buffer.push(g);
                                rgba16_buffer.push(b);
                                rgba16_buffer.push(a);
                            }
                        }
                    }
                }
                24 => {
                    // RGB8
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * bytes_per_row + x * 3) as usize;
                            if offset + 2 < data_slice.len() {
                                let r = (data_slice[offset] as u16) * 257;
                                let g = (data_slice[offset + 1] as u16) * 257;
                                let b = (data_slice[offset + 2] as u16) * 257;
                                rgba16_buffer.push(r);
                                rgba16_buffer.push(g);
                                rgba16_buffer.push(b);
                                rgba16_buffer.push(0xFFFF);
                            }
                        }
                    }
                }
                _ => {
                    CFRelease(cg_image_ref);
                    CFRelease(image_source_ref);
                    return Err(AppError::UnsupportedFormat(format!(
                        "Unsupported 8-bit format: {} bits per pixel",
                        bits_per_pixel
                    )));
                }
            }
        }

        // リソース解放
        CFRelease(cg_image_ref);
        CFRelease(image_source_ref);

        // image::DynamicImageに変換（16-bit）
        let img_buffer =
            ImageBuffer::<Rgba<u16>, Vec<u16>>::from_raw(width as u32, height as u32, rgba16_buffer)
                .ok_or(AppError::ImageConversion)?;

        Ok(DynamicImage::ImageRgba16(img_buffer))
    }
}
#[cfg(target_os = "linux")]
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<DynamicImage, AppError> {
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    // heif-convertコマンドが利用可能か確認
    let heif_convert_check = Command::new("heif-convert").arg("--version").output();

    if heif_convert_check.is_err() {
        return Err(AppError::DependencyNotFound(
            "heif-convert command not found. Please install libheif-tools: sudo apt install libheif-tools".to_string()
        ));
    }

    // 一時ファイルを作成（16-bit PNG形式、HDR対応）
    let mut temp_file = NamedTempFile::new().map_err(|e| AppError::IoError(e))?;

    let temp_path = temp_file.path().with_extension("png");

    // heif-convertでPNGに変換（-dオプションで16-bitサポート試行）
    let output = Command::new("heif-convert")
        .arg("-d") // depth-preserving mode
        .arg(path.as_ref())
        .arg(&temp_path)
        .output()
        .map_err(|e| AppError::IoError(e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::CommandFailed(format!(
            "heif-convert failed: {}",
            error_msg
        )));
    }

    // 変換されたPNGを読み込む
    let img = image::open(&temp_path).map_err(|e| AppError::ImageError(e))?;

    // 一時ファイルを削除
    std::fs::remove_file(&temp_path).map_err(|e| AppError::IoError(e))?;

    Ok(img)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn test_decode_heic() {
        // テスト用のHEICファイルがあれば
        let test_file = PathBuf::from("test_data/sample.heic");
        if test_file.exists() {
            let result = decode_heic(&test_file);
            assert!(result.is_ok());
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_heif_convert_available() {
        use std::process::Command;
        let output = Command::new("which").arg("heif-convert").output();
        if let Ok(out) = output {
            println!("heif-convert status: {}", out.status.success());
        }
    }
}
