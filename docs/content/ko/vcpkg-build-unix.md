# vcpkg 빌드 가이드 (macOS/Linux)

macOS/Linux에서 vcpkg로 C/C++ 의존성을 정적 링크하는 방법입니다.

## 플랫폼

- macOS: x64 / ARM64
- Linux: x64 / ARM64

## vcpkg 설치

```bash
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"
```

## 의존성 설치

자동 설치:

```bash
cd app/src-tauri
chmod +x setup-vcpkg.sh
./setup-vcpkg.sh
```

수동 예시(x64-linux):

```bash
vcpkg install aom:x64-linux
vcpkg install libavif[aom]:x64-linux
vcpkg install libjxl:x64-linux
vcpkg install libwebp:x64-linux
vcpkg install openjpeg:x64-linux
vcpkg install libjpeg-turbo:x64-linux
vcpkg install lcms:x64-linux
```

## 빌드

```bash
export VCPKG_ROOT="$HOME/vcpkg"
cd app/src-tauri
cargo build --release
```
