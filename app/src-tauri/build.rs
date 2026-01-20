use std::env;

fn main() {
    // vcpkg経由でlibjxlとlibavifを取得（静的リンク優先）
    let use_vcpkg = env::var("CARGO_FEATURE_VENDORED").is_err(); // vendored featureがない場合にvcpkgを使用

    if use_vcpkg {
        println!("cargo:info=Using vcpkg for C/C++ dependency management");

        // libavif-sys と libaom-sys にシステムライブラリを使わせる
        // これによりvcpkgでインストールしたライブラリが優先される
        println!("cargo:rustc-env=SYSTEM_DEPS_LINK=static");
        println!("cargo:rustc-env=SYSTEM_DEPS_BUILD_INTERNAL=never");

        // libjxl (JPEG XL encoder/decoder)
        match vcpkg::Config::new()
            .lib_name("jxl")
            .lib_name("jxl_cms")
            .lib_name("jxl_threads")
            .cargo_metadata(true) // Cargoにメタデータを伝達してリンカーエラーを防ぐ
            .probe("libjxl")
        {
            Ok(lib) => {
                println!("cargo:info=Found libjxl via vcpkg: {:?}", lib.link_paths);

                // jxl-sysがvcpkgのlibjxlを使用するように環境変数を設定
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }

                // jxl-sysに外部ビルドを使わせる
                unsafe {
                    env::set_var("JXL_STATIC", "1");
                    env::set_var("LIBJXL_NO_LOCAL_BUILD", "1");
                    if let Some(lib_path) = lib.link_paths.first() {
                        env::set_var("JXL_LIB_DIR", lib_path);
                    }
                    if let Some(include_path) = lib.include_paths.first() {
                        env::set_var("JXL_INCLUDE_DIR", include_path);
                    }
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
                    println!("cargo:warning=  Windows: vcpkg install libjxl:x64-windows-static");
                }
            }
        }

        // libaom (AV1 encoder for AVIF)
        match vcpkg::Config::new().lib_name("aom").probe("aom") {
            Ok(lib) => {
                println!("cargo:info=Found libaom via vcpkg: {:?}", lib.link_paths);

                // libaom-sys にvcpkgのライブラリを使うように指示
                for link_path in &lib.link_paths {
                    println!("cargo:rustc-link-search=native={}", link_path.display());
                }
                println!("cargo:rustc-link-lib=static=aom");

                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }

                // libavif-sys と libaom-sys にビルドをスキップさせる
                unsafe {
                    env::set_var("AOM_LIB_DIR", lib.link_paths[0].to_str().unwrap());
                    env::set_var("AOM_INCLUDE_DIR", lib.include_paths[0].to_str().unwrap());
                    env::set_var("AOM_STATIC", "1");
                }
            }
            Err(e) => {
                println!("cargo:warning=libaom not found via vcpkg: {}", e);
                println!("cargo:warning=Please install libaom:");
                println!("cargo:warning=  Windows: vcpkg install aom:x64-windows-static");
            }
        }

        // libavif (AVIF encoder/decoder)
        match vcpkg::Config::new().lib_name("avif").probe("libavif") {
            Ok(lib) => {
                println!("cargo:info=Found libavif via vcpkg: {:?}", lib.link_paths);

                // libavif-sys にvcpkgのライブラリを使うように指示
                for link_path in &lib.link_paths {
                    println!("cargo:rustc-link-search=native={}", link_path.display());
                }
                println!("cargo:rustc-link-lib=static=avif");

                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }

                // libavif-sys にビルドをスキップさせる
                unsafe {
                    env::set_var("AVIF_LIB_DIR", lib.link_paths[0].to_str().unwrap());
                    env::set_var("AVIF_INCLUDE_DIR", lib.include_paths[0].to_str().unwrap());
                    env::set_var("AVIF_STATIC", "1");
                }
            }
            Err(e) => {
                println!("cargo:warning=libavif not found via vcpkg: {}", e);
                println!("cargo:warning=Please install libavif:");
                println!("cargo:warning=  Windows: vcpkg install libavif[aom]:x64-windows-static");
            }
        }

        // libwebp (WebP encoder/decoder)
        match vcpkg::Config::new()
            .lib_name("libwebp")
            .lib_name("libwebpmux")
            .lib_name("libwebpdemux")
            .probe("libwebp")
        {
            Ok(lib) => {
                println!("cargo:info=Found libwebp via vcpkg: {:?}", lib.link_paths);
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=libwebp not found via vcpkg: {}", e);
                println!("cargo:warning=Please install libwebp:");
                println!("cargo:warning=  Windows: vcpkg install libwebp:x64-windows-static");
            }
        }

        // openjpeg (JPEG 2000 decoder)
        match vcpkg::Config::new().lib_name("openjp2").probe("openjpeg") {
            Ok(lib) => {
                println!("cargo:info=Found openjpeg via vcpkg: {:?}", lib.link_paths);
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=openjpeg not found via vcpkg: {}", e);
                println!("cargo:warning=Please install openjpeg:");
                println!("cargo:warning=  Windows: vcpkg install openjpeg:x64-windows-static");
                println!("cargo:warning=  macOS: brew install openjpeg");
                println!("cargo:warning=  Linux: sudo apt install libopenjp2-7-dev");
            }
        }

        // libjpeg-turbo (for jpegli_rs)
        match vcpkg::Config::new().lib_name("jpeg").probe("libjpeg-turbo") {
            Ok(lib) => {
                println!(
                    "cargo:info=Found libjpeg-turbo via vcpkg: {:?}",
                    lib.link_paths
                );
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=libjpeg-turbo not found via vcpkg: {}", e);
                println!("cargo:warning=Please install libjpeg-turbo:");
                println!("cargo:warning=  Windows: vcpkg install libjpeg-turbo:x64-windows-static");
            }
        }

        // lcms2 (Little CMS color management)
        match vcpkg::Config::new().lib_name("lcms2").probe("lcms2") {
            Ok(lib) => {
                println!("cargo:info=Found lcms2 via vcpkg: {:?}", lib.link_paths);
                for include_path in &lib.include_paths {
                    println!("cargo:include={}", include_path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=lcms2 not found via vcpkg: {}", e);
                println!("cargo:warning=Please install lcms2:");
                println!("cargo:warning=  Windows: vcpkg install lcms:x64-windows-static");
            }
        }
    }

    // Windows環境でのMSVC設定
    #[cfg(target_os = "windows")]
    {
        println!("cargo:warning=Building for Windows with MSVC");

        // vcpkg triplet設定（静的リンク）
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let target = env::var("TARGET").unwrap_or_default();
            let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

            // プロファイルに応じてトリプレットを選択
            // リリースビルドのみリリーストリプレットを使用
            let triplet = if target.contains("aarch64") {
                if profile == "release" {
                    "arm64-windows-static-release"
                } else {
                    "arm64-windows-static"
                }
            } else {
                if profile == "release" {
                    "x64-windows-static-release"
                } else {
                    "x64-windows-static"
                }
            };

            // VCPKGRS_TRIPLETが既に設定されていない場合のみ設定
            if env::var("VCPKGRS_TRIPLET").is_err() {
                unsafe {
                    env::set_var("VCPKGRS_TRIPLET", triplet);
                }
            }

            let actual_triplet =
                env::var("VCPKGRS_TRIPLET").unwrap_or_else(|_| triplet.to_string());
            println!("cargo:info=Using vcpkg triplet: {}", actual_triplet);

            unsafe {
                // pkg-config path for vcpkg libraries
                let pkg_config_path = format!(
                    "{}\\installed\\{}\\lib\\pkgconfig",
                    vcpkg_root, actual_triplet
                );
                env::set_var("PKG_CONFIG_PATH", &pkg_config_path);

                // Tell libavif-sys and libaom-sys to use pkg-config
                env::set_var("LIBAVIF_NO_LOCAL_BUILD", "1");
                env::set_var("LIBAOM_NO_LOCAL_BUILD", "1");
            }
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
