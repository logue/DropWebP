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
            println!("cargo:warning=Note: libavif and libjxl should be available for the target architecture");

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

    tauri_build::build()
}
