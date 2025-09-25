use crate::{
    encoder::{HighBitDepthImage, extract_pixel_data},
    error::AppError,
};
use jpegxl_rs::encode::{EncoderFrame, EncoderResult, EncoderSpeed::*, encoder_builder};
use serde::{Deserialize, Serialize};

/// 元画像のビット深度を推定する関数
/// f32データから逆算して元のビット深度を推定
fn estimate_original_bit_depth(pixels_f32: &[f32], icc_profile: &Option<Vec<u8>>) -> u8 {
    // ICCプロファイルサイズによる判定
    let profile_suggests_high_bit = icc_profile.as_ref().map_or(false, |p| p.len() > 400);

    // ピクセル値の精度分析
    let sample_size = (pixels_f32.len() / 100).max(1000).min(10000);
    let mut unique_values = std::collections::HashSet::new();

    for &pixel in pixels_f32.iter().take(sample_size) {
        if pixel >= 0.0 && pixel <= 1.0 {
            // f32値を8-bitスケールで逆変換
            let scaled_8bit = (pixel * 255.0).round() as u8;
            let rescaled = scaled_8bit as f32 / 255.0;

            // 元の値との差が小さければ8-bit由来と判定
            if (pixel - rescaled).abs() < 0.002 {
                unique_values.insert(scaled_8bit);
            }
        }
    }

    // 分析結果
    let appears_8bit_quantized =
        unique_values.len() <= 256 && pixels_f32.iter().take(sample_size).all(|&p| p <= 1.0);

    // 判定ロジック
    if appears_8bit_quantized && !profile_suggests_high_bit {
        println!("JXL: 8-bit量子化パターンを検出 - 標準8-bit画像と判定");
        8
    } else if profile_suggests_high_bit {
        println!("JXL: ICCプロファイル分析により10-bit相当と判定");
        10
    } else {
        println!("JXL: 高ビット深度パターンを検出");
        16
    }
}

/// JPEG XL形式のオプション
///
/// 注意: jpegxl-rs v0.11.2にはロスレスエンコードに関して既知の不具合があります。
/// 特にRGBA画像でロスレスを指定するとApiUsageエラーが発生します。
/// この実装では自動的に高品質モードにフォールバックします。
///
/// * `lossless` - ロスレス圧縮するか（RGBA画像では自動フォールバック）
/// * `speed` - エンコード速度（0~10）値が低いほど早いが品質が劣る
/// * `quality` - 品質（0.1〜15.0）値が高いほど高品質。デフォルトは1。推奨値0.5〜3.0。（ロスレス時は無視されます）
/// * `use_container` - JPEG XLコンテナ形式を使用するようにエンコーダを構成する
/// * `uses_original_profile` - エンコーダを元のカラープロファイルを使用するように設定する。（ロスレス時は常に有効）
/// * `decoding_speed` - デコード速度を設定（0~4）。値が低いほど高品質。デフォルトは0
/// * `init_buffer_size` - 出力バッファの初期サイズ（UI側はキロバイト単位、内部でバイト単位に変換）最小32KB
/// * `color_encoding` - カラーエンコード方法を設定する。デフォルトはsRGB
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

/// エンコード速度の列挙型
/// - Lightning: 最速、品質は最低
/// - Thunder: 非常に速い、品質は低い
/// - Falcon: 速い、品質はやや低い
/// - Cheetah: バランスの取れた速度と品質
/// - Hare: やや遅い、品質は良い
/// - Wombat: 遅い、品質は非常に良い
/// - Squirrel: 非常に遅い、品質は最高
/// - Kitten: 最高品質、非常に遅い
/// - Tortoise: 最高品質、非常に遅い
/// - Glacier: 最高品質、非常に遅い、アーカイブ向け
/// # 注意
/// - 速度が遅いほど品質が高くなりますが、エンコード時間も長くなります。
/// - 速度設定は0〜10の範囲で行い、0が最速、10が最高品質です。
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
    pub fn to_jxl(self) -> jpegxl_rs::encode::EncoderSpeed {
        match self {
            EncoderSpeed::Lightning => Lightning,
            EncoderSpeed::Thunder => Thunder,
            EncoderSpeed::Falcon => Falcon,
            EncoderSpeed::Cheetah => Cheetah,
            EncoderSpeed::Hare => Hare,
            EncoderSpeed::Wombat => Wombat,
            EncoderSpeed::Squirrel => Squirrel,
            EncoderSpeed::Kitten => Kitten,
            EncoderSpeed::Tortoise => Tortoise,
            EncoderSpeed::Glacier => Glacier,
        }
    }
}

