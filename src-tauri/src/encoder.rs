use crate::options;
use crate::{error::AppError, options::EncodeOptions};
use image::DynamicImage;
use imgref::Img;
use jpegxl_rs::{
    encode::{ColorEncoding, EncoderResult, EncoderSpeed},
    encoder_builder,
    parallel::ParallelRunner,
};
use libwebp_sys::{
    WebPEncodeLosslessRGB, WebPEncodeLosslessRGBA, WebPEncodeRGB, WebPEncodeRGBA, WebPFree,
};
use ravif::{AlphaColorMode, BitDepth, ColorModel, Encoder};
use rgb::{RGB8, RGBA8};
use std::{borrow::Cow, ffi::c_void, ptr::null_mut, slice::from_raw_parts};

/// 画像を指定された形式でエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `options`: エンコードオプション (options::EncodeOptions)
/// # 戻り値
/// - 成功した場合はエンコードされたバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - AVIF形式のエンコードには `ravif` クレートを使用しています。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
/// - WebP形式のエンコードには `libwebp-sys` クレートを使用しています。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
/// - JPEG XL形式のエンコードには `jpegxl-rs` クレートを使用しています。ビルド時に `libjxl` ライブラリがシステムにインストールされている必要があります。
pub fn encode(img: &DynamicImage, options: options::EncodeOptions) -> Result<Vec<u8>, AppError> {
    match options {
        EncodeOptions::Avif(opts) => {
            println!("Adapter: Converting AvifOptions for ravif encoder...");
            return convert_dynamic_image_to_avif(
                img,
                opts.quality,
                opts.bit_depth.to_ravif(),
                opts.alpha_quality,
                opts.speed,
                opts.color_model.to_ravif(),
                opts.threads,
                opts.alpha_color_mode.to_ravif(),
            );
        }
        EncodeOptions::Webp(opts) => {
            println!("Adapter: Converting WebpOptions for libwebp_sys encoder...");
            return convert_dynamic_image_to_webp(img, opts.quality, opts.lossless);
        }
        EncodeOptions::Jxl(opts) => {
            println!("Adapter: Converting JxlOptions for jpegxl_rs encoder...");
            return convert_dynamic_image_to_jxl(
                img,
                opts.lossless,
                opts.speed.to_jxl(),
                opts.quality,
                opts.use_container,
                opts.uses_original_profile,
                opts.decoding_speed,
                opts.init_buffer_size,
                opts.color_encoding.to_jxl(),
                None, // 並列ランナーは今のところサポートしない
            );
        }
        _ => Err(AppError::UnsupportedFormat),
    }
}

/// DynamicImageからエンコード用のピクセルデータを効率的に抽出します。
///
/// - 元の画像がRGB8/RGBA8形式の場合、データを借用して不要なコピーを避けます。
/// - それ以外の形式の場合は、RGBA8に変換して所有権を持つデータを生成します。
///
/// # Arguments
/// * `img` - 処理対象の`DynamicImage`への参照。
///
/// # Returns
/// * `(Cow<'a, [u8]>, bool)` - ピクセルデータと、アルファチャンネルの有無 (`true`ならRGBA) のタプル。
fn extract_pixel_data(img: &DynamicImage) -> (Cow<'_, [u8]>, bool) {
    match img {
        DynamicImage::ImageRgba8(buffer) => (Cow::Borrowed(buffer.as_raw()), true),
        DynamicImage::ImageRgb8(buffer) => (Cow::Borrowed(buffer.as_raw()), false),
        // 16ビット画像(Rgba16)や他の形式が来た場合...
        _ => {
            // ...img.to_rgba8() を使って8ビットのRGBA形式に変換する！
            let buffer = img.to_rgba8();
            // これで、どんな入力でも必ず8ビットRGBAデータになる
            (Cow::Owned(buffer.into_raw()), true)
        }
    }
}

