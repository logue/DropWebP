use crate::error::AppError;
use crate::options::HighBitDepthImage;
use libheif_rs::{HeifContext, ImageHandle, LibHeif};

// HDR処理用のヘルパー関数（将来的に必要になった場合のため保持）
#[allow(dead_code)]
fn apply_tone_mapping_if_needed(linear_value: f32, should_tone_map: bool) -> f32 {
    if should_tone_map {
        // Reinhard tone mapping: L_out = L_in / (1 + L_in)
        let exposure = 1.0;
        let adjusted = linear_value * exposure;
        adjusted / (1.0 + adjusted)
    } else {
        // HDR値をそのまま保持
        linear_value
    }
}

/// HEIFファイルを読み込み、HighBitDepthImageに変換する関数
/// HDR/広色域対応の改善版
pub fn decode(bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;
    let handle: ImageHandle = ctx
        .primary_image_handle()
        .map_err(|e| AppError::Decode(e.to_string()))?;

    // ICCプロファイルを取得
    let icc_profile: Option<Vec<u8>> = handle.color_profile_raw().map(|p| p.data.to_vec());

    // 画像の詳細情報を取得
    let bit_depth = handle.luma_bits_per_pixel();
    let width = handle.width();
    let height = handle.height();
    let has_alpha = handle.has_alpha_channel();

    // カラースペース情報をログ出力
    println!(
        "HEIC: 画像情報 - {}x{}, {}bit, アルファ: {}",
        width, height, bit_depth, has_alpha
    );

    if let Some(ref profile) = icc_profile {
        println!("HEIC: ICCプロファイル検出 - サイズ: {}bytes", profile.len());

        // ICCプロファイルから基本情報を抽出
        if profile.len() >= 128 {
            let profile_signature = &profile[36..40];
            let device_class = &profile[12..16];
            let color_space = &profile[16..20];

            println!("HEIC: ICCプロファイル詳細:");
            println!(
                "  - シグネチャ: {:?}",
                std::str::from_utf8(profile_signature).unwrap_or("不明")
            );
            println!(
                "  - デバイスクラス: {:?}",
                std::str::from_utf8(device_class).unwrap_or("不明")
            );
            println!(
                "  - カラースペース: {:?}",
                std::str::from_utf8(color_space).unwrap_or("不明")
            );
        }
    } else {
        println!("HEIC: ICCプロファイルなし（sRGBと仮定）");
    }

    // 適切なカラースペースでデコード
    // HDR画像の場合、線形RGBでデコードすることで色域を保持
    if bit_depth > 8 {
        println!("HEIC: HDR画像として処理（{}bit -> 線形RGB）", bit_depth);
    } else {
        println!("HEIC: SDR画像として処理（{}bit -> sRGB）", bit_depth);
    }

    // libheif-rsのデフォルトカラースペース（RGBA）でデコード
    let img = lib_heif
        .decode(
            &handle,
            libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
            None,
        )
        .map_err(|e| AppError::Decode(e.to_string()))?;

    let interleaved_plane = img
        .planes()
        .interleaved
        .ok_or_else(|| AppError::Decode("Interleaved plane not found".to_string()))?;

    // HDR対応のf32バッファ変換処理
    let high_bit_depth_image: HighBitDepthImage = if bit_depth > 8 {
        println!("HEIC: 高ビット深度処理 ({}bit)", bit_depth);

        let data_u16: &[u16] = bytemuck::cast_slice(interleaved_plane.data);
        let max_value = (1u32 << bit_depth) - 1;
        println!("HEIC: 最大値: {} ({}bit)", max_value, bit_depth);

    // デコーダーの責務：データをそのまま保持（変換はエンコーダー側で実行）
    let pixels_f32: Vec<f32> = data_u16
        .iter()
        .map(|&p| p as f32 / max_value as f32)
        .collect();        if has_alpha {
            let buffer =
                image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(width, height, pixels_f32)
                    .ok_or_else(|| {
                        AppError::Decode("Failed to create f32 ImageBuffer".to_string())
                    })?;
            HighBitDepthImage::Rgba(buffer)
        } else {
            // アルファなしの場合、RGBに変換
            let rgb_pixels: Vec<f32> = pixels_f32
                .chunks_exact(4)
                .flat_map(|rgba| &rgba[0..3])
                .cloned()
                .collect();
            let rgb_buffer =
                image::ImageBuffer::<image::Rgb<f32>, _>::from_raw(width, height, rgb_pixels)
                    .ok_or_else(|| {
                        AppError::Decode("Failed to create RGB f32 ImageBuffer".to_string())
                    })?;
            HighBitDepthImage::Rgb(rgb_buffer)
        }
    } else {
        println!("HEIC: 標準ビット深度処理 ({}bit)", bit_depth);

        // デコーダーの責務：データをそのまま保持
        // ICCプロファイルの情報はエンコーダー側で利用される
        if let Some(ref profile) = icc_profile {
            println!("HEIC: ICCプロファイル検出（{}bytes）", profile.len());
        }

        // 8bit画像も標準的な正規化のみ実行（変換処理はエンコーダー側）
        let pixels_f32: Vec<f32> = interleaved_plane
            .data
            .iter()
            .map(|&p| p as f32 / 255.0)
            .collect();

        if has_alpha {
            let buffer =
                image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(width, height, pixels_f32)
                    .ok_or_else(|| {
                        AppError::Decode("Failed to create f32 ImageBuffer".to_string())
                    })?;
            HighBitDepthImage::Rgba(buffer)
        } else {
            // アルファなしの場合、RGBに変換
            let rgb_pixels: Vec<f32> = pixels_f32
                .chunks_exact(4)
                .flat_map(|rgba| &rgba[0..3])
                .cloned()
                .collect();
            let rgb_buffer =
                image::ImageBuffer::<image::Rgb<f32>, _>::from_raw(width, height, rgb_pixels)
                    .ok_or_else(|| {
                        AppError::Decode("Failed to create RGB f32 ImageBuffer".to_string())
                    })?;
            HighBitDepthImage::Rgb(rgb_buffer)
        }
    };

    // ピクセルデータとICCプロファイルを両方返す
    Ok((high_bit_depth_image, icc_profile))
}
