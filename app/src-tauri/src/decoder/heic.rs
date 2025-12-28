/// HEIC/HEIF デコーダー（OS標準API使用、HDR対応）
/// - Windows: Windows Imaging Component (WIC) - 64bppRGBA
/// - macOS: ImageIO framework - 16-bit per channel + ICC profile
/// - Linux: heif-convert コマンド - 16-bit PNG
use crate::error::AppError;
use image::DynamicImage;
use std::path::Path;

#[cfg(target_os = "macos")]
use image::{ImageBuffer, Rgba};

#[cfg(target_os = "windows")]
use image::{ImageBuffer, Rgba};

#[cfg(target_os = "windows")]
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<(DynamicImage, Option<Vec<u8>>), AppError> {
    use std::ptr;
    use windows::{
        Win32::Foundation::GENERIC_ACCESS_RIGHTS, Win32::Graphics::Imaging::*,
        Win32::System::Com::*, core::*,
    };

    unsafe {
        // COMの初期化
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|e| AppError::WindowsError(e.to_string()))?;

        // WICファクトリーの作成
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;

        // デコーダーの作成
        let path_str = path.as_ref().to_str().ok_or(AppError::PathConversion)?;
        let wide_path: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();

        let decoder = factory.CreateDecoderFromFilename(
            PCWSTR(wide_path.as_ptr()),
            None,
            GENERIC_ACCESS_RIGHTS(0x80000000), // GENERIC_READ
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

        // TODO: Windows版もWICからICCプロファイルを抽出する必要がある
        Ok((DynamicImage::ImageRgba16(img_buffer), None))
    }
}

