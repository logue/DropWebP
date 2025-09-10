fn main() {
    let libs = ["libheif", "aom", "libjxl"];
    #[cfg(target_os = "macos")]
    {
        for lib in libs {
            match pkg_config::probe_library(lib) {
                Ok(info) => println!("cargo:info=Found {}: {:?}", lib, info),
                Err(e) => panic!("{} not found via pkg-config: {}", lib, e),
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        for lib in libs {
            match vcpkg::find_package(lib) {
                Ok(info) => println!("cargo:info=Found {}: {:?}", lib, info),
                Err(e) => panic!("{} not found via pkg-config: {}", lib, e),
            }
        }
    }

    tauri_build::build()
}
