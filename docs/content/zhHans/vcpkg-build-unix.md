# vcpkg 构建指南（macOS/Linux）

本文介绍在 macOS/Linux 上使用 vcpkg 静态链接 C/C++ 依赖库的方法。

## 平台

- macOS: x64 / ARM64
- Linux: x64 / ARM64

## 安装 vcpkg

```bash
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"
```

## 安装依赖

自动安装：

```bash
cd app/src-tauri
chmod +x setup-vcpkg.sh
./setup-vcpkg.sh
```

手动示例（x64-linux）：

```bash
vcpkg install aom:x64-linux
vcpkg install libavif[aom]:x64-linux
vcpkg install libjxl:x64-linux
vcpkg install libwebp:x64-linux
vcpkg install openjpeg:x64-linux
vcpkg install libjpeg-turbo:x64-linux
vcpkg install lcms:x64-linux
```

## 构建

```bash
export VCPKG_ROOT="$HOME/vcpkg"
cd app/src-tauri
cargo build --release
```
