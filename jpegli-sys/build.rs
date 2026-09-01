use std::env;
use std::path::PathBuf;

// jpegli's own C++ sources (lib/jpegli/*.cc from libjxl), matching the
// JPEGXL_INTERNAL_JPEGLI_SOURCES list in libjxl's lib/jxl_lists.cmake, minus
// header-only files. Kept in sync manually since we don't run CMake here.
const SOURCES: &[&str] = &[
    "adaptive_quantization.cc",
    "bit_writer.cc",
    "bitstream.cc",
    "color_quantize.cc",
    "color_transform.cc",
    "common.cc",
    "decode.cc",
    "decode_marker.cc",
    "decode_scan.cc",
    "destination_manager.cc",
    "downsample.cc",
    "encode.cc",
    "encode_finish.cc",
    "encode_streaming.cc",
    "entropy_coding.cc",
    "error.cc",
    "huffman.cc",
    "idct.cc",
    "input.cc",
    "memory_manager.cc",
    "quant.cc",
    "render.cc",
    "simd.cc",
    "source_manager.cc",
    "upsample.cc",
];

fn main() {
    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=VCPKG_ROOT");
    println!("cargo:rerun-if-env-changed=VCPKGRS_TRIPLET");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let vendor = manifest_dir.join("vendor");
    let jpegli_dir = vendor.join("lib/jpegli");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .pic(true)
        .warnings(false)
        .include(&vendor)
        .include(vendor.join("include")) // <jxl/types.h>
        .include(vendor.join("include/jpegli")); // <jpeglib.h>, <jconfig.h>, <jmorecfg.h>

    // Match a Release-style CMake build: assertions/JXL_ENABLE_ASSERT-style
    // debug checks stay off outside of debug builds.
    if env::var("PROFILE").as_deref() != Ok("debug") {
        build.define("NDEBUG", None);
    }

    for src in SOURCES {
        build.file(jpegli_dir.join(src));
    }

    link_highway(&mut build);

    build.compile("jpegli-static");
}

/// jpegli's SIMD code is built on Google's Highway library. This project's
/// vcpkg setup already pulls in `highway` as a dependency of the `libjxl`
/// port used elsewhere in the workspace, so we link against that instead of
/// vendoring/building highway ourselves.
fn link_highway(build: &mut cc::Build) {
    #[cfg(target_os = "windows")]
    if env::var("VCPKGRS_TRIPLET").is_err() {
        let target = env::var("TARGET").unwrap_or_default();
        let triplet = if target.contains("aarch64") {
            "arm64-windows-static-release"
        } else {
            "x64-windows-static-release"
        };
        unsafe { env::set_var("VCPKGRS_TRIPLET", triplet) };
    }

    // On Unix triplets vcpkg-rs expects the "lib" prefix spelled out in
    // lib_name() (it strips it only when emitting the -l flag); on Windows
    // triplets the .lib file has no such prefix.
    let hwy_lib_name = if cfg!(target_env = "msvc") {
        "hwy"
    } else {
        "libhwy"
    };
    match vcpkg::Config::new().lib_name(hwy_lib_name).probe("highway") {
        Ok(lib) => {
            for include_path in &lib.include_paths {
                build.include(include_path);
            }
        }
        Err(e) => {
            panic!(
                "highway (hwy) not found via vcpkg: {e}\n\
                 Install it with: vcpkg install highway"
            );
        }
    }

    if cfg!(any(target_os = "macos", target_os = "ios")) {
        println!("cargo:rustc-link-lib=c++");
    } else if !cfg!(target_env = "msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