/// カラーエンコード方法の列挙型
/// - Srgb: 標準的なsRGBカラースペース
/// - LinearSrgb: 線形sRGBカラースペース
/// - SrgbLuma: sRGBカラースペースで輝度情報を使用
/// - LinearSrgbLuma: 線形sRGBカラースペースで輝度情報を使用
/// # 注意
/// - 適切なカラーエンコードを選択することで、画像の品質を最適化できます。  
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorEncoding {
    Srgb,
    LinearSrgb,
    SrgbLuma,
    LinearSrgbLuma,
}

impl ColorEncoding {
    pub fn to_jxl(self) -> jpegxl_rs::encode::ColorEncoding {
        match self {
            ColorEncoding::Srgb => jpegxl_rs::encode::ColorEncoding::Srgb,
            ColorEncoding::LinearSrgb => jpegxl_rs::encode::ColorEncoding::LinearSrgb,
            ColorEncoding::SrgbLuma => jpegxl_rs::encode::ColorEncoding::SrgbLuma,
            ColorEncoding::LinearSrgbLuma => jpegxl_rs::encode::ColorEncoding::LinearSrgbLuma,
        }
    }
}

/// HighBitDepthImage を JPEG XL 形式のバイトデータに変換する
///
/// # 引数
/// * `pixel_data` - 変換元のHighBitDepthImage
/// * `icc_profile` - ICCプロファイル（Someの場合は埋め込み処理を行う）
/// * `options` - JXLエンコードオプション (JxlOptions)
/// # 戻り値
/// - 成功した場合は JPEG XL のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// * ICCプロファイルが提供された場合、カスタムメタデータボックスとして埋め込まれます
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &JxlOptions,
) -> Result<Vec<u8>, AppError> {
    // HighBitDepthImage から画像サイズを取得
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };

    // HighBitDepthImage から f32 のピクセルデータとアルファチャンネルの有無を取得
    let (pixels_f32, is_rgba) = extract_pixel_data(pixel_data);

    // エンコーダー設定（安全なデフォルト値を使用）
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(options.speed.to_jxl())
        .use_container(options.use_container);

    // decoding_speedの値を検証（範囲外の値はApiUsageエラーの原因になる）
    let safe_decoding_speed = options.decoding_speed.clamp(0, 4);
    if safe_decoding_speed != options.decoding_speed {
        eprintln!(
            "JXL Warning: decoding_speed値を調整しました {} -> {}",
            options.decoding_speed, safe_decoding_speed
        );
    }
    builder = builder.decoding_speed(safe_decoding_speed);

    // init_buffer_sizeの値を検証（UI側はキロバイト指定前提）
    // UI側の値をキロバイト単位からバイト単位に変換
    let buffer_size_kb = options.init_buffer_size; // UI側からの値（KB単位）
    let buffer_size_bytes = buffer_size_kb * 1024; // KB → bytes 変換

    // jpegxl-rsの最小要件: 32KB = 32768 bytes
    let safe_buffer_size = if buffer_size_bytes < 32768 {
        32768 // 32KB minimum (32768 bytes)
    } else {
        buffer_size_bytes
    };

    if safe_buffer_size != buffer_size_bytes {
        println!(
            "JXL Warning: init_buffer_size値を最小要件に調整しました {}KB -> 32KB (32768bytes)",
            buffer_size_kb
        );
    } else {
        // キロバイト単位で指定された場合の確認メッセージ
        println!(
            "JXL: バッファサイズを設定しました {}KB ({}bytes)",
            buffer_size_kb, buffer_size_bytes
        );
    }

    builder = builder.init_buffer_size(safe_buffer_size);

    // HDR画像の検出
    let max_pixel_value = pixels_f32
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap_or(1.0);

    // 元のビット深度推定（デコーダーからの情報を復元）
    let estimated_original_bit_depth = estimate_original_bit_depth(&pixels_f32, &icc_profile);

    println!(
        "JXL: 元画像ビット深度推定: {}bit",
        estimated_original_bit_depth
    );

    // ICCプロファイル情報も考慮したHDR/ワイドガムット検出
    let has_wide_gamut_profile = icc_profile.as_ref().map_or(false, |p| p.len() > 300);
    let is_hdr = max_pixel_value > 1.0;
    // 8-bit画像の不適切な高精度処理を回避
    let is_likely_8bit_source = estimated_original_bit_depth <= 8 && max_pixel_value <= 1.0;
    let is_wide_gamut_sdr = has_wide_gamut_profile && !is_hdr && !is_likely_8bit_source;

    if is_hdr {
        println!(
            "JXL: HDR画像を検出（最大輝度: {:.3}） - HDR用設定を適用",
            max_pixel_value
        );
        // HDR画像の場合、線形カラーエンコーディングを使用
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::LinearSrgb);
    } else if is_wide_gamut_sdr {
        println!(
            "JXL: ワイドガムットSDR画像を検出（ICCプロファイル: {}bytes） - 高品質設定を適用",
            icc_profile.as_ref().unwrap().len()
        );
        // ワイドガムットSDR画像の場合、sRGBを使用してICCプロファイルで色域を管理
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
        println!("JXL: ワイドガムット用高品質設定を適用");
    } else if is_likely_8bit_source {
        println!(
            "JXL: 標準8-bit画像を検出（最大値: {:.3}） - 効率的な8-bit設定を適用",
            max_pixel_value
        );
        // 8-bit画像の場合、標準sRGB設定で効率的に処理
        builder = builder.color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb);
    } else {
        // その他の高ビット深度画像
        builder = builder.color_encoding(options.color_encoding.to_jxl());
    }

    if options.uses_original_profile {
        builder = builder.uses_original_profile(true);
    }

    // jpegxl-rs v0.11.2のロスレス不具合の包括的対策
    // 既知の問題:
    // 1. RGBA画像 + ロスレス = ApiUsageエラー
    // 2. 特定の設定組み合わせで不安定
    // 3. ピクセル値の範囲チェックが厳しい

    let (use_lossless, fallback_reason) = if options.lossless {
        if is_rgba {
            (false, Some("RGBA画像でのロスレス不具合のため"))
        } else if width * height > 4096 * 4096 {
            // 大きな画像でのロスレス也不安定
            (false, Some("大サイズ画像でのロスレス不安定のため"))
        } else {
            (true, None)
        }
    } else {
        (false, None)
    };

    if let Some(reason) = fallback_reason {
        println!("JXL: {}、高品質モードにフォールバックします", reason);
    }

    if use_lossless {
        builder = builder.lossless(true);
    } else {
        // 品質値を厳密に検証
        // RGBA画像の場合は高品質設定を使用
        let target_quality = if is_rgba && options.lossless {
            0.5 // ロスレスからのフォールバック時は高品質を使用
        } else {
            options.quality
        };

        let safe_quality = target_quality.clamp(0.1, 15.0);
        if safe_quality != options.quality {
            if is_rgba && options.lossless {
                println!(
                    "JXL: RGBA画像のロスレスフォールバックのため、品質を{:.3}に設定しました",
                    safe_quality
                );
            } else {
                eprintln!(
                    "JXL Warning: quality値を調整しました {:.3} -> {:.3}",
                    options.quality, safe_quality
                );
            }
        }
        builder = builder.quality(safe_quality);
    }

    // エンコーダーを構築
    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL encoder build failed: {}", e)))?;

    // ICCプロファイルが提供された場合、カスタムメタデータとして追加
    if let Some(profile_data) = &icc_profile {
        println!(
            "JXL: ICCプロファイルを埋め込み中... (サイズ: {} bytes)",
            profile_data.len()
        );

        // ICCプロファイルをカスタムメタデータボックスとして追加
        // 'icc ' (ICCプロファイル用の標準的な4文字コード)
        let icc_type = *b"icc ";
        let metadata = jpegxl_rs::encode::Metadata::Custom(icc_type, profile_data);

        if let Err(e) = encoder.add_metadata(&metadata, false) {
            eprintln!("JXL: ICCプロファイルの埋め込みに失敗しました: {:?}", e);
            eprintln!("JXL: ICCプロファイルなしで処理を続行します");
        } else {
            println!("JXL: ICCプロファイルの埋め込みが完了しました");
        }
    }

    // エンコード情報
    let bit_depth_info = if is_likely_8bit_source {
        " [8-bit効率化]"
    } else if estimated_original_bit_depth > 8 {
        &format!(" [{}bit高精度]", estimated_original_bit_depth)
    } else {
        ""
    };

    println!(
        "JXL: {}x{} {}画像を変換中{}{}{}",
        width,
        height,
        if is_rgba { "RGBA" } else { "RGB" },
        bit_depth_info,
        if icc_profile.is_some() {
            " (ICCプロファイル付き)"
        } else {
            ""
        },
        if is_rgba && options.lossless && !use_lossless {
            " [ロスレスフォールバック]"
        } else if use_lossless {
            " [ロスレス]"
        } else {
            ""
        }
    );

    // ピクセル値の範囲をチェック（HDR対応版）
    if let Some(min_val) = pixels_f32.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        if let Some(max_val) = pixels_f32.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            println!("JXL: ピクセル値範囲 [{:.3}, {:.3}]", min_val, max_val);

            if *max_val > 1.0 {
                println!(
                    "JXL: HDR範囲を検出（最大値: {:.3}） - HDR情報を保持します",
                    max_val
                );
            } else if is_wide_gamut_sdr {
                println!(
                    "JXL: ワイドガムットSDR範囲（最大値: {:.3}） - ICCプロファイルで色域管理",
                    max_val
                );
            } else if is_likely_8bit_source {
                println!(
                    "JXL: 標準8-bit SDR範囲（最大値: {:.3}） - 効率的な8-bit処理",
                    max_val
                );
            }

            if *min_val < 0.0 {
                eprintln!("JXL Warning: 負の値を検出 - 0.0にクランプします");
            }
        }
    }

    // アルファチャンネル対応エンコード処理
    // GitHub Issue #96の解決策を適用: has_alpha()をビルダーで設定
    if is_rgba {
        println!("JXL: RGBA画像を処理中（アルファチャンネル保持）...");
        builder = builder.has_alpha(true);
        // エンコーダーを再構築（has_alphaはビルダー時に設定が必要）
        encoder = builder.build().map_err(|e| {
            AppError::Encode(format!("JXL encoder rebuild with alpha failed: {}", e))
        })?;
    }

    // GitHub Issue #96の解決策に基づくRGBA処理（HDR対応版）
    let final_data: Vec<f32> = if is_rgba {
        println!("JXL: RGBA画像をそのまま処理します（アルファチャンネル保持、HDR対応）");

        // RGBA画像の場合、アルファチャンネルをそのまま保持
        let mut rgba_data = pixels_f32.to_vec();

        // HDR対応：負の値のみクランプ、上限は設定しない
        for pixel in rgba_data.iter_mut() {
            if *pixel < 0.0 {
                *pixel = 0.0; // 負の値のみ0にクランプ
            }
            // 1.0を超える値は保持（HDR情報として）
        }

        rgba_data
    } else {
        // RGB画像の場合
        let mut rgb_data = pixels_f32.to_vec();

        // HDR対応：負の値のみクランプ、上限は設定しない
        for pixel in rgb_data.iter_mut() {
            if *pixel < 0.0 {
                *pixel = 0.0; // 負の値のみ0にクランプ
            }
            // 1.0を超える値は保持（HDR情報として）
        }

        rgb_data
    };

    // 最終チェック
    let expected_channels = if is_rgba { 4 } else { 3 };
    let expected_length = (width * height * expected_channels) as usize;

    println!(
        "JXL: エンコード開始 - data length: {}, expected: {} ({}チャンネル)",
        final_data.len(),
        expected_length,
        expected_channels
    );

    if final_data.len() != expected_length {
        return Err(AppError::Encode(format!(
            "JXL: data length mismatch: got {}, expected {} for {}x{} {}チャンネル画像",
            final_data.len(),
            expected_length,
            width,
            height,
            expected_channels
        )));
    }

    // GitHub Issue #96の解決策: EncoderFrameとencode_frameを使用
    println!("JXL: EncoderFrame使用によるエンコード実行中...");

    let encode_result: Result<Vec<u8>, _> = if is_likely_8bit_source {
        // 8-bit画像の場合：u8データを使用してビット深度を適切に設定
        println!("JXL: 8-bit画像として u8 データでエンコード");
        let data_u8: Vec<u8> = final_data
            .iter()
            .map(|&f| (f * 255.0).round().clamp(0.0, 255.0) as u8)
            .collect();

        let encoder_frame_u8 =
            EncoderFrame::new(data_u8.as_slice()).num_channels(expected_channels as u32);
        encoder
            .encode_frame::<u8, u8>(&encoder_frame_u8, width, height)
            .map(|result| result.to_vec())
    } else {
        // 高ビット深度画像の場合：f32データを使用
        println!("JXL: 高ビット深度画像として f32 データでエンコード");
        let encoder_frame_f32 =
            EncoderFrame::new(final_data.as_slice()).num_channels(expected_channels as u32);
        encoder
            .encode_frame::<f32, f32>(&encoder_frame_f32, width, height)
            .map(|result| result.to_vec())
    };

    let buffer = match encode_result {
        Ok(result) => {
            println!("JXL: エンコード成功 - 出力サイズ: {} bytes", result.len());
            result
        }
        Err(e) => {
            eprintln!("JXL: 1回目のエンコード失敗 - エラー詳細: {:?}", e);

            // jpegxl-rsの既知の不具合に対する段階的フォールバック戦略
            println!("JXL: jpegxl-rsの不具合対策として緊急フォールバックを試行します...");

            // 最も安全な設定で再試行
            let mut fallback_encoder = encoder_builder()
                .speed(Cheetah) // 中程度の速度
                .quality(1.0) // デフォルト品質
                .use_container(false) // コンテナなし
                .color_encoding(jpegxl_rs::encode::ColorEncoding::Srgb)
                .build()
                .map_err(|e| {
                    AppError::Encode(format!("JXL fallback encoder build failed: {}", e))
                })?;

            println!("JXL: 緊急フォールバック設定でエンコード再試行中...");

            // フォールバック時はRGB（3チャンネル）に変換
            let fallback_data = if is_rgba {
                let mut rgb = Vec::with_capacity((final_data.len() / 4) * 3);
                for chunk in final_data.chunks_exact(4) {
                    rgb.push(chunk[0]); // R
                    rgb.push(chunk[1]); // G
                    rgb.push(chunk[2]); // B
                    // アルファチャンネルは破棄
                }
                rgb
            } else {
                final_data.clone()
            };

            // フォールバック時も8-bit判定に基づいて適切な型を使用
            let fallback_result: Result<Vec<u8>, _> = if is_likely_8bit_source {
                println!("JXL: フォールバック - 8-bit u8 データでエンコード");
                let fallback_u8: Vec<u8> = fallback_data
                    .iter()
                    .map(|&f| (f * 255.0).round().clamp(0.0, 255.0) as u8)
                    .collect();
                let fallback_frame_u8 = EncoderFrame::new(fallback_u8.as_slice()).num_channels(3);
                fallback_encoder
                    .encode_frame::<u8, u8>(&fallback_frame_u8, width, height)
                    .map(|result| result.to_vec())
            } else {
                println!("JXL: フォールバック - f32 データでエンコード");
                let fallback_frame_f32 =
                    EncoderFrame::new(fallback_data.as_slice()).num_channels(3);
                fallback_encoder
                    .encode_frame::<f32, f32>(&fallback_frame_f32, width, height)
                    .map(|result| result.to_vec())
            };

            match fallback_result {
                Ok(result) => {
                    println!(
                        "JXL: 緊急フォールバック成功 - 出力サイズ: {} bytes",
                        result.len()
                    );
                    println!("JXL: 注意: 元の設定ではなく安全な設定を使用しました");
                    result
                }
                Err(fallback_err) => {
                    eprintln!("JXL: 緊急フォールバックも失敗しました");
                    eprintln!("JXL: 元のエラー: {:?}", e);
                    eprintln!("JXL: フォールバックエラー: {:?}", fallback_err);
                    eprintln!("JXL: 設定情報:");
                    eprintln!("  - Width: {}, Height: {}", width, height);
                    eprintln!("  - Is RGBA: {}", is_rgba);
                    eprintln!("  - Data length: {}", final_data.len());
                    eprintln!("  - Lossless (requested): {}", options.lossless);
                    eprintln!("  - Lossless (actual): {}", use_lossless);
                    eprintln!("  - Quality: {}", options.quality);
                    eprintln!("  - Speed: {:?}", options.speed);
                    eprintln!("  - Use container: {}", options.use_container);
                    eprintln!(
                        "  - Uses original profile: {}",
                        options.uses_original_profile
                    );
                    eprintln!("  - Color encoding: {:?}", options.color_encoding);
                    if let Some(reason) = fallback_reason {
                        eprintln!("  - フォールバック理由: {}", reason);
                    }
                    eprintln!("JXL: jpegxl-rs v0.11.2の既知の不具合により変換に失敗しました");
                    eprintln!(
                        "JXL: より新しいバージョンのライブラリまたは代替ライブラリの使用を検討してください"
                    );
                    return Err(AppError::Encode(format!(
                        "JXL encode failed even with fallback: original={:?}, fallback={:?}",
                        e, fallback_err
                    )));
                }
            }
        }
    };

    Ok(buffer)
}

