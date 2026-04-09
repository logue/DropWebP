# vcpkg 建置指南（macOS/Linux）

本文介紹在 macOS/Linux 上使用 vcpkg 靜態連結 C/C++ 相依庫的方法。

## 平台

- macOS: x64 / ARM64
- Linux: x64 / ARM64

## 安裝 vcpkg

```bash
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"
```

## 安裝相依套件

自動安裝：

```bash
cd app/src-tauri
chmod +x setup-vcpkg.sh
./setup-vcpkg.sh
```

手動範例（x64-linux）：

```bash
vcpkg install aom:x64-linux
vcpkg install libavif[aom]:x64-linux
vcpkg install libjxl:x64-linux
vcpkg install libwebp:x64-linux
vcpkg install openjpeg:x64-linux
vcpkg install libjpeg-turbo:x64-linux
vcpkg install lcms:x64-linux
```

## 建置

```bash
export VCPKG_ROOT="$HOME/vcpkg"
cd app/src-tauri
cargo build --release
```
