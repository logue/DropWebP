fn main() {
    #[cfg(target_os = "windows")]
    {
        let vcpkg_root =
            std::env::var("VCPKG_ROOT").expect("VCPKG_ROOT must be set (vcpkg install path)");

        let lib_path = format!("{}\\installed\\x64-windows\\lib", vcpkg_root);
        let bin_path = format!("{}\\installed\\x64-windows\\bin", vcpkg_root);
        let include_path = format!("{}\\installed\\x64-windows\\include", vcpkg_root);

        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:include={}", include_path);

        println!("cargo:rustc-link-lib=heif");
        println!("cargo:rustc-link-lib=aom");

        // 実行時に DLL を PATH に追加
        println!("cargo:rerun-if-env-changed=PATH={}", bin_path);
    }

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
