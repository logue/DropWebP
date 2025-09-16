use crate::{encoder::extract_pixel_data, error::AppError};
use image::DynamicImage;
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

/// DynamicImage を JPEG XL 形式のバイトデータに変換する (jpegxl-rs クレート使用)
///
/// # 引数
/// * `img` - 変換元のDynamicImage
/// * `options` - JXLエンコードオプション (JxlOptions)
/// # 戻り値
/// - 成功した場合は JPEG XL のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - `jpegxl-rs` クレートを使用して JPEG XL エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
pub fn encode(img: &DynamicImage, options: &JxlOptions) -> Result<Vec<u8>, AppError> {
    let width = img.width();
    let height = img.height();

    // 1. 効率的なデータ準備 (Cow<T>の利用)
    let (pixel_data, is_rgba) = extract_pixel_data(img);

    // 2. エンコーダーの組み立て (ビルダーパターンの活用)
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(options.speed.to_jxl())
        .use_container(options.use_container)
        .uses_original_profile(options.uses_original_profile)
        .decoding_speed(options.decoding_speed)
        .init_buffer_size(options.init_buffer_size)
        .color_encoding(options.color_encoding.to_jxl());

    /*
    // 並列処理ランナーの設定
    if let Some(runner) = options.parallel_runner {
        builder = builder.parallel_runner(runner);
    }
    */

    // 可逆/非可逆と品質の設定
    if options.lossless {
        builder = builder.lossless(true);
    } else {
        // libjxlの品質設定は「バターワース距離」です。
        // 1.0が視覚的にロスレスに近い高品質、数値が大きいほど低品質になります。
        // 0.0は特別な意味を持つ場合があるため、通常は0.1以上が安全です。
        builder = builder.quality(options.quality.clamp(0.1, 15.0));
        //.uses_original_profile(uses_original_profile);
    }

    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL encoder build failed: {}", e)))?;

    // 3. ピクセルフォーマット情報の設定
    encoder.has_alpha = is_rgba;

    // 4. エンコード処理と結果の返却
    //    - unwrap() を避け、`?` でエラーハンドリングします。
    //    - `encode` の戻り値は `Result<Vec<u8>, _>` なので、そのまま返します。
    let buffer: EncoderResult<f32> = encoder
        .encode(&pixel_data, width, height)
        .map_err(|e| AppError::Encode(format!("JXL encode failed: {}", e)))?;

    Ok(buffer.to_vec())
}

/// JPEG XL 形式にトランスコードする
/// - `img` - 変換元のJPEGバイトデータ
/// - `options` - JXLエンコードオプション (JxlOptions)
/// # 戻り値
/// - 成功した場合は JPEG XL のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
pub fn transcode(img: &[u8], options: &JxlOptions) -> Result<Vec<u8>, AppError> {
    // 2. エンコーダーの組み立て (ビルダーパターンの活用)
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(options.speed.to_jxl())
        .use_container(options.use_container)
        .uses_original_profile(true)
        .decoding_speed(options.decoding_speed)
        .init_buffer_size(options.init_buffer_size)
        .color_encoding(options.color_encoding.to_jxl());

    /*
    // 並列処理ランナーの設定
    if let Some(runner) = options.parallel_runner {
        builder = builder.parallel_runner(runner);
    }
    */

    // 可逆/非可逆と品質の設定
    if options.lossless {
        builder = builder.lossless(true);
    } else {
        // libjxlの品質設定は「バターワース距離」です。
        // 1.0が視覚的にロスレスに近い高品質、数値が大きいほど低品質になります。
        // 0.0は特別な意味を持つ場合があるため、通常は0.1以上が安全です。
        builder = builder.quality(options.quality.clamp(0.1, 15.0));
        //.uses_original_profile(uses_original_profile);
    }

    let mut encoder = builder
        .build()
        .map_err(|e| AppError::Encode(format!("JXL encoder build failed: {}", e)))?;

    // 4. エンコード処理と結果の返却
    //    - unwrap() を避け、`?` でエラーハンドリングします。
    //    - `encode` の戻り値は `Result<Vec<u8>, _>` なので、そのまま返します。
    let buffer: EncoderResult<u8> = encoder
        .encode_jpeg(img)
        .map_err(|e| AppError::Encode(format!("JXL encode failed: {}", e)))?;

    Ok(buffer.to_vec())
}
