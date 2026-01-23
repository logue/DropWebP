# vcpkg Build Guide (macOS/Linux)

このドキュメントは、macOS/Linux環境でvcpkgを使用してC/C++依存ライブラリを静的リンクする方法を説明します。

## 対応プラットフォーム

- **macOS**: x64 (Intel) / ARM64 (Apple Silicon)
- **Linux**: x64 / ARM64

## 前提条件

### macOS

- Xcode Command Line Tools: `xcode-select --install`
- Rust toolchain (rustup推奨)
- Git
- Homebrew (オプション、従来のビルド方法用)

### Linux

- GCC/Clang
- Rust toolchain (rustup推奨)
- Git
- Build essentials: `sudo apt install build-essential curl zip unzip tar pkg-config`

## vcpkgのセットアップ

### 1. vcpkgのインストール

```bash
# vcpkgをクローン
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg

# vcpkgをブートストラップ
cd ~/vcpkg
./bootstrap-vcpkg.sh

# 環境変数を設定（~/.bashrc または ~/.zshrc に追加）
export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"

# シェル設定を再読み込み
source ~/.bashrc  # または source ~/.zshrc
```

### 2. 依存関係のインストール

#### 自動インストール（推奨）

```bash
cd app/src-tauri
chmod +x setup-vcpkg.sh
./setup-vcpkg.sh
```

スクリプトは自動的にアーキテクチャとOSを検出し、適切なtripletを使用します：

- **macOS (Apple Silicon)**: `arm64-osx`
- **macOS (Intel)**: `x64-osx`
- **Linux (ARM64)**: `arm64-linux`
- **Linux (x64)**: `x64-linux`

#### 手動インストール

**macOS (Apple Silicon)**:

```bash
vcpkg install aom:arm64-osx
vcpkg install libavif[aom]:arm64-osx
vcpkg install libjxl:arm64-osx
vcpkg install libwebp:arm64-osx
vcpkg install openjpeg:arm64-osx
vcpkg install libjpeg-turbo:arm64-osx
vcpkg install lcms:arm64-osx
```

**macOS (Intel)**:

```bash
vcpkg install aom:x64-osx
vcpkg install libavif[aom]:x64-osx
vcpkg install libjxl:x64-osx
vcpkg install libwebp:x64-osx
vcpkg install openjpeg:x64-osx
vcpkg install libjpeg-turbo:x64-osx
vcpkg install lcms:x64-osx
```

**Linux (x64)**:

```bash
vcpkg install aom:x64-linux
vcpkg install libavif[aom]:x64-linux
vcpkg install libjxl:x64-linux
vcpkg install libwebp:x64-linux
vcpkg install openjpeg:x64-linux
vcpkg install libjpeg-turbo:x64-linux
vcpkg install lcms:x64-linux
```

**Linux (ARM64)**:

```bash
vcpkg install aom:arm64-linux
vcpkg install libavif[aom]:arm64-linux
vcpkg install libjxl:arm64-linux
vcpkg install libwebp:arm64-linux
vcpkg install openjpeg:arm64-linux
vcpkg install libjpeg-turbo:arm64-linux
vcpkg install lcms:arm64-linux
```

### 3. ビルド

```bash
# 環境変数を設定（必須）
export VCPKG_ROOT="$HOME/vcpkg"

# ビルド
cd app/src-tauri
cargo build --release
```

## インストールされるライブラリ

- **libaom**: AV1エンコーダー（AVIF形式用）
- **libavif**: AVIF画像フォーマット
- **libjxl**: JPEG XL画像フォーマット
- **libwebp**: WebP画像フォーマット
- **openjpeg**: JPEG 2000画像フォーマット
- **libjpeg-turbo**: JPEG画像処理（jpegli用）
- **lcms**: Little CMS カラーマネジメント

## トラブルシューティング

### vcpkg が見つからない

**問題**: `vcpkg: command not found`

**解決策**:

1. `VCPKG_ROOT`環境変数が設定されているか確認
2. PATHにvcpkgが追加されているか確認

```bash
# 環境変数を確認
echo $VCPKG_ROOT
which vcpkg

# 設定を追加（~/.bashrc または ~/.zshrc）
export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"
```

### ライブラリが見つからない

**問題**: `cargo build` 実行時に「library not found」エラー

**解決策**:

1. vcpkgでライブラリがインストールされているか確認
2. `build.rs`がvcpkg経由でライブラリを検出するか確認

```bash
# 確認コマンド
vcpkg list | grep aom
vcpkg list | grep avif
vcpkg list | grep jxl

# インストール状況を確認
ls -la ~/vcpkg/installed/*/lib/
```

### ビルド時にヘッダーファイルが見つからない

**問題**: `fatal error: 'xxx.h' file not found`

**解決策**:

1. tripletが正しいか確認
2. vcpkgのインストールが完了しているか確認

```bash
# tripletを確認（macOS例）
uname -m  # arm64 または x86_64
ls -la ~/vcpkg/installed/
```

### Homebrew との競合

vcpkgとHomebrewの両方がインストールされている場合、競合が発生する可能性があります。

**推奨設定**:

- vcpkgを優先的に使用する場合、環境変数`VCPKG_ROOT`を設定
- Homebrewを使用する場合、vcpkgをアンインストールまたは環境変数を未設定に

## vcpkg vs Homebrew/apt

### vcpkg を使用する利点

1. **クロスプラットフォーム統一**: Windows/macOS/Linux で同じビルド手順
2. **バージョン管理**: 特定バージョンで固定可能
3. **静的リンク**: ポータブルなバイナリ
4. **依存関係の一元管理**: すべてのC/C++ライブラリをvcpkgで管理

### 従来の方法（Homebrew/apt）を使用する利点

1. **セットアップが簡単**: パッケージマネージャーでインストール
2. **ディスクスペース**: システムライブラリを共有
3. **アップデート管理**: OS標準のパッケージ管理

## 参考リンク

- [vcpkg](https://github.com/Microsoft/vcpkg)
- [vcpkg-rs (Cargo integration)](https://github.com/mcgoo/vcpkg-rs)
- [libaom](https://aomedia.googlesource.com/aom/)
- [libavif](https://github.com/AOMediaCodec/libavif)
- [libjxl](https://github.com/libjxl/libjxl)

## 次のステップ

ビルドが成功したら、以下のドキュメントを参照してください：

- [MACOS_COMPATIBILITY.md](MACOS_COMPATIBILITY.md) - Apple Silicon互換性ノート
- [INTEL_MAC_BUILD.md](INTEL_MAC_BUILD.md) - Intel Mac向けクロスコンパイル
- [DOCKER_BUILD.md](docker-build.md) - Docker を使用したLinuxビルド
