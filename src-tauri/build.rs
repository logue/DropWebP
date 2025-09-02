fn main() {
    #[cfg(target_os = "macos")]
    {
        // Homebrew で libheif / libaom をインストール済みを想定
        if pkg_config::probe_library("libheif").is_err() {
            panic!("libheif not found. Install via Homebrew: brew install libheif");
        }
        if pkg_config::probe_library("aom").is_err() {
            panic!("libaom not found. Install via Homebrew: brew install libaom");
        }
    }
    tauri_build::build()
}
