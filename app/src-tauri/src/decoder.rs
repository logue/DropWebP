mod avif;
mod common;
mod heic;
mod jpeg2k;
mod jxl;

// Re-export IccProfileInfo for use in other modules
pub use common::IccProfileInfo;

use crate::error::AppError;
use crate::options::HighBitDepthImage;
use exif;
use image::{self, DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;
use std::path::Path;

/// ファイルパスから画像をデコードし、HighBitDepthImageとして返す
/// HEIC形式の場合はOS標準APIを使用
pub fn decode_from_path<P: AsRef<Path>>(
    path: P,
) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    let decode_start = std::time::Instant::now();

    // ファイルを読み込む
    let data = std::fs::read(path.as_ref()).map_err(|e| AppError::IoError(e))?;

    // フォーマットを検出
    let format = detect_format(&data)
        .ok_or_else(|| AppError::Decode("Unsupported or unknown image format".to_string()))?;

    // HEICの場合はOS標準APIを使用（HDR対応16-bit）
    if matches!(format, DetectedFormat::Heic) {
        println!("Decoder: Using OS-native HEIC decoder (HDR-capable)...");
        let path_ref = path.as_ref();
        let mut img = heic::decode_heic(path_ref)?;

        // EXIF Orientation を処理して画像を回転
        if let Ok(exif_data) = std::fs::read(path_ref) {
            if let Ok(exif_reader) =
                exif::Reader::new().read_from_container(&mut std::io::Cursor::new(&exif_data))
            {
                if let Some(orientation_field) =
                    exif_reader.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                {
                    if let Some(orientation_value) = orientation_field.value.get_uint(0) {
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
                }
            }
        }

        // HEICは16-bit RGBA (Rgba16)でデコードされる
        // iPhone HEIC は Display P3 の SDR (0-1 範囲) を 16-bit で表現
        let high_bit_img = match img {
            DynamicImage::ImageRgba16(rgba16) => {
                println!("HEIC: 16-bit image detected, converting Display P3 to sRGB");
                let (width, height) = rgba16.dimensions();

                // macOS ImageIO は BGR 順序で出力する
                // Display P3 → sRGB/BT.709 色域変換を適用
                let pixels_f32: Vec<f32> = rgba16
                    .pixels()
                    .flat_map(|p| {
                        // BGR として読み取る（macOS ImageIO の出力形式）
                        let b = p.0[0] as f32 / 65535.0;
                        let g = p.0[1] as f32 / 65535.0;
                        let r = p.0[2] as f32 / 65535.0;
                        let a = p.0[3] as f32 / 65535.0;

                        // sRGB ガンマを解除（線形化）
                        let r_lin = if r <= 0.04045 {
                            r / 12.92
                        } else {
                            ((r + 0.055) / 1.055).powf(2.4)
                        };
                        let g_lin = if g <= 0.04045 {
                            g / 12.92
                        } else {
                            ((g + 0.055) / 1.055).powf(2.4)
                        };
                        let b_lin = if b <= 0.04045 {
                            b / 12.92
                        } else {
                            ((b + 0.055) / 1.055).powf(2.4)
                        };

                        // Display P3 → sRGB/BT.709 変換行列（線形空間）
                        let r_out = r_lin * 1.2249 + g_lin * -0.2247 + b_lin * -0.0002;
                        let g_out = r_lin * -0.0420 + g_lin * 1.0419 + b_lin * 0.0001;
                        let b_out = r_lin * -0.0197 + g_lin * -0.0786 + b_lin * 1.0983;

                        // クリッピング（色域外の色を範囲内に）
                        let r_clip = r_out.max(0.0).min(1.0);
                        let g_clip = g_out.max(0.0).min(1.0);
                        let b_clip = b_out.max(0.0).min(1.0);

                        // sRGB ガンマを適用
                        let r_srgb = if r_clip <= 0.0031308 {
                            r_clip * 12.92
                        } else {
                            1.055 * r_clip.powf(1.0 / 2.4) - 0.055
                        };
                        let g_srgb = if g_clip <= 0.0031308 {
                            g_clip * 12.92
                        } else {
                            1.055 * g_clip.powf(1.0 / 2.4) - 0.055
                        };
                        let b_srgb = if b_clip <= 0.0031308 {
                            b_clip * 12.92
                        } else {
                            1.055 * b_clip.powf(1.0 / 2.4) - 0.055
                        };

                        [r_srgb, g_srgb, b_srgb, a]
                    })
                    .collect();

                let buffer =
                    image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(width, height, pixels_f32)
                        .ok_or_else(|| {
                            AppError::Decode("Failed to create RGBA f32 HDR buffer".to_string())
                        })?;

                HighBitDepthImage::Rgba(buffer)
            }
            DynamicImage::ImageRgba8(rgba) => {
                // フォールバック: 8-bit RGBA (SDR)
                println!("HEIC: 8-bit image, converting to f32");
                HighBitDepthImage::Rgba(image::DynamicImage::ImageRgba8(rgba).to_rgba32f())
            }
            _ => {
                println!("HEIC: Converting other format to f32");
                HighBitDepthImage::Rgba(img.to_rgba32f())
            }
        };

        // HEIC は通常 Display P3 色域を使用
        // 合成 ICC プロファイルマーカーを返して広色域として認識させる
        let synthetic_icc = b"Display P3".to_vec();
        println!(
            "Decoder: HEIC decoded in {:.2}s",
            decode_start.elapsed().as_secs_f64()
        );
        return Ok((high_bit_img, Some(synthetic_icc)));
    }

    // その他の形式は従来のdecode関数を使用
    let result = decode(&data);
    println!(
        "Decoder: Image decoded in {:.2}s",
        decode_start.elapsed().as_secs_f64()
    );
    result
}

/// バイトデータから画像をデコードし、HighBitDepthImageとして返す
/// サポートする形式: JPEG 2000, JPEG XL, そして imageクレートが対応する形式
/// # 引数
/// - `image_bytes`: 画像のバイトデータ
/// # 戻り値
/// - 成功した場合は `HighBitDepthImage` を返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - JPEG 2000形式のデコードには `jpeg2k` クレートを使用しています。
///   ただし、このクレートはすべてのJPEG 2000ファイルに対応しているわけではないため、特定のファイルでエラーが発生する可能性があります。
/// - HEIC/HEIF形式はOS標準APIを使用してデコードします：
///   - Windows: Windows Imaging Component (WIC)
///   - macOS: ImageIO framework
///   - Linux: heif-convert コマンド (要 libheif-tools パッケージ)
pub fn decode(image_bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    // まず、バイトデータから画像形式を判別する
    let format = detect_format(image_bytes)
        .ok_or_else(|| AppError::Decode("Unsupported or unknown image format".to_string()))?;

    // 判別した形式に応じて、適切なデコーダーを呼び出す
    match format {
        DetectedFormat::Avif => {
            println!("Decoder: Using AVIF decoder...");
            avif::decode(image_bytes)
        }
        DetectedFormat::Heic => {
            // HEICはファイルパスが必要なため、decode_from_pathを使用する必要がある
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

            // 1. まずはDynamicImageとしてメモリから読み込む
            let img: DynamicImage = image::load_from_memory(image_bytes)
                .map_err(|e| AppError::Decode(e.to_string()))?;

            // 2. カラータイプとビット深度を分析
            let color_type = img.color();
            let (width, height) = img.dimensions();

            println!("PNG: {}x{} - {:?}", width, height, color_type);

            // ICCプロファイル分析によるワイドガムット検出
            let has_wide_gamut_profile = if let Some(ref profile) = icc_profile {
                println!("PNG: ICCプロファイル検出 - サイズ: {}bytes", profile.len());
                // 大きなICCプロファイル（Display P3, Rec2020など）はワイドガムットの可能性
                profile.len() > 400 && profile.len() < 1000
            } else {
                false
            };

            // ビット深度判定：16-bit形式 または ワイドガムットICCプロファイル付き8-bit
            let requires_high_precision = match color_type {
                image::ColorType::L16 | image::ColorType::Rgb16 | image::ColorType::Rgba16 => {
                    println!("PNG: 16-bit画像として高精度処理");
                    true
                }
                _ if has_wide_gamut_profile => {
                    println!("PNG: 8-bit画像だがワイドガムットICCプロファイル検出 - 高精度処理");
                    true
                }
                _ => {
                    println!("PNG: 標準8-bit画像として処理");
                    false
                }
            };

            // 3. ビット深度に応じた適切な変換
            if requires_high_precision {
                println!("PNG: 高精度f32変換を実行");
                return match color_type {
                    // アルファチャンネルを持たない形式の場合
                    image::ColorType::L8
                    | image::ColorType::L16
                    | image::ColorType::Rgb8
                    | image::ColorType::Rgb16 => {
                        Ok((HighBitDepthImage::Rgb(img.to_rgb32f()), icc_profile))
                    }
                    // アルファチャンネルを持つ形式の場合
                    _ => Ok((HighBitDepthImage::Rgba(img.to_rgba32f()), icc_profile)),
                };
            } else {
                println!("PNG: 標準精度処理（8-bit効率化）");
                // 8-bit標準画像：効率的な変換（不要な高精度変換を避ける）
                return match color_type {
                    // アルファチャンネルを持たない形式の場合
                    image::ColorType::L8 | image::ColorType::Rgb8 => {
                        // 8-bitデータを効率的にf32に変換（0-1範囲）
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
                    // アルファチャンネルを持つ8-bit形式の場合
                    image::ColorType::La8 | image::ColorType::Rgba8 => {
                        println!("PNG: 8-bit RGBA画像を効率的に変換");
                        // 8-bitデータを効率的にf32に変換（0-1範囲）
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
                    // その他（16-bit等）はフォールバック
                    _ => {
                        println!("PNG: フォールバック - 高精度変換");
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
                };
            }
        }
    }
}

// 独自の形式を定義するためのenum
enum DetectedFormat {
    Avif,
    Heic,
    Jpeg2000,
    Jxl,
    // imageクレートがサポートするその他の形式
    Standard(ImageFormat),
}

/// バイトデータのマジックナンバーから画像形式を判別する
fn detect_format(bytes: &[u8]) -> Option<DetectedFormat> {
    // HEIC/AVIF (ISOBMFFコンテナ) のチェック
    // ftyp ボックスが "heic", "heix", "avif" などを含むか
    if bytes.len() > 12 && &bytes[4..8] == b"ftyp" {
        let ftyp = &bytes[8..12];
        if ftyp == b"heic" || ftyp == b"heix" || ftyp == b"hevc" || ftyp == b"heim" {
            return Some(DetectedFormat::Heic);
        }
        // AVIFの判別
        if ftyp == b"avif" || ftyp == b"avis" {
            return Some(DetectedFormat::Avif);
        }
    }

    // JPEG 2000のチェック
    if bytes.starts_with(b"\x00\x00\x00\x0CjP  \r\n\x87\n") {
        return Some(DetectedFormat::Jpeg2000);
    }

    // JPEG XLのチェック
    if bytes.starts_with(b"\xFF\x0A") || bytes.starts_with(b"\x00\x00\x00\x0CJXL ") {
        return Some(DetectedFormat::Jxl);
    }

    // 上記のいずれでもない場合、imageクレートの形式推測に任せる
    if let Ok(format) = image::guess_format(bytes) {
        return Some(DetectedFormat::Standard(format));
    }

    None
}

pub fn extract_icc_profile(bytes: &[u8]) -> Option<Vec<u8>> {
    // --- PNGの場合 ---
    // PNGのマジックナンバー (89 50 4E 47 0D 0A 1A 0A) を確認
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        let decoder = png::Decoder::new(Cursor::new(bytes));
        if let Ok(reader) = decoder.read_info() {
            if let Some(profile) = &reader.info().icc_profile {
                // iCCPチャンクからプロファイルデータを取得
                return Some(profile.to_vec());
            }
        }
        return None;
    }

    // --- JPEGの場合 ---
    // JPEGのマジックナンバー (FF D8) を確認
    if bytes.starts_with(&[0xFF, 0xD8]) {
        let mut icc_chunks = std::collections::BTreeMap::new();
        let mut pos = 2; // SOIマーカーの後からスキャン開始

        while pos < bytes.len() - 4 {
            // マーカー (FFで始まる) を探す
            if bytes[pos] != 0xFF {
                pos += 1;
                continue;
            }

            let marker = bytes[pos + 1];

            // APP2マーカー (FF E2) かどうかを確認
            if marker == 0xE2 {
                let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
                let segment_data = &bytes[pos + 4..pos + 2 + length];

                // "ICC_PROFILE" という識別子があるか確認
                if segment_data.starts_with(b"ICC_PROFILE\0") {
                    // チャンク情報を取得
                    let chunk_index = segment_data[12];
                    let total_chunks = segment_data[13];
                    let profile_part = &segment_data[14..];

                    icc_chunks.insert(chunk_index, profile_part);

                    // 全てのチャンクが集まったか確認
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
            // 次のマーカーへ移動
            let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
            pos += 2 + length;
        }

        return None;
    }

    // 未対応のフォーマット
    None
}
