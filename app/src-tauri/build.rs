fn main() {
    // Windows環境でvcpkgを使用してライブラリを検出
    #[cfg(target_os = "windows")]
    {
        // vcpkgでインストールされたライブラリを検出
        let libs = ["libheif", "libavif", "libjxl"];
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
        let libs = ["libheif", "libavif", "libjxl"];
        for lib in libs {
            match pkg_config::probe_library(lib) {
                Ok(info) => println!("cargo:info=Found {}: {:?}", lib, info),
                Err(e) => panic!("{} not found via pkg-config: {}", lib, e),
            }
        }
    }

    tauri_build::build()
}