/*
/// 画像のダイナミックレンジを判定するためのenum
#[derive(Debug, PartialEq)]
pub enum ImageRange {
    Sdr,
    Hdr,
}

/// 8bitのSDR画像データを保持するためのenum。RGBとRGBAを区別します。
pub enum SdrImage {
    Rgb(ImageBuffer<Rgb<u8>, Vec<u8>>),
    Rgba(ImageBuffer<Rgba<u8>, Vec<u8>>),
}

/// DynamicImageのビット深度からSDRかHDRかを判定する
fn determine_image_range(img: &DynamicImage) -> ImageRange {
    match img.color() {
        ColorType::L8 | ColorType::La8 | ColorType::Rgb8 | ColorType::Rgba8 => ImageRange::Sdr,
        ColorType::L16
        | ColorType::La16
        | ColorType::Rgb16
        | ColorType::Rgba16
        | ColorType::Rgb32F
        | ColorType::Rgba32F => ImageRange::Hdr,
        _ => ImageRange::Sdr,
    }
}

/// SDR画像を標準的なRGB8またはRGBA8形式に正規化する
fn normalize_sdr(img: &DynamicImage) -> Result<SdrImage, AppError> {
    println!("SDR image detected. Normalizing...");
    // 元画像がアルファチャンネルを持つか否かで、変換先を分岐
    match img.color().has_alpha() {
        true => Ok(SdrImage::Rgba(img.to_rgba8())),
        false => Ok(SdrImage::Rgb(img.to_rgb8())),
    }
}

/// HDR画像をACESトーンマッピングでSDR画像(RGB8/RGBA8)に変換する
fn tonemap_hdr_to_sdr(hdr_image: &DynamicImage) -> Result<SdrImage, AppError> {
    println!("HDR image detected. Applying ACES tonemapping...");

    let rgba32f_image = hdr_image.to_rgba32f();

    // HDRピクセルデータ (f32) を取得
    let mut rgb_pixels: Vec<[f32; 3]> =
        rgba32f_image.pixels().map(|p| [p[0], p[1], p[2]]).collect();

    // ★★★ここが修正点★★★
    // 各ピクセルに対して、個別にトーンマッピング関数を適用します
    for pixel in rgb_pixels.iter_mut() {
        tonemap::tonemap_aces_narkowicz(pixel);
    }

    // --- 以降のロジックは同じ ---
    let has_alpha = match hdr_image.color() {
        ColorType::La16 | ColorType::Rgba16 | ColorType::Rgba32F => true,
        _ => false,
    };

    if has_alpha {
        let sdr_buffer: Vec<u8> = rgb_pixels
            .iter()
            .zip(rgba32f_image.pixels())
            .flat_map(|(rgb, original_rgba)| {
                let r = (rgb[0].clamp(0.0, 1.0) * 255.0).round() as u8;
                let g = (rgb[1].clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (rgb[2].clamp(0.0, 1.0) * 255.0).round() as u8;
                let a = (original_rgba[3].clamp(0.0, 1.0) * 255.0).round() as u8;
                [r, g, b, a]
            })
            .collect();
        let image_buffer = ImageBuffer::from_raw(hdr_image.width(), hdr_image.height(), sdr_buffer)
            .ok_or_else(|| AppError::ImageProcessing("SDR RGBAバッファの生成に失敗".to_string()))?;
        Ok(SdrImage::Rgba(image_buffer))
    } else {
        let sdr_buffer: Vec<u8> = rgb_pixels
            .iter()
            .flat_map(|p| {
                let r = (p[0].clamp(0.0, 1.0) * 255.0).round() as u8;
                let g = (p[1].clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (p[2].clamp(0.0, 1.0) * 255.0).round() as u8;
                [r, g, b]
            })
            .collect();
        let image_buffer = ImageBuffer::from_raw(hdr_image.width(), hdr_image.height(), sdr_buffer)
            .ok_or_else(|| AppError::ImageProcessing("SDR RGBバッファの生成に失敗".to_string()))?;
        Ok(SdrImage::Rgb(image_buffer))
    }
}
*/

/// 画像を WebP にエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `quality`: 品質 (0〜100)
/// - `lossless`: ロスレス
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - `libwebp-sys` クレートを使用して WebP エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
fn convert_dynamic_image_to_webp(
    img: &DynamicImage,
    quality: f32,
    lossless: bool,
) -> Result<Vec<u8>, AppError> {
    if quality < 0.0 || quality > 100.0 {
        return Err(AppError::Encode("Quality must be between 0 and 100".into()));
    }

    let width = img.width() as i32;
    let height = img.height() as i32;

    // 1. データ準備
    let (raw, is_rgba) = extract_pixel_data(img);

    unsafe {
        // 出力バッファのポインタ
        let mut out_buf: *mut u8 = null_mut();
        // ストライドの計算
        let stride = if is_rgba {
            width.checked_mul(4).ok_or(AppError::Encode(
                "Stride calculation overflowed".to_string(),
            ))?
        } else {
            width.checked_mul(3).ok_or(AppError::Encode(
                "Stride calculation overflowed".to_string(),
            ))?
        };

        // WebP にエンコード
        // qualityが100の場合はロスレスエンコードを使用
        let len = if is_rgba {
            println!("Optimized path: Encoding as RGBA...");
            // RGBA圧縮
            if lossless == true {
                WebPEncodeLosslessRGBA(raw.as_ptr(), width, height, stride, &mut out_buf)
            } else {
                WebPEncodeRGBA(raw.as_ptr(), width, height, stride, quality, &mut out_buf)
            }
        } else {
            println!("Optimized path: Encoding as RGB...");
            // RGB圧縮
            if lossless == true {
                WebPEncodeLosslessRGB(raw.as_ptr(), width, height, stride, &mut out_buf)
            } else {
                WebPEncodeRGB(raw.as_ptr(), width, height, stride, quality, &mut out_buf)
            }
        };

        if out_buf.is_null() || len == 0 {
            return Err(AppError::Encode("WebP encoding failed".into()));
        }

        // Rust Vec にコピー
        let slice = from_raw_parts(out_buf, len as usize);
        let result = slice.to_vec();

        // C 側で確保されたメモリを解放
        WebPFree(out_buf as *mut c_void);

        println!("Finished encoding WebP.");

        Ok(result)
    }
}

