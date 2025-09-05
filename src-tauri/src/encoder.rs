use crate::options;
use image::{self, DynamicImage};
use imgref::Img;
use libwebp_sys::{
    WebPEncodeLosslessRGB, WebPEncodeLosslessRGBA, WebPEncodeRGB, WebPEncodeRGBA, WebPFree,
};
use ravif::{AlphaColorMode, BitDepth, ColorModel, Encoder};
use rgb::{RGB8, RGBA8};
use std::{error::Error, ffi::c_void, ptr::null_mut, slice::from_raw_parts};

pub fn encode(
    img: &DynamicImage,
    options: options::EncodeOptions,
) -> Result<Vec<u8>, Box<dyn Error>> {
    if let Some(avif_opts) = options.avif {
        // ここで `AvifOptions` から `ravif` 用の引数への変換を行う
        println!("Adapter: Converting AvifOptions for ravif encoder...");

        // ravifを使ったエンコード処理...
        return convert_dynamic_image_to_avif(
            img,
            avif_opts.quality,
            avif_opts.bit_depth.to_ravif(),
            avif_opts.alpha_quality,
            avif_opts.speed,
            avif_opts.color_model.to_ravif(),
            avif_opts.threads,
            avif_opts.alpha_color_mode.to_ravif(),
        );
    } else if let Some(webp_opts) = options.webp {
        println!("Adapter: Converting WebpOptions for libwebp_sys encoder...");
        return convert_dynamic_image_to_webp(img, webp_opts.quality, webp_opts.lossless);
    }
    Ok(vec![]) // 仮の戻り値
}

/// 画像を WebP にエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `quality`: 品質 (0〜100)
/// - `lossless`: ロスレス
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - `libwebp-sys` クレートを使用して WebP エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
fn convert_dynamic_image_to_webp(
    img: &DynamicImage,
    quality: f32,
    lossless: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    if quality < 0.0 || quality > 100.0 {
        return Err("Quality must be between 0 and 100".into());
    }

    let width = img.width() as i32;
    let height = img.height() as i32;

    // RGBA または RGB の判定と格納処理
    let (raw, is_rgba) = match img {
        DynamicImage::ImageRgba8(img) => (img.clone().into_raw(), true),
        DynamicImage::ImageRgb8(img) => (img.clone().into_raw(), false),
        _ => {
            let img_buf = img.to_rgba8();
            (img_buf.into_raw(), true)
        }
    };

    unsafe {
        // 出力バッファのポインタ
        let mut out_buf: *mut u8 = null_mut();
        // ストライドの計算
        let stride = if is_rgba {
            width
                .checked_mul(4)
                .ok_or("Stride calculation overflowed")?
        } else {
            width
                .checked_mul(3)
                .ok_or("Stride calculation overflowed")?
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
            return Err("WebP encoding failed".into());
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
/// * `dynamic_image` - 変換元のDynamicImage
/// * `quality` - 品質 (0-100)。0は可逆圧縮、100は最高品質。
/// * `bit_depth` - ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten, BitDepth::Twelve)
/// * `alpha_quality` - アルファチャンネルの品質
/// * `speed` - エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。
/// * `color_model` - カラーモデル (ColorModel::YCbCr, ColorModel::RGB)
/// * `threads` - 使用するスレッド数 (Noneの場合は自動設定)
/// * `alpha_color_mode` - アルファチャネルの色モード (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// # 戻り値
/// * 成功した場合はAVIF形式のバイト列をVec<u8>として返します。
/// * 失敗した場合はBox<dyn Error>を返します。
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
) -> Result<Vec<u8>, Box<dyn Error>> {
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
            encoder.encode_rgb(image_view)?
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
            encoder.encode_rgba(image_view)?
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

            encoder.encode_rgba(image_view)?
        }
    };
    println!("Finished encoding AVIF.");

    Ok(encoded_avif.avif_file)
}
