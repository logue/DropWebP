use crate::{encoder::extract_pixel_data, error::AppError};
use image::DynamicImage;
use jpegxl_rs::{
    encode::{ColorEncoding, EncoderResult, EncoderSpeed},
    encoder_builder,
    parallel::ParallelRunner,
};

/// DynamicImage を JPEG XL 形式のバイトデータに変換する (jpegxl-rs クレート使用)
///
/// # 引数
/// * `img` - 変換元のDynamicImage
/// * `lossless` - ロスレス圧縮するか
/// * `speed` - エンコード速度（1~10）値が低いほど早いが品質が劣る
/// * `quality` - 品質（0.1〜15.0）値が高いほど高品質。デフォルトは1。推奨値0.5〜3.0。（ロスレス時は無視されます）
/// * `use_container` - JPEG XLコンテナ形式を使用するようにエンコーダを構成する
/// * `uses_original_profile` - エンコーダを元のカラープロファイルを使用するように設定する。（ロスレス時は常に有効）
/// * `decoding_speed` - デコード速度を設定（0~4）。値が低いほど高品質。デフォルトは0
/// * `init_buffer_size` - 出力バッファの初期サイズ（バイト単位）32未満は32kbに切り上げ
/// * `color_encoding` - カラーエンコード方法を設定する。デフォルトはsRGB
/// * `parallel_runner` - 並列ランナーを設定する。デフォルト: None
/// - 成功した場合は JPEG XL のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - `jpegxl-rs` クレートを使用して JPEG XL エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
pub fn encode(
    img: &DynamicImage,
    lossless: bool,
    speed: EncoderSpeed,
    quality: f32,
    use_container: bool,
    _uses_original_profile: bool,
    decoding_speed: i64,
    init_buffer_size: usize,
    color_encoding: ColorEncoding,
    parallel_runner: Option<&dyn ParallelRunner>,
) -> Result<Vec<u8>, AppError> {
    let width = img.width();
    let height = img.height();

    // 1. 効率的なデータ準備 (Cow<T>の利用)
    let (pixel_data, is_rgba) = extract_pixel_data(img);

    // 2. エンコーダーの組み立て (ビルダーパターンの活用)
    //    - unwrap() を避け、`?` 演算子でエラーを伝播させます。
    let mut binding = encoder_builder();
    let mut builder = binding
        .speed(speed)
        .use_container(use_container)
        .decoding_speed(decoding_speed)
        .init_buffer_size(init_buffer_size)
        .color_encoding(color_encoding);

    // 並列処理ランナーの設定
    if let Some(runner) = parallel_runner {
        builder = builder.parallel_runner(runner);
    }

    // 可逆/非可逆と品質の設定
    if lossless {
        builder = builder.lossless(true).uses_original_profile(true);
    } else {
        // libjxlの品質設定は「バターワース距離」です。
        // 1.0が視覚的にロスレスに近い高品質、数値が大きいほど低品質になります。
        // 0.0は特別な意味を持つ場合があるため、通常は0.1以上が安全です。
        builder = builder.quality(quality.clamp(0.1, 15.0));
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
