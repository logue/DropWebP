use std::env;

fn main() {
    // vcpkg経由でlibjxlとlibavifを取得（静的リンク優先）
    let use_vcpkg = env::var("CARGO_FEATURE_VENDORED").is_err(); // vendored featureがない場合にvcpkgを使用

    if use_vcpkg {
        println!("cargo:info=Using vcpkg for dependency management");

        // libjxl (JPEG XL encoder/decoder)
        match vcpkg::Config::new()
            .lib_name("jxl")
            .lib_name("jxl_cms")
            .lib_name("jxl_threads")
            .probe("libjxl")
        {
            Ok(lib) => {
                println!("cargo:info=Found libjxl via vcpkg: {:?}", lib.link_paths);
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=libjxl not found via vcpkg: {}", e);
                println!("cargo:warning=Falling back to pkg-config or system libraries");

                // pkg-configでフォールバック
                if let Err(e2) = pkg_config::probe_library("libjxl") {
                    println!(
                        "cargo:warning=libjxl not found via pkg-config either: {}",
                        e2
                    );
                    println!("cargo:warning=Please install libjxl:");
                    println!("cargo:warning=  macOS: brew install jpeg-xl");
                    println!("cargo:warning=  Linux: sudo apt install libjxl-dev");
                    println!("cargo:warning=  vcpkg: vcpkg install libjxl");
                }
            }
        }

        // libavif (AVIF encoder/decoder)
        match vcpkg::Config::new().lib_name("avif").probe("libavif") {
            Ok(lib) => {
                println!("cargo:info=Found libavif via vcpkg: {:?}", lib.link_paths);
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=libavif not found via vcpkg: {}", e);
                println!("cargo:warning=Falling back to pkg-config or system libraries");

                if let Err(e2) = pkg_config::probe_library("libavif") {
                    println!(
                        "cargo:warning=libavif not found via pkg-config either: {}",
                        e2
                    );
                    println!("cargo:warning=libavif-sys will handle this dependency");
                }
            }
        }
    }

    // Windows環境でのMSVC設定
    #[cfg(target_os = "windows")]
    {
        println!("cargo:warning=Building for Windows with MSVC");

        // vcpkg triplet設定（静的リンク）
        if env::var("VCPKG_ROOT").is_ok() {
            let target = env::var("TARGET").unwrap_or_default();
            let triplet = if target.contains("x86_64") {
                "x64-windows-static"
            } else if target.contains("aarch64") {
                "arm64-windows-static"
            } else {
                "x64-windows-static"
            };

            unsafe {
                env::set_var("VCPKGRS_TRIPLET", triplet);
            }
            println!("cargo:info=Using vcpkg triplet: {}", triplet);
        }
    }

    // macOS環境での設定
    #[cfg(target_os = "macos")]
    {
        let target = env::var("TARGET").unwrap_or_default();
        let host = env::var("HOST").unwrap_or_default();
        let is_cross_compiling = target != host;

        if is_cross_compiling {
            println!("cargo:warning=Cross-compiling for target: {}", target);
            println!("cargo:warning=Host architecture: {}", host);

            if target.contains("x86_64") {
                println!("cargo:rustc-link-search=native=/usr/local/lib");
                println!("cargo:warning=Added /usr/local/lib for x86_64 target");
            }
        }

        // vcpkg triplet設定（静的リンク）
        if env::var("VCPKG_ROOT").is_ok() {
            let triplet = if target.contains("x86_64") {
                "x64-osx"
            } else {
                "arm64-osx"
            };

            unsafe {
                env::set_var("VCPKGRS_TRIPLET", triplet);
            }
            println!("cargo:info=Using vcpkg triplet: {}", triplet);
        }
    }

    // Linux環境での設定
    #[cfg(target_os = "linux")]
    {
        let target = env::var("TARGET").unwrap_or_default();
        let host = env::var("HOST").unwrap_or_default();
        let is_cross_compiling = target != host;

        println!("cargo:info=Building for target: {}", target);

        if is_cross_compiling {
            println!("cargo:warning=Cross-compiling for target: {}", target);

            if let Ok(pkg_path) = env::var("PKG_CONFIG_PATH") {
                println!("cargo:info=PKG_CONFIG_PATH: {}", pkg_path);
            } else {
                println!("cargo:warning=PKG_CONFIG_PATH not set for cross-compilation");
            }
        }

        // GTK/WebKit依存関係のチェック（Tauri要件）
        let gtk_libs = ["gtk+-3.0", "webkit2gtk-4.1", "glib-2.0", "gobject-2.0"];
        for lib in gtk_libs {
            match pkg_config::probe_library(lib) {
                Ok(info) => println!("cargo:info=Found {}: version {}", lib, info.version),
                Err(e) => {
                    println!("cargo:warning={} not found via pkg-config: {}", lib, e);
                }
            }
        }

        // vcpkg triplet設定（静的リンク）
        if env::var("VCPKG_ROOT").is_ok() {
            let triplet = if target.contains("x86_64") {
                "x64-linux"
            } else if target.contains("aarch64") {
                "arm64-linux"
            } else {
                "x64-linux"
            };

            unsafe {
                env::set_var("VCPKGRS_TRIPLET", triplet);
            }
            println!("cargo:info=Using vcpkg triplet: {}", triplet);
        }
    }

    tauri_build::build()
}
