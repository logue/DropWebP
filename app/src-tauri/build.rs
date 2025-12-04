fn main() {
    // 確認対象のライブラリ
    let libs = ["libavif", "libjxl"];
    // Windows環境でvcpkgを使用してライブラリを検出
    #[cfg(target_os = "windows")]
    {
        for lib in libs {
            match vcpkg::find_package(lib) {
                Ok(info) => {
                    println!("cargo:info=Found {}: {:?}", lib, info);

                    // libjxlの場合、jpegxl-sys用に追加の環境変数を設定
                    if lib == "libjxl" {
                        // jpegxl-sysがvcpkgのライブラリを見つけられるように
                        // リンクパスとインクルードパスを設定
                        for path in &info.link_paths {
                            println!("cargo:rustc-link-search=native={}", path.display());
                        }
                        for path in &info.include_paths {
                            println!("cargo:include={}", path.display());
                        }
                        println!("cargo:rustc-link-lib=static=jxl");
                        println!("cargo:rustc-link-lib=static=jxl_cms");
                        println!("cargo:rustc-link-lib=static=jxl_threads");
                    }
                }
                Err(e) => {
                    println!("cargo:warning={} not found via vcpkg: {}", lib, e);
                    println!(
                        "cargo:warning=Please install {} using: vcpkg install {}",
                        lib, lib
                    );
                }
            }
        }
    }

    // macOS環境でpkg-configを使用
    #[cfg(target_os = "macos")]
    {
        // クロスコンパイルのターゲットを取得
        let target = std::env::var("TARGET").unwrap_or_default();
        let host = std::env::var("HOST").unwrap_or_default();
        let is_cross_compiling = target != host;

        // クロスコンパイル時は警告のみ表示
        if is_cross_compiling {
            println!("cargo:warning=Cross-compiling for target: {}", target);
            println!("cargo:warning=Host architecture: {}", host);
            println!(
                "cargo:warning=Note: libavif and libjxl should be available for the target architecture"
            );

            // x86_64向けの場合、Homebrewのx86_64パスを追加
            if target.contains("x86_64") {
                println!("cargo:rustc-link-search=native=/usr/local/lib");
                println!("cargo:warning=Added /usr/local/lib to library search path for x86_64");
            }
        } else {
            // 同じアーキテクチャの場合は通常通りpkg-configでチェック
            for lib in libs {
                match pkg_config::probe_library(lib) {
                    Ok(info) => println!("cargo:info=Found {}: {:?}", lib, info),
                    Err(e) => {
                        println!("cargo:warning={} not found via pkg-config: {}", lib, e);
                        println!("cargo:warning=Attempting to continue with default library paths");
                    }
                }
            }
        }
    }

    // Linux環境でpkg-configを使用
    #[cfg(target_os = "linux")]
    {
        // クロスコンパイルのターゲットを取得
        let target = std::env::var("TARGET").unwrap_or_default();
        let host = std::env::var("HOST").unwrap_or_default();
        let is_cross_compiling = target != host;

        println!("cargo:info=Building for target: {}", target);

        // Tauri依存のGTK/WebKitライブラリをチェック
        let gtk_libs = ["gtk+-3.0", "webkit2gtk-4.1", "glib-2.0", "gobject-2.0"];

        if is_cross_compiling {
            println!("cargo:warning=Cross-compiling for target: {}", target);
            println!("cargo:warning=Host architecture: {}", host);
            println!(
                "cargo:warning=Note: GTK and WebKit libraries should be available for the target architecture"
            );

            // PKG_CONFIG_PATHの確認
            if let Ok(pkg_path) = std::env::var("PKG_CONFIG_PATH") {
                println!("cargo:info=PKG_CONFIG_PATH: {}", pkg_path);
            } else {
                println!("cargo:warning=PKG_CONFIG_PATH not set for cross-compilation");
            }
        } else {
            // 同じアーキテクチャの場合はライブラリをチェック
            for lib in gtk_libs {
                match pkg_config::probe_library(lib) {
                    Ok(info) => println!("cargo:info=Found {}: version {}", lib, info.version),
                    Err(e) => {
                        println!("cargo:warning={} not found via pkg-config: {}", lib, e);
                        println!("cargo:warning=Please install development packages:");
                        println!(
                            "cargo:warning=  Debian/Ubuntu: sudo apt-get install libgtk-3-dev libwebkit2gtk-4.1-dev"
                        );
                        println!(
                            "cargo:warning=  Fedora/RHEL: sudo dnf install gtk3-devel webkit2gtk4.1-devel"
                        );
                        println!("cargo:warning=  Arch: sudo pacman -S gtk3 webkit2gtk-4.1");
                    }
                }
            }
        }

        // libavif, libjxlのチェック（オプショナル）
        for lib in libs {
            match pkg_config::probe_library(lib) {
                Ok(info) => println!("cargo:info=Found {}: {:?}", lib, info),
                Err(e) => {
                    println!("cargo:warning={} not found via pkg-config: {}", lib, e);
                    println!("cargo:warning=Will use vendored version from Rust crates");
                }
            }
        }
    }

    tauri_build::build()
}
