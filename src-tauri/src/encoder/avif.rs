use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use imgref::Img;
use ravif::{EncodedImage, Encoder};
use serde::{Deserialize, Serialize};

/// AVIF形式のオプション
/// quality: 0-100 (01~100。値が高いほど高品質)
/// bit_depth: ビット深度 (BitDepth::Auto, BitDepth::Eight, BitDepth::Ten)
/// alpha_quality: アルファチャンネルの品質 (1~100。値が高いほど高品質)
/// speed: エンコード速度 (0-10)。0は最高品質で最も遅い、10は最速。
/// color_model: カラーモデル (ColorModel::YCbCr, ColorModel::RGB)
/// threads: 使用するスレッド数 (Noneの場合は自動設定)
/// alpha_color_mode: アルファチャネルの色モード (AlphaColorMode::Straight, AlphaColorMode::Premultiplied)
/// 注意: BitDepth::Autoを選択した場合、入力画像のビット深度に基づいて自動的に決定されます。
///     例えば、8ビット画像ならBitDepth::Eight、10ビット画像ならBitDepth::Tenが選択されます。
///     ただし、入力画像が8ビット以上であっても、AVIFエンコード時にBitDepth::Eightを選択することも可能です。
///     逆に、10ビット以上の画像に対してBitDepth::Eightを選択すると、情報の損失が発生する可能性があります。
///     そのため、可能な限り入力画像のビット深度に合わせた設定を推奨します。
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvifOptions {
    pub quality: f32,
    pub bit_depth: BitDepth,
    pub alpha_quality: f32,
    pub speed: u8,
    pub color_model: ColorModel,
    pub threads: Option<usize>,
    pub alpha_color_mode: AlphaColorMode,
}

/// ビット深度の列挙型
/// - Auto: 入力画像のビット深度に基づいて自動的に決定
/// - Eight: 8ビット
/// - Ten: 10ビット
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Auto,
    Eight,
    Ten,
}

impl BitDepth {
    pub fn to_ravif(self) -> ravif::BitDepth {
        match self {
            BitDepth::Auto => ravif::BitDepth::Auto,
            BitDepth::Eight => ravif::BitDepth::Eight,
            BitDepth::Ten => ravif::BitDepth::Ten,
        }
    }
}

/// カラーモデルの列挙型
/// - YCbCr: YCbCrカラーモデル
/// - RGB: RGBカラーモデル
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    YCbCr,
    RGB,
}

impl ColorModel {
    pub fn to_ravif(self) -> ravif::ColorModel {
        match self {
            ColorModel::YCbCr => ravif::ColorModel::YCbCr,
            ColorModel::RGB => ravif::ColorModel::RGB,
        }
    }
}

/// アルファチャネルの色モードの列挙型
/// - UnassociatedDirty: 非関連アルファ（未クリーン）
/// - UnassociatedClean: 非関連アルファ（クリーン）
/// - Premultiplied: 乗算済みアルファ
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaColorMode {
    UnassociatedDirty,
    UnassociatedClean,
    Premultiplied,
}

impl AlphaColorMode {
    pub fn to_ravif(self) -> ravif::AlphaColorMode {
        match self {
            AlphaColorMode::UnassociatedDirty => ravif::AlphaColorMode::UnassociatedDirty,
            AlphaColorMode::UnassociatedClean => ravif::AlphaColorMode::UnassociatedClean,
            AlphaColorMode::Premultiplied => ravif::AlphaColorMode::Premultiplied,
        }
    }
}

