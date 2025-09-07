fn main() {
    let libs = ["libheif", "aom"];
    #[cfg(target_os = "macos")]
    {
        for lib in libs {
            match pkg_config::probe_library(lib) {
                Ok(info) => println!("cargo:warning=Found {}: {:?}", lib, info),
                Err(e) => panic!("{} not found via pkg-config: {}", lib, e),
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        for lib in libs {
            match pkg_config::Config::new().statik(true).probe(lib) {
                Ok(info) => println!("cargo:warning=Found {}: {:?}", lib, info),
                Err(e) => panic!("{} not found via pkg-config: {}", lib, e),
            }
        }
    }

    tauri_build::build()
}
