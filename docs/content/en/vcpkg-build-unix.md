# vcpkg Build Guide (macOS/Linux)

This guide explains how to use vcpkg to statically link C/C++ dependencies on macOS and Linux.

## Platforms

- macOS: x64 / ARM64
- Linux: x64 / ARM64

## Prerequisites

- Rust toolchain (rustup recommended)
- Git
- Compiler toolchain (Xcode CLT on macOS, GCC/Clang on Linux)

## Install vcpkg

```bash
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"
```

## Install Dependencies

Automatic:

```bash
cd app/src-tauri
chmod +x setup-vcpkg.sh
./setup-vcpkg.sh
```

Manual example (x64-linux):

```bash
vcpkg install aom:x64-linux
vcpkg install libavif[aom]:x64-linux
vcpkg install libjxl:x64-linux
vcpkg install libwebp:x64-linux
vcpkg install openjpeg:x64-linux
vcpkg install libjpeg-turbo:x64-linux
vcpkg install lcms:x64-linux
```

## Build

```bash
export VCPKG_ROOT="$HOME/vcpkg"
cd app/src-tauri
cargo build --release
```

## Troubleshooting

- `vcpkg: command not found`: verify `VCPKG_ROOT` and `PATH`
- `library not found`: verify installed packages and triplet
- Header not found: verify architecture/triplet
