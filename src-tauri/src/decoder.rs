mod heif;
mod jpeg2k;
mod jxl;

use crate::error::AppError;
use crate::options::HighBitDepthImage;
use image::{self, DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;

/// バイトデータから画像をデコードし、HighBitDepthImageとして返す
/// サポートする形式: HEIC, JPEG 2000, そして imageクレートが対応する形式
/// # 引数
/// - `image_bytes`: 画像のバイトデータ
/// # 戻り値
/// - 成功した場合は `HighBitDepthImage` を返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - EXR形式はこのバージョンではサポートされていません
/// - HEIC形式のデコードには `libheif-rs` クレートを使用しています。ビルド時に `libheif` ライブラリがシステムにインストールされている必要があります。
/// - JPEG 2000形式のデコードには `jpeg2k` クレートを使用しています。
///  ただし、このクレートはすべてのJPEG 2000ファイルに対応しているわけではないため、特定のファイルでエラーが発生する可能性があります。
pub fn decode(image_bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    // まず、バイトデータから画像形式を判別する
    let format = detect_format(image_bytes)
        .ok_or_else(|| AppError::Decode("Unsupported or unknown image format".to_string()))?;

    // 判別した形式に応じて、適切なデコーダーを呼び出す
    match format {
        DetectedFormat::Heic => {
            println!("Decoder: Using heif decoder...");
            heif::decode(image_bytes)
        }
        DetectedFormat::Exr => Err(AppError::Decode(
            "EXR format is not supported in this version".into(),
        )),
        DetectedFormat::Jpeg2000 => {
            println!("Decoder: Using Jpeg2k decoder...");
            jpeg2k::decode(image_bytes).map(|img| (img, None))
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
    Heic,
    Exr,
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
        // AVIFの判別もここに追加できる
        if ftyp == b"avif" || ftyp == b"avis" {
            // AVIFの場合はimageクレートが扱えるのでStandardに流す
            if let Ok(format) = image::guess_format(bytes) {
                return Some(DetectedFormat::Standard(format));
            }
        }
    }
    // EXRのチェック
    if bytes.starts_with(&[0x76, 0x2f, 0x31, 0x01]) {
        return Some(DetectedFormat::Exr);
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
