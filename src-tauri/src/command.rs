use drop_webp_lib::encode_webp;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// ディレクトリ内のファイル一覧を取得します。
/// # 引数
/// - `path`: 対象ディレクトリのパス
/// # 戻り値
/// - 成功した場合はファイル名のベクターを返します。
/// - 失敗した場合はエラーメッセージを返します。
/// # 注意
/// - サブディレクトリ内のファイルは含まれません。
///
#[tauri::command]
pub fn list_full_paths(path: String) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            paths.push(path_str.to_string());
        }
    }

    Ok(paths)
}

/// 単一の画像を WebP にエンコードします。
/// # 引数
/// - `input`: 対象ファイルパス
/// - `output`: 出力先ファイルパス (None の場合は元ファイルと同じディレクトリに保存されます)
/// - `quality`: 品質 (0〜100)。100 の場合はロスレスになります。
///
/// # 戻り値
/// - 成功した場合は `true` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
#[tauri::command]
pub fn convert_image(
    input: PathBuf,
    output: Option<PathBuf>,
    quality: i32,
) -> Result<bool, String> {
    // 画像を開く
    let img: DynamicImage = image::open(&input).map_err(|e| e.to_string())?;

    // WebP に変換
    let encoded = encode_webp(&img, quality).map_err(|e| e.to_string())?;

    // 出力先: 元ファイルと同じディレクトリに保存
    let parent = input.parent().ok_or("Invalid file path".to_string())?;
    let filename = input
        .file_stem()
        .ok_or("Invalid file name".to_string())?
        .to_string_lossy();
    let out_path = output.unwrap_or(parent.join(format!("{}.webp", filename)));

    // 書き込み
    std::fs::write(&out_path, &encoded).map_err(|e| e.to_string())?;

    Ok(true)
}
