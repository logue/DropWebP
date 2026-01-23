# Windows Build with vcpkg

このドキュメントは、Windows環境でvcpkgを使用してC/C++依存ライブラリ(libaom、libavif、libjxl、libwebp等)を静的リンクする方法を説明します。

## 重要: リリース専用トリプレット

**vcpkgのデフォルト`x64-windows-static`トリプレットはデバッグシンボルを含むため、Rustのリリースビルドでリンクエラーが発生します。**

このプロジェクトでは**`x64-windows-static-release`トリプレット**を使用します:

- **リリースビルド** (`cargo build --release` / `pnpm run build:tauri`): `x64-windows-static-release` (デバッグシンボルなし)

`.cargo/config.toml`で環境変数が設定されており、`LIB_AOM_STATIC_LIB_PATH`などの環境変数を通じてvcpkgのライブラリパスを指定します。

## 前提条件

- Visual Studio 2019/2022 (MSVC toolchain)
- Rust toolchain (rustup推奨)
- Git
- PowerShell
- **LLVM/Clang** (jxl-sysのbindgen用、必須)

### LLVM/Clangのインストール

jxl-sysはbindgenを使用するため、LLVM/Clangが必要です。

**方法1: wingetでインストール（推奨）**

```powershell
winget install LLVM.LLVM
```

**方法2: インストーラーからインストール**