/// DynamicImage を AVIF 形式のバイトデータに変換する (raif クレート使用)
///
/// # 引数
/// * `img` - 変換元のDynamicImage
/// * `quality` - 品質 (1-100)。値が高いほど最高品質。
/// * `bit_depth` - ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten)
/// * `alpha_quality` - アルファチャンネルの品質（1-100）。値が高いほど最高品質。
/// * `speed` - エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。
/// * `color_model` - カラーモデル (ColorModel::YCbCr, ColorModel::RGB)
/// * `threads` - 使用するスレッド数 (Noneの場合は自動設定)
/// * `alpha_color_mode` - アルファチャネルの色モード (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// # 戻り値
/// * 成功した場合はAVIF形式のバイト列をVec<u8>として返します。
/// * 失敗した場合はAppErrorを返します。
/// # 注意
/// * `ravif` クレートを使用してAVIFエンコードを行います。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
fn convert_dynamic_image_to_avif(
    img: &DynamicImage,
    quality: f32,
    bit_depth: BitDepth,
    alpha_quality: f32,
    speed: u8,
    color_model: ColorModel,
    threads: Option<usize>,
    alpha_color_mode: AlphaColorMode,
) -> Result<Vec<u8>, AppError> {
    // エンコーダーの設定は先に済ませておく
    let encoder = Encoder::new()
        .with_quality(quality)
        .with_bit_depth(bit_depth)
        .with_internal_color_model(color_model)
        .with_num_threads(threads)
        .with_alpha_color_mode(alpha_color_mode)
        .with_speed(speed)
        .with_alpha_quality(alpha_quality);

    // DynamicImageの具体的な型でマッチングして処理を分岐
    let encoded_avif = match img {
        // --- RGB8形式の場合 ---
        DynamicImage::ImageRgb8(rgb_image) => {
            println!("Optimized path: Encoding as RGB...");
            let width = rgb_image.width() as usize;
            let height = rgb_image.height() as usize;

            // &[u8] を &[RGB8] に変換
            let pixels_rgb8: &[RGB8] = bytemuck::cast_slice(rgb_image.as_raw());
            let image_view = Img::new(pixels_rgb8, width, height);

            // encode_rgb を使用
            encoder.encode_rgb(image_view).map_err(AppError::Ravif)?
        }
        // --- RGBA8形式の場合 ---
        DynamicImage::ImageRgba8(rgba_image) => {
            println!("Standard path: Encoding as RGBA...");
            let width = rgba_image.width() as usize;
            let height = rgba_image.height() as usize;

            // &[u8] を &[RGBA8] に変換
            let pixels_rgba8: &[RGBA8] = bytemuck::cast_slice(rgba_image.as_raw());
            let image_view = Img::new(pixels_rgba8, width, height);

            // encode_rgba を使用
            encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
        }
        // --- その他の形式の場合 (Luma8, Bgr8など) ---
        // 汎用的なRGBA8に変換してから処理する（フォールバック）
        _ => {
            println!("Fallback path: Converting to RGBA then encoding...");
            let rgba_image = img.to_rgba8();
            let width = rgba_image.width() as usize;
            let height = rgba_image.height() as usize;

            let pixels_rgba8: &[RGBA8] = bytemuck::cast_slice(rgba_image.as_raw());
            let image_view = Img::new(pixels_rgba8, width, height);

            encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
        }
    };
    println!("Finished encoding AVIF.");

    Ok(encoded_avif.avif_file)
}

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
fn convert_dynamic_image_to_jxl(
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
