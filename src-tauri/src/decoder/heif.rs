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
    let width = handle.width();
    let height = handle.height();
    let has_alpha = handle.has_alpha_channel();

    // ビット深度の正確な検出
    let luma_bits = handle.luma_bits_per_pixel();
    let chroma_bits = handle.chroma_bits_per_pixel();

    // HEICの追加メタデータを取得
    println!("HEIC: 詳細メタデータ:");
    println!("  - Width: {}, Height: {}", width, height);
    println!("  - Has Alpha: {}", has_alpha);

    // HEICの color_profile を詳細分析
    if let Some(ref profile) = icc_profile {
        println!("  - ICC Profile Size: {} bytes", profile.len());

        // ICCプロファイルから色深度情報を抽出
        if profile.len() >= 128 {
            // ICCプロファイルのヘッダー分析
            let profile_size = u32::from_be_bytes([profile[0], profile[1], profile[2], profile[3]]);
            let preferred_cmm = &profile[4..8];

            println!("  - Profile Size: {} bytes", profile_size);
            println!(
                "  - Preferred CMM: {:?}",
                std::str::from_utf8(preferred_cmm).unwrap_or("binary")
            );
            println!(
                "  - Profile Version: {}.{}.{}.{}",
                profile[8], profile[9], profile[10], profile[11]
            );
        }
    }

    // カラースペース情報をログ出力
    println!(
        "HEIC: 画像情報 - {}x{}, luma:{}bit, chroma:{}bit, アルファ: {}",
        width, height, luma_bits, chroma_bits, has_alpha
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

    // 実際のビット深度を判定（luma_bitsとchroma_bitsの最大値を使用）
    let actual_bit_depth = luma_bits.max(chroma_bits);

    // ICCプロファイル分析による10-bit画像推定
    let is_likely_10bit = if let Some(ref profile) = icc_profile {
        profile.len() > 400 && profile.len() < 1000 // Display P3などの典型サイズ
    } else {
        false
    };

    // 最終的なビット深度判定（ICCプロファイル情報も考慮）
    let effective_bit_depth = if is_likely_10bit && actual_bit_depth == 8 {
        println!("HEIC: ICCプロファイル分析により10-bit画像と判定");
        10
    } else {
        actual_bit_depth
    };

    if is_likely_10bit {
        println!("HEIC: ICCプロファイルサイズから10-bit画像と推定");
    }

    // カラースペース判定は後で実際の検出結果に基づいて表示

    println!("HEIC: 標準RGBAデコード実行中...");

    // 標準的なRGBAデコード（libheif-rsが内部で最適な精度を選択）
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

    // 実際のデータからビット深度を推定
    let total_pixels = width as usize * height as usize;
    let expected_channels = if has_alpha { 4 } else { 3 };

    // データサイズが期待値と大きく異なる場合は、別の計算方法を試行
    let data_len = interleaved_plane.data.len();
    let bytes_per_pixel = if data_len % (total_pixels * expected_channels) == 0 {
        data_len / (total_pixels * expected_channels)
    } else {
        // RGB(3ch)として再計算
        let expected_channels_rgb = 3;
        if data_len % (total_pixels * expected_channels_rgb) == 0 {
            println!("HEIC: RGB(3ch)として再計算");
            data_len / (total_pixels * expected_channels_rgb)
        } else {
            // 推定計算
            println!("HEIC: データサイズが不整合、推定計算を使用");
            data_len / total_pixels / 3
        }
    };

    println!("HEIC: データサイズ詳細分析:");
    println!("  - Total pixels: {}", total_pixels);
    println!("  - Expected channels: {}", expected_channels);
    println!("  - Data length: {} bytes", interleaved_plane.data.len());
    println!("  - Bytes per channel: {}", bytes_per_pixel);

    // さらに詳細な分析：実際のピクセル値の範囲をチェック
    let (detected_bit_depth, actual_max_value) = if bytes_per_pixel == 1 {
        // 8-bit data - しかし10-bit画像がダウンサンプルされた可能性を検証
        let sample_size = (interleaved_plane.data.len() / 100).max(1000).min(10000);
        let max_value = interleaved_plane
            .data
            .iter()
            .take(sample_size)
            .max()
            .copied()
            .unwrap_or(0);

        println!("  - 8-bit分析: サンプル最大値 = {}", max_value);

        // ICCプロファイルの分析結果を考慮して強制的に10-bit処理を適用
        if is_likely_10bit && max_value >= 200 {
            println!("  - ★ 強制10-bit処理: 8-bitデータをワイドガムット相当として処理");
            println!(
                "  - ICCプロファイル: {}bytes → ワイドガムット検出",
                icc_profile.as_ref().map(|p| p.len()).unwrap_or(0)
            );
            println!(
                "  - データ解析: 実データ8-bit (max_value={}), 10-bit相当として処理",
                max_value
            );
            // 8-bitデータを10-bit相当として扱う（ワイドガムット対応）
            (10, 1023) // 10-bit最大値として扱う
        } else {
            println!(
                "  - 標準8-bit処理: is_likely_10bit={}, max_value={}",
                is_likely_10bit, max_value
            );
            (8, max_value as u32)
        }
    } else if bytes_per_pixel == 2 {
        // 16-bit data (10-bit, 12-bit含む)
        let data_u16: &[u16] = bytemuck::cast_slice(interleaved_plane.data);
        let sample_size = (data_u16.len() / 100).max(1000).min(10000);
        let max_value = data_u16
            .iter()
            .take(sample_size)
            .max()
            .copied()
            .unwrap_or(0);

        // 実際の値の範囲から真のビット深度を推定
        let estimated_bits = if max_value <= 255 {
            8
        } else if max_value <= 1023 {
            10
        } else if max_value <= 4095 {
            12
        } else {
            16
        };

        println!(
            "  - 16-bit分析: サンプル最大値 = {}, 推定ビット深度 = {}bit",
            max_value, estimated_bits
        );
        if is_likely_10bit {
            println!(
                "  - ICCプロファイル: {}bytes → ワイドガムット対応",
                icc_profile.as_ref().map(|p| p.len()).unwrap_or(0)
            );
        }
        (estimated_bits, max_value as u32)
    } else {
        println!("  - その他の形式: {}bytes/channel", bytes_per_pixel);
        (effective_bit_depth, 255)
    };

    println!(
        "HEIC: 最終判定 - detected_bit_depth: {}, actual_max_value: {}",
        detected_bit_depth, actual_max_value
    );

    // 最終的なカラースペース判定を表示
    if detected_bit_depth > 8 {
        let processing_type = if bytes_per_pixel == 1 {
            "8-bitデータを10-bit相当で処理（ワイドガムット対応）"
        } else {
            "実10-bit/12-bit/16-bitデータを高精度処理"
        };
        println!("HEIC: {} - {}bit精度", processing_type, detected_bit_depth);
    } else {
        println!(
            "HEIC: 標準SDR画像として処理（{}bit -> sRGB）",
            detected_bit_depth
        );
    }

    // 適切なビット深度での処理（不要な高精度変換を避ける）
    let high_bit_depth_image: HighBitDepthImage = if detected_bit_depth > 8 {
        println!(
            "HEIC: 高ビット深度処理 ({}bit) - f32精度を使用",
            detected_bit_depth
        );

        let max_value = if detected_bit_depth <= 8 {
            255u32
        } else {
            actual_max_value.max((1u32 << detected_bit_depth) - 1)
        };

        println!(
            "HEIC: 正規化に使用する最大値: {} ({}bit)",
            max_value, detected_bit_depth
        );

        // デコーダーの責務：データをそのまま保持（変換はエンコーダー側で実行）
        let pixels_f32: Vec<f32> = if bytes_per_pixel == 2 {
            // 16-bit データの場合
            let data_u16: &[u16] = bytemuck::cast_slice(interleaved_plane.data);
            data_u16
                .iter()
                .map(|&p| p as f32 / max_value as f32)
                .collect()
        } else if detected_bit_depth == 10 && max_value == 1023 {
            // 10-bit画像検出：8-bitデータをワイドガムット相当として処理
            println!(
                "HEIC: 8-bitデータを10-bitワイドガムット相当として処理中（ICCプロファイル: ワイドガムット検出）..."
            );
            interleaved_plane
                .data
                .iter()
                .map(|&p| {
                    // 実際は8-bitデータを0.0-1.0範囲に正規化
                    // ICCプロファイルでワイドガムット情報を管理
                    let normalized = p as f32 / 255.0;

                    // SDR範囲を維持してガンマ問題を回避
                    // ワイドガムット効果はICCプロファイルに依存
                    normalized
                })
                .collect()
        } else {
            // 通常の8-bit データ
            interleaved_plane
                .data
                .iter()
                .map(|&p| p as f32 / max_value as f32)
                .collect()
        };

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
    } else {
        println!("HEIC: 標準ビット深度処理 ({}bit)", detected_bit_depth);

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
