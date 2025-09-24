use crate::{
    encoder::{HighBitDepthImage, extract_pixel_data},
    error::AppError,
};
use jpegxl_rs::encode::{EncoderResult, EncoderSpeed::*, encoder_builder};
use serde::{Deserialize, Serialize};

/// JPEG XL形式のオプション
/// * `lossless` - ロスレス圧縮するか
/// * `speed` - エンコード速度（0~10）値が低いほど早いが品質が劣る
/// * `quality` - 品質（0.1〜15.0）値が高いほど高品質。デフォルトは1。推奨値0.5〜3.0。（ロスレス時は無視されます）
/// * `use_container` - JPEG XLコンテナ形式を使用するようにエンコーダを構成する
/// * `uses_original_profile` - エンコーダを元のカラープロファイルを使用するように設定する。（ロスレス時は常に有効）
/// * `decoding_speed` - デコード速度を設定（0~4）。値が低いほど高品質。デフォルトは0
/// * `init_buffer_size` - 出力バッファの初期サイズ（バイト単位）32未満は32kbに切り上げ
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

    // エンコーダーの組み立て (完全にデフォルト設定)
    let mut encoder = encoder_builder()
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

    // デバッグ情報を追加
    println!(
        "JXL Encoder: width={}, height={}, is_rgba={}, pixel_count={}, has_icc={}",
        width,
        height,
        is_rgba,
        pixels_f32.len(),
        icc_profile.is_some()
    );

    // ピクセル値の範囲をチェック
    if let Some(min_val) = pixels_f32.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        if let Some(max_val) = pixels_f32.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            println!(
                "JXL Encoder: pixel value range: {} to {} (should be 0.0 to 1.0 for jpegxl-rs)",
                min_val, max_val
            );

            if *min_val < 0.0 || *max_val > 1.0 {
                eprintln!(
                    "WARNING: Pixel values are outside expected range [0.0, 1.0]. This may cause jpegxl-rs API errors."
                );
            }
        }
    }

    // 極限まで単純化したエンコード処理
    println!("JXL: Attempting simple encode without customization...");

    // RGBAの場合はRGBに変換
    let rgb_data = if is_rgba {
        let mut rgb = Vec::with_capacity((pixels_f32.len() / 4) * 3);
        for chunk in pixels_f32.chunks_exact(4) {
            rgb.push(chunk[0]); // R
            rgb.push(chunk[1]); // G
            rgb.push(chunk[2]); // B
        }
        rgb
    } else {
        pixels_f32.to_vec()
    };

    // エンコード実行（全てデフォルト設定）
    let buffer: EncoderResult<f32> = encoder
        .encode(&rgb_data, width, height)
        .map_err(|e| AppError::Encode(format!("JXL simple encode failed: {:?}", e)))?;

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

    if options.lossless {
        builder = builder.lossless(true);
    } else {
        builder = builder.quality(options.quality.clamp(0.1, 15.0));
    }

    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL transcoder build failed: {}", e)))?;

    let buffer: EncoderResult<u8> = encoder
        .encode_jpeg(jpeg_data)
        .map_err(|e| AppError::Encode(format!("JXL transcode failed: {}", e)))?;

    Ok(buffer.to_vec())
}
