# Windows ビルド環境セットアップ

このガイドでは、WindowsでDrop Compress Imageをビルドするための開発環境のセットアップ手順を説明します。

## ビルド方法の選択

Windowsでのビルドには3つの方法があります：

1. **DockerでLinuxビルド（推奨）**: Windows環境からLinux向けパッケージをビルド
2. **Docker環境でWindowsビルド**: クリーンな環境で依存関係の競合を回避
3. **ネイティブ環境でのビルド**: より高速だが環境構築が複雑

---

## 方法1: DockerでLinuxビルド（推奨）

WindowsからDockerを使用してLinux向けパッケージ（.deb、.rpm）をビルドできます。

### 前提条件

- Windows 10/11 (64-bit)
- Docker Desktop for Windows
- WSL 2（推奨）
- PowerShell 5.1以上
- 8GB以上のRAM（16GB推奨）

### 手順

1. **Docker Desktop のインストール**

   [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/)をダウンロードしてインストールします。

2. **WSL 2 の有効化**

   Docker Desktop の設定で「Use WSL 2 based engine」を有効にします（推奨）。

3. **プロジェクトのクローン**

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

4. **Linuxパッケージのビルド**

   ```powershell
   # x86_64 Linux用
   pnpm run build:tauri:linux-x64

   # ARM64 Linux用
   pnpm run build:tauri:linux-arm64

   # または直接スクリプトを実行
   pwsh .\scripts\build-linux-docker.ps1 -Target x64
   ```

5. **ビルド成果物の確認**

   ビルドが成功すると、以下の場所にパッケージが生成されます：
   - `app/src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/`
   - `app/src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/rpm/`

### Linux向けビルドの利点

- ✅ WindowsからLinux向けパッケージを直接ビルド可能
- ✅ CI/CD環境との一貫性
- ✅ クロスプラットフォーム開発が容易
- ✅ ホスト環境を汚さない

> **詳細**: [Linux向けビルド（Docker使用）](./build-linux-docker.md)を参照してください。

---

## 方法2: Docker環境でWindowsビルド

### 前提条件

- Windows 10/11 Pro、Enterprise、Education（Hyper-V対応）
- Docker Desktop for Windows

### 手順