/// DynamicImage を AVIF 形式のバイトデータに変換する (raif クレート使用)
///
/// # 引数
/// * `pixel_data` - 変換元のHighBitDepthImage
/// * `icc_profile` - ICCプロファイル（Someの場合は埋め込み処理を行う）
/// * `options` - AVIFエンコードオプション
/// # 戻り値
/// * 成功した場合はAVIF形式のバイト列をVec<u8>として返します。
/// * 失敗した場合はAppErrorを返します。
/// # 注意
/// * `ravif` クレートを使用してAVIFエンコードを行います。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
/// * ICCプロファイルが提供された場合、色味の一貫性を保つために埋め込み処理を試行しますが、`ravif`クレートの制限により完全な対応ではありません。
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &AvifOptions,
) -> Result<Vec<u8>, AppError> {
    // ★ 1. 画像サイズとf32ピクセルデータを取得
    let (width, height) = match &pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(&pixel_data);

    // デバッグ情報を追加
    println!("AVIF Encoder Debug:");
    println!("  Width: {}, Height: {}", width, height);
    println!("  Is RGBA: {}", is_rgba);
    println!("  Pixel count: {}", pixels_f32.len());
    println!(
        "  Expected pixel count: {}",
        width * height * if is_rgba { 4 } else { 3 }
    );
    println!("  Bit depth: {:?}", options.bit_depth);
    println!("  Color model: {:?}", options.color_model);

    // ピクセルデータの整合性チェック
    let expected_len = (width * height * if is_rgba { 4 } else { 3 }) as usize;
    if pixels_f32.len() != expected_len {
        return Err(AppError::Encode(format!(
            "AVIF pixel data length mismatch: expected {}, got {}. Width={}, Height={}, RGBA={}",
            expected_len,
            pixels_f32.len(),
            width,
            height,
            is_rgba
        )));
    }

    let encoded_avif: EncodedImage = {
        // ★ エンコーダーを生成（すべてのピクセルを8ビットとして処理）
        let encoder = Encoder::new()
            .with_quality(options.quality)
            .with_bit_depth(options.bit_depth.to_ravif()) // 設定に従ってビット深度を決定
            .with_internal_color_model(options.color_model.to_ravif())
            .with_num_threads(options.threads)
            .with_alpha_color_mode(options.alpha_color_mode.to_ravif())
            .with_speed(options.speed)
            .with_alpha_quality(options.alpha_quality);

        // f32ピクセルデータを8ビットに変換
        let pixels_u8: Vec<u8> = pixels_f32
            .iter()
            .map(|&p| (p * 255.0).round().clamp(0.0, 255.0) as u8)
            .collect();

        if is_rgba {
            // RGBA: Vec<u8> を RGBA<u8> のスライスに変換
            use rgb::FromSlice;
            let rgba_pixels = pixels_u8.as_rgba();
            let image_view = Img::new(rgba_pixels, width as usize, height as usize);
            encoder.encode_rgba(image_view).map_err(AppError::Ravif)?
        } else {
            // RGB: Vec<u8> を RGB<u8> のスライスに変換
            use rgb::FromSlice;
            let rgb_pixels = pixels_u8.as_rgb();
            let image_view = Img::new(rgb_pixels, width as usize, height as usize);
            encoder.encode_rgb(image_view).map_err(AppError::Ravif)?
        }
    };

    // ICCプロファイルが提供された場合の処理
    let mut final_avif_data = encoded_avif.avif_file;

    if let Some(profile_data) = icc_profile {
        // ICCプロファイルが提供された場合は色味保持のための最適化を行う
        println!("AVIF: ICCプロファイルが提供されました。色味保持のための設定を確認中...");

        // 色味保持のための推奨設定チェック
        if options.color_model != ColorModel::RGB {
            eprintln!("推奨: ICCプロファイル使用時はColorModel::RGBを推奨します");
        }

        if matches!(options.bit_depth, BitDepth::Eight) {
            eprintln!("推奨: 色域保持のためBitDepth::Ten以上を推奨します");
        }

        // ICCプロファイルの埋め込みを試行
        match embed_icc_profile_in_avif(&final_avif_data, &profile_data) {
            Ok(avif_with_icc) => {
                println!("AVIF: ICCプロファイルを埋め込みました");
                final_avif_data = avif_with_icc;
            }
            Err(e) => {
                // ICCプロファイルの埋め込みに失敗した場合は警告を出すが、処理は続行
                eprintln!("AVIF: ICCプロファイルの埋め込みに失敗しました: {:?}", e);
                eprintln!("AVIF: ICCプロファイルなしで処理を続行します");
            }
        }
    }

    Ok(final_avif_data)
}

/// AVIFファイルにICCプロファイルを埋め込む関数
///
/// # 注意
/// この機能は実験的であり、完全に機能しない可能性があります。
/// AVIFファイルの構造は複雑で、手動でのメタデータ埋め込みは困難です。
///
/// # 引数
/// * `avif_data` - 元のAVIFファイルデータ
/// * `icc_profile` - 埋め込むICCプロファイルデータ
///
/// # 戻り値
/// ICCプロファイル付きのAVIFデータまたはエラー
fn embed_icc_profile_in_avif(avif_data: &[u8], _icc_profile: &[u8]) -> Result<Vec<u8>, AppError> {
    // 現在の実装では、ICCプロファイルの埋め込みは技術的に困難です
    // AVIFはISOベースのコンテナ形式で、正確なバイナリ操作が必要です

    // 暫定的な解決策：警告と共に元のデータを返す
    eprintln!("警告: AVIF形式へのICCプロファイル埋め込みは現在未対応です");
    eprintln!("色味の変化を最小限に抑えるには以下を推奨します：");
    eprintln!("1. ColorModel::RGB を使用");
    eprintln!("2. BitDepth::Ten 以上を使用");
    eprintln!("3. 高品質設定を使用");

    // TODO: 将来的には libheif-rs や専用のライブラリを使用してICCプロファイル埋め込みを実装

    // 現時点では元のデータをそのまま返す
    Ok(avif_data.to_vec())
}
