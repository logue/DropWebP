use crate::encoder::{avif::AvifOptions, jxl::JxlOptions, webp::WebpOptions};
use serde::{Deserialize, Serialize};

/// ファイルパス情報
/// file_name: ファイル名 (拡張子含む)
/// extension: 拡張子 (ドット無し)
/// parent_dir: 親ディレクトリのパス
/// exists: パスが存在するか
/// is_file: ファイルであるか
/// is_dir: ディレクトリであるか
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")] // JS側でキャメルケースになるように設定
pub struct PathInfo {
    pub(crate) file_name: Option<String>,
    pub(crate) extension: Option<String>,
    pub(crate) parent_dir: Option<String>,
    pub(crate) exists: Option<String>,
    pub(crate) is_file: Option<bool>,
    pub(crate) is_dir: Option<bool>,
}

/// 全てのエンコードオプションをまとめる親構造体
/// エンコード形式を一つだけ指定するための列挙型
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum EncodeOptions {
    Avif(AvifOptions),
    Webp(WebpOptions),
    Jxl(JxlOptions),
}