#[cfg(target_os = "macos")]
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<(DynamicImage, Option<Vec<u8>>), AppError> {
    use core_foundation::base::TCFType;
    use core_foundation::data::CFData;
    use std::os::raw::c_void;
    use std::ptr;

    // C型定義
    type CGImageSourceRef = *const c_void;
    type CGImageRef = *const c_void;
    type CGDataProviderRef = *const c_void;
    type CGColorSpaceRef = *const c_void;
    type CFDictionaryRef = *const c_void;

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
        fn CGImageSourceCopyPropertiesAtIndex(
            isrc: CGImageSourceRef,
            index: usize,
            options: core_foundation::dictionary::CFDictionaryRef,
        ) -> CFDictionaryRef;
        fn CGImageGetWidth(image: CGImageRef) -> usize;
        fn CGImageGetHeight(image: CGImageRef) -> usize;
        fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
        fn CGImageGetBitsPerPixel(image: CGImageRef) -> usize;
        fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
        fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
        fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
        fn CGColorSpaceCopyICCData(space: CGColorSpaceRef) -> core_foundation::data::CFDataRef;
        fn CGColorSpaceCopyName(space: CGColorSpaceRef) -> core_foundation::string::CFStringRef;
        fn CGDataProviderCopyData(provider: CGDataProviderRef) -> core_foundation::data::CFDataRef;
        fn CFRelease(cf: *const c_void);
    }

    unsafe {
        // ファイルパスからCFURLを作成
        let path_str = path.as_ref().to_str().ok_or(AppError::PathConversion)?;
        let cf_url = core_foundation::url::CFURL::from_path(path_str, false)
            .ok_or(AppError::PathConversion)?;

        // CGImageSourceを作成
        let image_source_ref =
            CGImageSourceCreateWithURL(cf_url.as_concrete_TypeRef(), ptr::null());

        if image_source_ref.is_null() {
            return Err(AppError::ImageDecoding);
        }

        // 画像メタデータを取得してHDR Gain Mapをチェック
        // ファイルの生データから直接HDR Gain Map関連の文字列を検索
        let mut has_gain_map = false;

        if let Ok(file_data) = std::fs::read(path.as_ref()) {
            // Convert to string for searching (lossy is OK for metadata detection)
            let file_str = String::from_utf8_lossy(&file_data);

            // Check for Apple HDR Gain Map indicators
            // These strings appear in HEIC files with Gain Map HDR
            if file_str.contains("HDRGainMap")
                || file_str.contains("hdrgainmap")
                || file_str.contains("urn:com:apple:photo:2020:aux:hdrgainmap")
                || file_str.contains("Apple_Gain_Map")
                || file_str.contains("GainMapHeadroom")
                || file_str.contains("GainMapVersion")
            {
                has_gain_map = true;
                println!("HEIC: Apple HDR Gain Map detected in file metadata");

                // Check if libheif-convert is available (optional external tool)
                // Cross-platform: Use .exe extension on Windows
                #[cfg(target_os = "windows")]
                let heif_convert_cmd = "heif-convert.exe";
                #[cfg(not(target_os = "windows"))]
                let heif_convert_cmd = "heif-convert";

                let heif_available = std::process::Command::new(heif_convert_cmd)
                    .arg("--version")
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false);

                if heif_available {
                    println!("HEIC: libheif-convert found - Gain Map HDR can be processed");
                    println!(
                        "HEIC: NOTE: For best results, consider using libheif-convert directly"
                    );
                    println!("HEIC: Example: {} input.heic output.png", heif_convert_cmd);
                } else {
                    println!("HEIC: WARNING - Gain Map HDR detected but not fully supported");
                    println!("HEIC: The base SDR image (8-bit Display P3) will be decoded");
                    println!("HEIC: HDR tone mapping from Gain Map is NOT applied");
                    println!("HEIC: ");
                    println!("HEIC: To enable HDR support, install libheif (optional):");
                    #[cfg(target_os = "macos")]
                    println!("HEIC:   macOS: brew install libheif");
                    #[cfg(target_os = "linux")]
                    println!("HEIC:   Linux: sudo apt install libheif-examples");
                    #[cfg(target_os = "windows")]
                    {
                        println!(
                            "HEIC:   Windows: Download from https://github.com/strukturag/libheif/releases"
                        );
                        println!("HEIC:   Or use Chocolatey: choco install libheif");
                    }
                    println!("HEIC: ");
                    println!("HEIC: Alternative: Convert with ffmpeg or macOS Preview first");
                    println!("HEIC:   ffmpeg -i input.heic -pix_fmt rgb48le output.png");
                }
            }
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

        // カラースペース情報とICCプロファイルを取得
        let color_space_ref = CGImageGetColorSpace(cg_image_ref);
        let mut icc_profile: Option<Vec<u8>> = None;
        let mut color_space_name = String::new();

        if !color_space_ref.is_null() {
            // カラースペース名を取得
            let cs_name_ref = CGColorSpaceCopyName(color_space_ref);
            if !cs_name_ref.is_null() {
                let cs_name =
                    core_foundation::string::CFString::wrap_under_create_rule(cs_name_ref);
                color_space_name = cs_name.to_string();
                println!("HEIC: Color space: {}", color_space_name);
            }

            // ICCプロファイルを取得
            let icc_data_ref = CGColorSpaceCopyICCData(color_space_ref);
            if !icc_data_ref.is_null() {
                let icc_data = CFData::wrap_under_create_rule(icc_data_ref);
                let icc_bytes = icc_data.bytes();
                if !icc_bytes.is_empty() {
                    let mut profile = icc_bytes.to_vec();

                    // カラースペース名にPQ/HLG/HDR情報が含まれている場合、
                    // ICCプロファイルの末尾にメタデータとして追加
                    // これにより、encoder側でHDR検出が可能になる
                    if color_space_name.contains("PQ")
                        || color_space_name.contains("HLG")
                        || color_space_name.contains("HDR")
                        || color_space_name.contains("BT2020")
                        || color_space_name.contains("Rec2020")
                        || has_gain_map
                    {
                        // カラースペース名をUTF-8バイト列として追加
                        profile.extend_from_slice(b"\n[ColorSpace]");
                        if has_gain_map && !color_space_name.contains("HDR") {
                            // Gain Map HDRの場合は明示的にマーク
                            profile.extend_from_slice(
                                format!("{} (Apple Gain Map HDR)", color_space_name).as_bytes(),
                            );
                        } else {
                            profile.extend_from_slice(color_space_name.as_bytes());
                        }
                        println!("HEIC: Added color space metadata to ICC profile");
                    }

                    icc_profile = Some(profile);
                    println!(
                        "HEIC: Extracted ICC profile ({} bytes)",
                        icc_profile.as_ref().unwrap().len()
                    );
                } else {
                    println!("HEIC: ICC profile is empty");
                }
            } else {
                println!("HEIC: No ICC profile available in color space");

                // ICCプロファイルがない場合でも、カラースペース名からHDR情報を生成
                if color_space_name.contains("PQ")
                    || color_space_name.contains("HLG")
                    || color_space_name.contains("HDR")
                    || has_gain_map
                {
                    let mut synthetic_profile = Vec::new();
                    synthetic_profile.extend_from_slice(b"[ColorSpace]");
                    if has_gain_map {
                        synthetic_profile.extend_from_slice(
                            format!("{} (Apple Gain Map HDR)", color_space_name).as_bytes(),
                        );
                    } else {
                        synthetic_profile.extend_from_slice(color_space_name.as_bytes());
                    }
                    icc_profile = Some(synthetic_profile);
                    println!("HEIC: Created synthetic HDR profile from color space name");
                }
            }
        }

        // 16-bit/chの場合はRGBA16、8-bit/chの場合は8→16変換
        let is_16bit = bits_per_component == 16;
        println!("HEIC: Bit depth: {}-bit per component", bits_per_component);
        let mut rgba16_buffer = Vec::with_capacity((width * height * 4) as usize);

        if is_16bit {
            // 16-bit/ch の場合（HDR対応）
            match bits_per_pixel {
                64 => {
                    // RGBA16（ビッグエンディアン）
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * bytes_per_row + x * 8) as usize;
                            if offset + 7 < data_slice.len() {
                                // macOS CGImageはRGBA16順序（ビッグエンディアン）
                                let r = u16::from_be_bytes([
                                    data_slice[offset],
                                    data_slice[offset + 1],
                                ]);
                                let g = u16::from_be_bytes([
                                    data_slice[offset + 2],
                                    data_slice[offset + 3],
                                ]);
                                let b = u16::from_be_bytes([
                                    data_slice[offset + 4],
                                    data_slice[offset + 5],
                                ]);
                                let a = u16::from_be_bytes([
                                    data_slice[offset + 6],
                                    data_slice[offset + 7],
                                ]);
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
                                let r = u16::from_be_bytes([
                                    data_slice[offset],
                                    data_slice[offset + 1],
                                ]);
                                let g = u16::from_be_bytes([
                                    data_slice[offset + 2],
                                    data_slice[offset + 3],
                                ]);
                                let b = u16::from_be_bytes([
                                    data_slice[offset + 4],
                                    data_slice[offset + 5],
                                ]);
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
                    // RGBA8（macOS CGImageは通常RGBA順序）
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * bytes_per_row + x * 4) as usize;
                            if offset + 3 < data_slice.len() {
                                // RGBA8からRGBA16に変換（8bit→16bit拡張）
                                let r = (data_slice[offset] as u16) * 257;
                                let g = (data_slice[offset + 1] as u16) * 257;
                                let b = (data_slice[offset + 2] as u16) * 257;
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
        let img_buffer = ImageBuffer::<Rgba<u16>, Vec<u16>>::from_raw(
            width as u32,
            height as u32,
            rgba16_buffer,
        )
        .ok_or(AppError::ImageConversion)?;

        Ok((DynamicImage::ImageRgba16(img_buffer), icc_profile))
    }
}

#[cfg(target_os = "linux")]
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<(DynamicImage, Option<Vec<u8>>), AppError> {
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

    // TODO: Linux版もheif-convertからICCプロファイルを抽出する必要がある
    Ok((img, None))
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