/// JPEGをJPEG XL形式にロスレス変換する
///
/// # 引数
/// * `jpeg_data` - 変換元のJPEGバイトデータ
/// * `options` - JXLエンコードオプション (JxlOptions)
/// # 戻り値
/// - 成功した場合は JPEG XL のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
#[allow(dead_code)]
pub fn transcode(jpeg_data: &[u8], options: &JxlOptions) -> Result<Vec<u8>, AppError> {
    // こちらの関数はピクセルデータを直接扱わないため、修正は不要です。
    // uses_original_profile(true) はJPEGからの再圧縮で有効です。
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(options.speed.to_jxl())
        .use_container(options.use_container)
        .uses_original_profile(true)
        .decoding_speed(options.decoding_speed)
        .init_buffer_size(options.init_buffer_size)
        .color_encoding(options.color_encoding.to_jxl());

    // JPEGトランスコードでもロスレス不具合の対策を適用
    // JPEGは元々非可逆であるため、ロスレスの意味は薄いが、
    // ライブラリの不具合を回避するため非可逆モードを使用
    let use_transcode_lossless = false; // 安全のため常に非可逆

    if use_transcode_lossless && options.lossless {
        builder = builder.lossless(true);
    } else {
        // JPEGトランスコードでは高品質設定を使用
        let transcode_quality = if options.lossless {
            0.5
        } else {
            options.quality
        };
        builder = builder.quality(transcode_quality.clamp(0.1, 15.0));

        if options.lossless {
            println!("JXL: JPEGトランスコードではライブラリ不具合のため高品質モードを使用します");
        }
    }

    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL transcoder build failed: {}", e)))?;

    let buffer: EncoderResult<u8> = encoder
        .encode_jpeg(jpeg_data)
        .map_err(|e| AppError::Encode(format!("JXL transcode failed: {}", e)))?;

    Ok(buffer.to_vec())
}
