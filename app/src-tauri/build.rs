fn main() {
    // 確認対象のライブラリ
    let libs = ["libavif", "libjxl"];

    // Windows環境でvendored featureを使用する場合、CMakeジェネレータを設定
    #[cfg(target_os = "windows")]
    {
        // jpegxl-rsのvendoredビルドのために、MSVCツールチェーンを強制
        // ClangCLではなくMSVCを使用するように設定
        // SAFETY: ビルドスクリプト内での環境変数設定は安全
        // 他のスレッドとの競合はビルドプロセスの性質上発生しない
        unsafe {
            std::env::set_var("CMAKE_GENERATOR", "Visual Studio 17 2022");
            std::env::remove_var("CMAKE_GENERATOR_TOOLSET"); // ClangCL指定を削除

            // jpegxl-src内部でClangCLを使わないようにする
            std::env::set_var("JPEGXL_NO_CLANGCL", "1");

            // libaom-sysでアセンブラなしでビルド（NASMなしでもビルド可能）
            // libaom-sys専用の環境変数でCMakeに直接オプションを渡す
            std::env::set_var("AOM_CMAKE_ARGS", "-DAOM_TARGET_CPU=generic");
        }

        println!("cargo:warning=Using vendored libraries for Windows build");
        println!("cargo:warning=CMake generator: Visual Studio 17 2022 (MSVC)");
        println!(
            "cargo:warning=Building libaom without assembler optimizations (generic CPU target)"
        );
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
