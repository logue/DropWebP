use std::env;

fn main() {
    // Cargo依存ファイルの変更を監視
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=capabilities/");

    // 環境変数の変更を監視
    println!("cargo:rerun-if-env-changed=VCPKG_ROOT");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=HOST");

    let target = env::var("TARGET").unwrap_or_default();
    let host = env::var("HOST").unwrap_or_default();

    println!("cargo:info=Building for target: {}", target);
    println!("cargo:info=Host architecture: {}", host);

    // macOSでのクロスコンパイルのためにvcpkgのtripletを設定
    #[cfg(target_os = "macos")]
    if env::var("VCPKGRS_TRIPLET").is_err() {
        let triplet = if target.contains("x86_64") {
            "x64-osx"
        } else if target.contains("aarch64") {
            "arm64-osx"
        } else {
            "arm64-osx" // デフォルトはarm64
        };
        unsafe {
            env::set_var("VCPKGRS_TRIPLET", triplet);
        }
        println!("cargo:info=Setting vcpkg triplet to: {}", triplet);
    }

    // vcpkg の PKG_CONFIG_PATH を設定
    if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
        let triplet = env::var("VCPKGRS_TRIPLET").unwrap_or_else(|_| {
            if target.contains("x86_64") {
                "x64-osx".to_string()
            } else {
                "arm64-osx".to_string()
            }
        });

        let pkg_config_path = format!("{}/installed/{}/lib/pkgconfig", vcpkg_root, triplet);
        println!("cargo:info=Setting PKG_CONFIG_PATH to: {}", pkg_config_path);

        unsafe {
            // 既存のPKG_CONFIG_PATHに追加
            let current_path = env::var("PKG_CONFIG_PATH").unwrap_or_default();
            let new_path = if current_path.is_empty() {
                pkg_config_path
            } else {
                format!("{}:{}", pkg_config_path, current_path)
            };
            env::set_var("PKG_CONFIG_PATH", new_path);
        }
    }

    // vcpkgを使用したC/C++ライブラリ管理
    // システムパッケージマネージャー（brew、apt、rpm等）への依存を避け、
    // vcpkgで統一的にライブラリを管理
    println!("cargo:rustc-env=SYSTEM_DEPS_LINK=static");
    println!("cargo:rustc-env=SYSTEM_DEPS_BUILD_INTERNAL=never");

    // libjxl (JPEG XL encoder/decoder)
    match vcpkg::Config::new()
        .lib_name("jxl")
        .lib_name("jxl_cms")
        .lib_name("jxl_threads")
        .cargo_metadata(true)
        .probe("libjxl")
    {
        Ok(lib) => {
            println!("cargo:info=Found libjxl via vcpkg");
            for include_path in &lib.include_paths {
                println!("cargo:include={}", include_path.display());
            }
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
            println!("cargo:warning=Please install via vcpkg: vcpkg install libjxl");
        }
    }

    // libaom (AV1 encoder for AVIF)
    match vcpkg::Config::new().lib_name("aom").probe("aom") {
        Ok(lib) => {
            println!("cargo:info=Found libaom via vcpkg");
            for link_path in &lib.link_paths {
                println!("cargo:rustc-link-search=native={}", link_path.display());
            }
            println!("cargo:rustc-link-lib=static=aom");
            for include_path in &lib.include_paths {
                println!("cargo:include={}", include_path.display());
            }
            // libaom-sysに内部ビルドをスキップさせる
            // libaom-sys 0.17.xが認識する正式な環境変数を設定
            unsafe {
                if !lib.link_paths.is_empty() && !lib.include_paths.is_empty() {
                    let lib_dir = lib.link_paths[0].to_str().unwrap();
                    let inc_dir = lib.include_paths[0].to_str().unwrap();
                    let pkgconfig_dir = format!("{}/pkgconfig", lib_dir);

                    // libaom-sysのbuild.rsが参照する環境変数（必須）
                    env::set_var("LIB_AOM_STATIC_LIB_PATH", lib_dir);
                    env::set_var("LIB_AOM_INCLUDE_PATH", inc_dir);
                    env::set_var("LIB_AOM_PKG_CONFIG_PATH", &pkgconfig_dir);

                    println!("cargo:info=Set LIB_AOM_STATIC_LIB_PATH={}", lib_dir);
                    println!("cargo:info=Set LIB_AOM_INCLUDE_PATH={}", inc_dir);
                    println!("cargo:info=Set LIB_AOM_PKG_CONFIG_PATH={}", pkgconfig_dir);
                }
            }
        }
        Err(e) => {
            println!("cargo:warning=libaom not found via vcpkg: {}", e);
            println!("cargo:warning=Please install via vcpkg: vcpkg install aom");
        }
    }

    // libavif (AVIF encoder/decoder)
    match vcpkg::Config::new().lib_name("avif").probe("libavif") {
        Ok(lib) => {
            println!("cargo:info=Found libavif via vcpkg");
            for link_path in &lib.link_paths {
                println!("cargo:rustc-link-search=native={}", link_path.display());
            }
            println!("cargo:rustc-link-lib=static=avif");
            for include_path in &lib.include_paths {
                println!("cargo:include={}", include_path.display());
            }
            // libavif-sysに内部ビルドをスキップさせる
            unsafe {
                if !lib.link_paths.is_empty() {
                    env::set_var("AVIF_LIB_DIR", lib.link_paths[0].to_str().unwrap());
                }
                if !lib.include_paths.is_empty() {
                    env::set_var("AVIF_INCLUDE_DIR", lib.include_paths[0].to_str().unwrap());
                }
                env::set_var("AVIF_STATIC", "1");
                env::set_var("LIBAVIF_NO_LOCAL_BUILD", "1");
            }
        }
        Err(e) => {
            println!("cargo:warning=libavif not found via vcpkg: {}", e);
            println!("cargo:warning=Please install via vcpkg: vcpkg install 'libavif[aom]'");
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
            println!("cargo:info=Found libwebp via vcpkg");
            for include_path in &lib.include_paths {
                println!("cargo:include={}", include_path.display());
            }
        }
        Err(e) => {
            println!("cargo:warning=libwebp not found via vcpkg: {}", e);
            println!("cargo:warning=Please install via vcpkg: vcpkg install libwebp");
        }
    }

    // openjpeg (JPEG 2000 decoder)
    match vcpkg::Config::new().lib_name("openjp2").probe("openjpeg") {
        Ok(lib) => {
            println!("cargo:info=Found openjpeg via vcpkg");
            for include_path in &lib.include_paths {
                println!("cargo:include={}", include_path.display());
            }
        }
        Err(e) => {
            println!("cargo:warning=openjpeg not found via vcpkg: {}", e);
            println!("cargo:warning=Please install via vcpkg: vcpkg install openjpeg");
        }
    }

    // libjpeg-turbo (jpegli_rsが使用)
    match vcpkg::Config::new().lib_name("jpeg").probe("libjpeg-turbo") {
        Ok(_lib) => {
            println!("cargo:info=Found libjpeg-turbo via vcpkg");
        }
        Err(e) => {
            println!("cargo:warning=libjpeg-turbo not found via vcpkg: {}", e);
            println!("cargo:warning=Please install via vcpkg: vcpkg install libjpeg-turbo");
        }
    }

    // lcms2 (Little CMS color management)
    match vcpkg::Config::new().lib_name("lcms2").probe("lcms2") {
        Ok(lib) => {
            println!("cargo:info=Found lcms2 via vcpkg");
            for include_path in &lib.include_paths {
                println!("cargo:include={}", include_path.display());
            }
        }
        Err(e) => {
            println!("cargo:warning=lcms2 not found via vcpkg: {}", e);
            println!("cargo:warning=Please install via vcpkg: vcpkg install lcms");
        }
    }

    // Windows環境での設定
    #[cfg(target_os = "windows")]
    {
        if let Ok(_vcpkg_root) = env::var("VCPKG_ROOT") {
            let target = env::var("TARGET").unwrap_or_default();
            let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

            // プロファイルに応じてトリプレットを選択
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

            if env::var("VCPKGRS_TRIPLET").is_err() {
                unsafe {
                    env::set_var("VCPKGRS_TRIPLET", triplet);
                }
            }

            let actual_triplet =
                env::var("VCPKGRS_TRIPLET").unwrap_or_else(|_| triplet.to_string());
            println!("cargo:info=Using vcpkg triplet: {}", actual_triplet);

            unsafe {
                // libavif-sysとlibaom-sysに内部ビルドをスキップさせる
                // Windows特有の環境変数も設定
                env::set_var("LIBAVIF_NO_LOCAL_BUILD", "1");
                env::set_var("LIBAOM_NO_LOCAL_BUILD", "1");
                env::set_var("LIBAOM_NO_FETCH", "1");
                env::set_var("LIBAOM_NO_BUILD", "1");
                env::set_var("LIBAOM_SYS_STATIC", "1");
            }
        } else {
            println!(
                "cargo:warning=VCPKG_ROOT not set. Please set it to your vcpkg installation directory."
            );
        }
    }

    // macOS環境での設定
    #[cfg(target_os = "macos")]
    {
        let target = env::var("TARGET").unwrap_or_default();
        let host = env::var("HOST").unwrap_or_default();
        let is_cross_compiling = target != host;

        if is_cross_compiling {
            println!("cargo:info=Cross-compiling from {} to {}", host, target);
        }

        // VCPKG_ROOTの設定（未設定の場合はデフォルトパスを探す）
        if env::var("VCPKG_ROOT").is_err() {
            let default_vcpkg_path =
                format!("{}/Developer/vcpkg", env::var("HOME").unwrap_or_default());
            if std::path::Path::new(&default_vcpkg_path).exists() {
                unsafe {
                    env::set_var("VCPKG_ROOT", &default_vcpkg_path);
                }
                println!("cargo:info=Set VCPKG_ROOT to {}", default_vcpkg_path);
            }
        }

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

            // x86_64クロスコンパイル時は、libaom/libavifの内部ビルドを無効化
            // vcpkgでビルド済みのライブラリを使用
            if is_cross_compiling && target.contains("x86_64") {
                println!("cargo:info=Using vcpkg pre-built libraries for x86_64 cross-compilation");

                unsafe {
                    env::set_var("LIBAVIF_NO_LOCAL_BUILD", "1");
                    env::set_var("LIBAOM_NO_LOCAL_BUILD", "1");
                    env::set_var("SYSTEM_DEPS_BUILD_INTERNAL", "never");
                }
            }
        } else {
            println!(
                "cargo:warning=VCPKG_ROOT not set. Please set it to your vcpkg installation directory."
            );
        }
    }

    // Linux環境での設定
    #[cfg(target_os = "linux")]
    {
        let target = env::var("TARGET").unwrap_or_default();
        let host = env::var("HOST").unwrap_or_default();
        let is_cross_compiling = target != host;

        if is_cross_compiling {
            println!("cargo:info=Cross-compiling from {} to {}", host, target);
        }

        // GTK/WebKit依存関係のチェック（Tauri要件）
        // これらはpkg-config経由で取得する必要がある
        let gtk_libs = ["gtk+-3.0", "webkit2gtk-4.1", "glib-2.0", "gobject-2.0"];
        for lib in gtk_libs {
            match pkg_config::probe_library(lib) {
                Ok(info) => println!("cargo:info=Found {}: version {}", lib, info.version),
                Err(e) => {
                    println!("cargo:warning={} not found: {}", lib, e);
                    println!("cargo:warning=GTK dependencies are required for Tauri on Linux");
                }
            }
        }

        // vcpkg triplet設定（画像処理ライブラリ用）
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
        } else {
            println!(
                "cargo:warning=VCPKG_ROOT not set. Install image libraries via vcpkg for best compatibility."
            );
        }
    }

    tauri_build::build()
}