1. **Docker Desktop のインストール**

   [Docker Desktop](https://www.docker.com/products/docker-desktop)をダウンロードしてインストールします。

2. **Windowsコンテナモードへの切り替え**

   Docker Desktopのタスクトレイアイコンを右クリックし、「Switch to Windows containers...」を選択します。

3. **プロジェクトのクローン**

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

4. **Dockerイメージのビルド**（初回のみ、30-60分程度かかります）

   ```powershell
   docker build -f Dockerfile.windows-x64 -t dropwebp-windows-builder .
   ```

5. **アプリケーションのビルド**

   ```powershell
   docker run --rm -v ${PWD}:C:\workspace dropwebp-windows-builder
   ```

6. **ビルド成果物の確認**

   ビルドが成功すると、`app/src-tauri/target/release/bundle/`ディレクトリに実行ファイルとインストーラーが生成されます。

### Docker環境の利点

- ✅ ホスト環境を汚さない
- ✅ 依存関係の競合を回避
- ✅ 再現可能なビルド
- ✅ クリーンな環境でのビルド
- ✅ CI/CD環境との一貫性

---

## 方法3: ネイティブ環境でのビルド

## 1. Chocolateyのインストール

1. 管理者権限でPowerShellを開き、以下のコマンドでChocolateyパッケージマネージャーをインストールします：

   ```powershell
   Set-ExecutionPolicy Bypass -Scope Process -Force;
   [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
   iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
   ```

2. インストール後、バージョンを確認しましょう：

   ```powershell
   choco -v
   ```

## 2. Gitのインストール

1. ChocolateyでGitをインストールします：

   ```powershell
   choco install git -y
   ```

2. インストール後、バージョンを確認しましょう：

   ```powershell
   git --version
   ```

## 3. プロジェクトのクローン

1. GitHubからプロジェクトをクローンし、プロジェクトディレクトリに移動します：

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

## 4. Visual Studio Community 2022のインストール

1. Visual Studio Community 2022をインストールします：

   ```powershell
   choco install visualstudio2022community -y
   ```

2. C++デスクトップ開発ワークロードをインストールします：

   ```powershell
   choco install visualstudio2022-workload-nativedesktop -y
   ```

3. Clang/LLVMビルドツールをインストールします。これは一部の画像コーデックライブラリのビルドに必要です：

   ```powershell
   choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset" -y
   ```

4. インストールが完了したら、Visual Studio Installerでインストール内容を確認できます。

> **注意:** C++デスクトップ開発ワークロードには、MSVC（Microsoftのコンパイラ）、Windows SDK、CMakeなど、Rustのネイティブ拡張ビルドに必要なツールが含まれています。

## 5. NASMとNinjaのインストール

1. NASMとNinjaをインストールします。これらは画像コーデックライブラリのビルドに必要です：

   ```powershell
   choco install nasm ninja -y
   ```

2. インストール後、バージョンを確認しましょう：

   ```powershell
   nasm -v
   ninja --version
   ```

3. NASMをシステムのPATHに追加します。これによりCargoがビルド時にNASMを見つけられるようになります：

   ```powershell
   [System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') + ';C:\Program Files\NASM', 'User')
   ```

4. 設定を反映させるため、ターミナルまたはPowerShellセッションを再起動してください。

> **注意:** NASMはアセンブラで、libavifなどの高速化されたコーデックライブラリのビルドに使用されます。Ninjaは高速なビルドシステムで、CMakeと組み合わせて使用されます。

## 6. Node.jsとpnpmのインストール

1. Node.jsとpnpmをインストールします：

   ```powershell
   choco install nodejs pnpm -y
   ```

2. インストール後、バージョンを確認しましょう：

   ```powershell
   node -v
   pnpm -v
   ```

## 7. Rustのインストール（公式推奨）

1. 公式の方法でRustをインストールします。PowerShellまたはコマンドプロンプトで以下を実行します：

   ```powershell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. インストール後、バージョンを確認しましょう：

   ```powershell
   rustc --version
   ```

> **警告:** ChocolateyでもRustをインストールできますが、MinGWツールチェーンでインストールされるため、ライブラリとの互換性問題が発生する可能性があります。

## 8. vcpkgのセットアップ

1. vcpkgリポジトリをクローンします：

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
   cd C:\vcpkg
   ```

2. ブートストラップスクリプトを実行します：

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. 環境変数を設定します（システム環境変数に追加することを推奨）：

   ```powershell
   $env:VCPKG_ROOT = "C:\vcpkg"
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```

> **重要:** VCPKG_ROOT環境変数はビルドシステムがvcpkgライブラリを見つけるために必須です。

## 9. 依存ライブラリのインストール

### リリース用トリプレットの作成

vcpkgのデフォルトトリプレットはデバッグシンボルを含むため、Rustのリリースビルドでリンクエラーが発生します。カスタムトリプレットを作成します：

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### 依存ライブラリのインストール

> **注意 (2026年2月更新):** AVIFエンコーダーとして、WindowsではRust製の`rav1e`を使用するようになりました。これにより、`libaom`および`aom`パッケージのインストールは不要になります。`rav1e`はNASMのmultipass optimization要件を回避し、Windowsでのビルドの安定性が向上します。

自動インストールスクリプトを使用（推奨）:

```powershell
cd DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

または手動でインストール：

```powershell
cd C:\vcpkg

# x64-windows-static-release tripletでインストール（リリース専用）
# 注: aomとlibavif[aom]は現在不要です（rav1e使用のため）
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

インストールされるライブラリ:

- **rav1e**: AV1エンコーダー（Rust製、AVIFエンコード用）- Cargoで自動ビルド
- **libjxl**: JPEG XL画像フォーマット
- **libwebp**: WebP画像フォーマット
- **openjpeg**: JPEG 2000画像フォーマット
- **libjpeg-turbo**: JPEG画像処理（jpegli用）
- **lcms**: Little CMS カラーマネジメント

> **macOS/Linuxユーザーへの注記:** macOSとLinuxでは`libaom`を使用することも可能です。これらのプラットフォームではNASMやCMakeの設定が安定しているためです。

インストール確認:

```powershell
.\vcpkg list | Select-String "jxl|webp|openjpeg|jpeg|lcms"
```

## 10. アプリケーションのビルド

1. appディレクトリに移動し、依存関係をインストールします：

   ```powershell
   cd app
   pnpm install
   ```

2. 開発モードでアプリケーションをビルドして実行します：

   ```powershell
   pnpm run dev:tauri
   ```

3. プロダクション用にビルドする場合：

   ```powershell
   pnpm run build:tauri
   ```

これで、Windowsでアプリケーションのビルドが成功するはずです。問題が発生した場合は、すべての依存関係が正しくインストールされ、環境変数が正しく設定されていることを確認してください。

---

## Arm64 Windows向けクロスビルド

Arm64 Windows（Windows on ARM）向けにx64 Windowsマシンからクロスビルドできます。

### 前提条件

- 上記のx64ビルド環境がセットアップ済み
- Arm64ターゲットのvcpkg依存関係

### 1. Rustツールチェインの追加

```powershell
rustup target add aarch64-pc-windows-msvc
```

### 2. Arm64用vcpkg依存関係のインストール

リリース用トリプレットの作成（まだの場合）:

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\arm64-windows-static-release.cmake
```

依存関係をインストール:

```powershell
cd C:\vcpkg

# 注: aomとlibavif[aom]は現在不要です（rav1e使用のため）
.\vcpkg install libjxl:arm64-windows-static-release
.\vcpkg install libwebp:arm64-windows-static-release
.\vcpkg install openjpeg:arm64-windows-static-release
.\vcpkg install libjpeg-turbo:arm64-windows-static-release
.\vcpkg install lcms:arm64-windows-static-release
```

### 3. Arm64向けビルド

**重要:** Arm64向けビルドの前に、必ず以下の環境変数を設定してください：

```powershell
$env:VCPKGRS_TRIPLET="arm64-windows-static-release"
$env:VCPKG_DEFAULT_TRIPLET="arm64-windows-static-release"
$env:LIB_AOM_STATIC_LIB_PATH="C:/vcpkg/installed/arm64-windows-static-release/lib"
$env:LIB_AOM_INCLUDE_PATH="C:/vcpkg/installed/arm64-windows-static-release/include"
```

その後、ビルドを実行します：

```powershell
cd path\to\DropWebP\app
pnpm run build:tauri:windows-arm64
```

または手動でビルド:

```powershell
cd app\src-tauri
cargo build --release --target aarch64-pc-windows-msvc
cd ..
pnpm tauri build --target aarch64-pc-windows-msvc
```

**トラブルシューティング:**

リンクエラーが発生する場合：

1. ビルドキャッシュを完全にクリーンアップ：

   ```powershell
   cd app\src-tauri
   cargo clean
   ```

2. 環境変数を設定してから再ビルド：
   ```powershell
   $env:VCPKGRS_TRIPLET="arm64-windows-static-release"
   $env:VCPKG_DEFAULT_TRIPLET="arm64-windows-static-release"
   cargo build --release --target aarch64-pc-windows-msvc
   ```

### 注意事項

- Arm64バイナリはArm64 Windowsデバイス（Surface Pro X等）でのみ動作します
- クロスビルドしたバイナリはx64マシンでは実行できません
- ビルド成果物は`app/src-tauri/target/aarch64-pc-windows-msvc/release/`に生成されます