1. [LLVM Releases](https://github.com/llvm/llvm-project/releases)からLLVM-19.x.x-win64.exeをダウンロード
2. インストーラーを実行
3. "Add LLVM to the system PATH"オプションを選択

**インストール後の確認:**

```powershell
clang --version
# 出力例: clang version 19.1.6
```

システム再起動後、環境変数`LIBCLANG_PATH`が自動設定されます。手動設定する場合：

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

## vcpkgのセットアップ

### 0. リリーストリプレットの作成

vcpkgのデフォルトトリプレットはデバッグビルドを含むため、カスタムトリプレットを作成します:

```powershell
# C:\vcpkg\triplets\x64-windows-static-release.cmake を作成
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### 1. vcpkgのインストール

```powershell
# vcpkgをクローン
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg

# vcpkgをブートストラップ
cd C:\vcpkg
.\bootstrap-vcpkg.bat

# 環境変数を設定（システム環境変数に追加することを推奨）
$env:VCPKG_ROOT = "C:\vcpkg"
[System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
```

### 2. 依存関係のインストール

自動インストールスクリプトを使用（推奨）:

```powershell
cd path\to\DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

または手動でインストール：

```powershell
cd C:\vcpkg

# x64-windows-static-release tripletでインストール（リリース専用静的リンク）
.\vcpkg install aom:x64-windows-static-release
.\vcpkg install libavif[aom]:x64-windows-static-release
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

インストールされるライブラリ：

- **libaom**: AV1エンコーダー（AVIF形式用） - **必須**
- **libavif**: AVIF画像フォーマット
- **libjxl**: JPEG XL画像フォーマット
- **libwebp**: WebP画像フォーマット
- **openjpeg**: JPEG 2000画像フォーマット
- **libjpeg-turbo**: JPEG画像処理（jpegli用）
- **lcms**: Little CMS カラーマネジメント

> **重要**: `libaom`はAVIFエンコーディングに必須の依存ライブラリです。vcpkg経由でインストールすることで、libaom-sysの内部ビルド（nasmエラーが発生しやすい）を回避できます。

### 3. ビルド

```powershell
# 環境変数を設定（必須）
$env:VCPKG_ROOT = "C:\vcpkg"
$env:VCPKGRS_TRIPLET = "x64-windows-static"

# ビルド
cd app\src-tauri
cargo build --release
```

## トラブルシューティング

### libaom-sys / libavif-sys がvcpkgライブラリを認識しない

**問題**: `error: failed to run custom build command for 'libaom-sys'`

vcpkgでlibaomをインストールしても、`libaom-sys`がソースからビルドしようとする場合があります。

**解決策**:

1. ビルドキャッシュをクリア

```powershell
cd app\src-tauri
cargo clean -p libaom-sys -p libavif-sys
```

2. vcpkgでライブラリがインストールされているか確認

```powershell
vcpkg list | findstr aom
vcpkg list | findstr avif

# 出力例:
# aom:x64-windows-static      3.11.0          AV1 Codec Library
# libavif:x64-windows-static  1.1.1           libavif - Library for encoding...
```

3. 環境変数を確認

```powershell
echo $env:VCPKG_ROOT
# 出力例: C:\vcpkg
```

4. クリーンビルド

```powershell
cargo clean
cargo build --release
```

### libaomのコンパイルに失敗する

**問題**: `libavif-sys`がlibaomをソースからビルドしようとして失敗する

**解決策**:

1. vcpkgでlibaomを事前にインストール（リリーストリプレット使用）
2. `VCPKG_ROOT`環境変数が正しく設定されているか確認
3. `build.rs`がvcpkg経由でlibaomを検出するか確認

```powershell
# 確認コマンド
vcpkg list | findstr aom
vcpkg list | findstr avif
```

### リンクエラー: `__imp__CrtDbgReport`未解決

**問題**: `error LNK2001: unresolved external symbol __imp__CrtDbgReport`

**原因**: vcpkgのデフォルトトリプレット(`x64-windows-static`)はデバッグシンボルを含み、Rustのリリースビルドとリンクできません。

**解決策**: **`x64-windows-static-release`トリプレット**を使用

1. カスタムトリプレットファイルを作成:

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

2. すべてのライブラリを再インストール:

```powershell
# 既存のライブラリを削除
vcpkg remove aom libavif libjxl libwebp openjpeg libjpeg-turbo lcms --triplet x64-windows-static --recurse

# リリーストリプレットで再インストール
vcpkg install aom libavif[aom] libjxl libwebp openjpeg libjpeg-turbo lcms --triplet x64-windows-static-release
```

3. `.cargo/config.toml`でトリプレットを指定:

```toml
[env]
VCPKGRS_TRIPLET = "x64-windows-static-release"
```

### その他のリンクエラー

**問題**: `LINK : fatal error LNK1181: cannot open input file 'aom.lib'`

**解決策**:

1. tripletが正しいか確認（`x64-windows-static-release`）
2. vcpkgのライブラリパスが正しいか確認

```powershell
# 環境変数を確認
echo $env:VCPKG_ROOT
echo $env:VCPKGRS_TRIPLET

# vcpkgのライブラリを確認
dir "$env:VCPKG_ROOT\installed\x64-windows-static-release\lib"
```

### 動的リンクと静的リンク

vcpkgでは`triplet`によってリンク方法を制御：

- **`x64-windows-static-release`**: 静的リンク（リリース専用、推奨）
- `x64-windows-static`: 静的リンク（デバッグシンボル含む、非推奨）
- `x64-windows`: 動的リンク
- `x64-windows-static-md`: 静的リンク + 動的CRT

**推奨設定**: `x64-windows-static-release`

理由：

- 配布が容易（DLLを同梱する必要がない）
- バージョン衝突を回避
- ポータブルなバイナリ
- **デバッグシンボルを含まないためリンクエラーを回避**

## build.rsの動作

`app/src-tauri/build.rs`は以下の順序で依存関係を解決：

1. **vcpkg経由での検出**（推奨）
   - `VCPKG_ROOT`が設定されている場合
   - libaom、libavif、libjxl、libwebp、openjpeg、libjpeg-turbo、lcms2を検出
   - 静的リンクライブラリを使用

2. **pkg-config経由での検出**（フォールバック）
   - vcpkgで見つからない場合
   - システムにインストールされたライブラリを検出

3. **libavif-sysのビルド**（最後の手段）
   - 上記のいずれも失敗した場合
   - ソースからlibaomをビルド（失敗しやすい）

## 参考リンク

- [vcpkg](https://github.com/Microsoft/vcpkg)
- [vcpkg-rs (Cargo integration)](https://github.com/mcgoo/vcpkg-rs)
- [libaom](https://aomedia.googlesource.com/aom/)
- [libavif](https://github.com/AOMediaCodec/libavif)
- [libjxl](https://github.com/libjxl/libjxl)
