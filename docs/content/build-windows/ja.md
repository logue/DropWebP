# Windows ビルド環境セットアップ

このガイドでは、WindowsでDropWebPをビルドするための開発環境のセットアップ手順を説明します。

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

## 8. vcpkgのセットアップ（公式手順）

1. vcpkgリポジトリをクローンします：

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   ```

2. ブートストラップスクリプトを実行します：

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. vcpkgをVisual Studioに統合します（この手順でVCPKG_ROOTなどの環境変数が設定されます）：

   ```powershell
   .\vcpkg integrate install
   ```

> **警告:** 必要に応じて、VCPKG_ROOT環境変数をvcpkgのインストールディレクトリ（例：C:\vcpkg）に設定し、vcpkgをPATHに追加することもできます。

## 9. 必要なライブラリのインストール

1. 画像処理に必要なライブラリをインストールします：

   ```powershell
   .\vcpkg install libavif libjxl libheif --triplet x64-windows-static-md
   ```

2. インストール後、ライブラリが正しくインストールされたことを確認できます：

   ```powershell
   .\vcpkg list
   ```

> **注意:** x64-windows-static-mdトリプレットは、RustのデフォルトMSVCランタイムとの互換性を保証します。

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
