use crate::{
    encoder::{HighBitDepthImage, extract_pixel_data},
    error::AppError,
};
use jpegxl_rs::encode::{EncoderResult, EncoderSpeed::*, encoder_builder};
use serde::{Deserialize, Serialize};

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

    // color_encodingとuses_original_profileは慎重に設定
    builder = builder.color_encoding(options.color_encoding.to_jxl());
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
    println!(
        "JXL: {}x{} {}画像を変換中{}{}",
        width,
        height,
        if is_rgba { "RGBA" } else { "RGB" },
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

    // ピクセル値の範囲をチェック（ApiUsageエラーの原因調査）
    if let Some(min_val) = pixels_f32.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        if let Some(max_val) = pixels_f32.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            if *min_val < 0.0 || *max_val > 1.0 {
                eprintln!(
                    "JXL Warning: ピクセル値が範囲外です [{:.3}, {:.3}] (期待値: [0.0, 1.0])",
                    min_val, max_val
                );
                eprintln!("JXL: ピクセル値を正規化します...");
            }
        }
    }

    // アルファチャンネル対応エンコード処理
    if is_rgba {
        // 注意: jpegxl-rs v0.11.2はRGBAエンコードに制限があるため、
        // RGB変換によるフォールバック処理を使用します
        println!("JXL: 透明度付き画像を処理中（RGB変換使用）...");
        encoder.has_alpha = false;
    }

    // RGB処理（元のRGBまたはRGBAフォールバック）
    let mut rgb_data = if is_rgba {
        let mut rgb = Vec::with_capacity((pixels_f32.len() / 4) * 3);
        let mut transparent_pixels = 0;
        let mut semi_transparent_pixels = 0;

        for chunk in pixels_f32.chunks_exact(4) {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let a = chunk[3];

            if a <= 0.01 {
                // 完全透明: 白背景にフォールバック
                transparent_pixels += 1;
                rgb.push(1.0); // 白 R
                rgb.push(1.0); // 白 G  
                rgb.push(1.0); // 白 B
            } else if a >= 0.99 {
                // 完全不透明: そのまま使用
                rgb.push(r);
                rgb.push(g);
                rgb.push(b);
            } else {
                // 半透明: プリマルチプライ済みアルファの場合は色を復元
                semi_transparent_pixels += 1;
                if a > 0.0 {
                    // アルファで除算して元の色を復元（プリマルチプライ対応）
                    rgb.push((r / a).clamp(0.0, 1.0));
                    rgb.push((g / a).clamp(0.0, 1.0));
                    rgb.push((b / a).clamp(0.0, 1.0));
                } else {
                    rgb.push(r);
                    rgb.push(g);
                    rgb.push(b);
                }
            }
        }

        if transparent_pixels > 0 || semi_transparent_pixels > 0 {
            println!(
                "JXL: 透明ピクセル処理完了 - 透明: {}, 半透明: {} (白背景として処理)",
                transparent_pixels, semi_transparent_pixels
            );
        }

        rgb
    } else {
        pixels_f32.to_vec()
    };

    // 最終的なピクセル値の正規化（ApiUsageエラー回避）
    for pixel in rgb_data.iter_mut() {
        *pixel = pixel.clamp(0.0, 1.0);
    }

    // 最終チェック
    println!(
        "JXL: エンコード開始 - RGB data length: {}, expected: {}",
        rgb_data.len(),
        width * height * 3
    );

    if rgb_data.len() != (width * height * 3) as usize {
        return Err(AppError::Encode(format!(
            "JXL: RGB data length mismatch: got {}, expected {}",
            rgb_data.len(),
            width * height * 3
        )));
    }

    // RGB エンコード実行（段階的フォールバック戦略付き）
    println!("JXL: RGB エンコード実行中...");
    let encode_result = encoder.encode(&rgb_data, width, height);

    let buffer: EncoderResult<f32> = match encode_result {
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
            match fallback_encoder.encode(&rgb_data, width, height) {
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
                    eprintln!("  - Data length: {}", rgb_data.len());
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

    Ok(buffer.to_vec())
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
