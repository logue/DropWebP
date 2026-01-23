# Chocolatey Package Configuration

このディレクトリには、Chocolatey（Windows用パッケージマネージャー）向けのパッケージ定義が含まれています。

## ファイル構成

```
.choco/
├── drop-compress-image.nuspec.template  # パッケージメタデータ（テンプレート）
├── drop-compress-image.nuspec          # 生成されたファイル（.gitignore対象）
└── tools/
    └── chocolateyinstall.ps1           # インストールスクリプト
```

**注意**: `drop-compress-image.nuspec`はビルド時に自動生成されます。編集する場合は`.template`ファイルを編集してください。

## バージョン管理

**重要**: `drop-compress-image.nuspec.template`内のバージョンは、プレースホルダー `{{VERSION}}` を使用しています。

```xml
<version>{{VERSION}}</version>
```

実際のバージョンは、ビルド時に**ルートディレクトリの`.env`ファイル**から自動的に読み取られます：

```dotenv
# .env
VERSION=3.2.1
```

ビルドスクリプトは、テンプレートから`drop-compress-image.nuspec`を生成し、プレースホルダーを実際のバージョンに置換します。

## パッケージのビルド

```powershell
# .envからバージョンを自動読み取り
pnpm run package:chocolatey

# または、バージョンを明示的に指定
.\scripts\build-chocolatey.ps1 -Version 3.2.1
```

ビルドスクリプト (`scripts/build-chocolatey.ps1`) は以下の処理を行います：

1. `.env`ファイルからバージョンを読み取り
2. MSIファイル（`app/src-tauri/target/release/bundle/msi/*.msi`）を検索
3. MSIファイルのSHA256チェックサムを計算
4. `drop-compress-image.nuspec.template`から`drop-compress-image.nuspec`を生成
5. `{{VERSION}}`プレースホルダーを実際のバージョンに置換
6. `tools/chocolateyinstall.ps1`のチェックサムとバージョンを更新
7. `.nupkg`パッケージファイルを生成

## パッケージのテスト

### 前提条件

Chocolateyのインストールテストは**管理者権限のPowerShell**で実行する必要があります。

### テスト方法

#### 方法1: GitHub Releasesからインストール（推奨）

該当バージョンのMSIファイルがGitHub Releasesに公開されている場合：

```powershell
# 管理者権限のPowerShellで実行
choco install drop-compress-image -source .\.choco -y

# アンインストール
choco uninstall drop-compress-image -y
```

#### 方法2: ローカルMSIファイルで直接テスト

GitHubにリリースする前にローカルでテストする場合：

```powershell
# 管理者権限でMSIを直接インストール
$msiPath = "app\src-tauri\target\release\bundle\msi\drop-compress-image_3.2.1_x64_en-US.msi"
Start-Process msiexec.exe -ArgumentList "/i `"$msiPath`" /qn /l*v install.log" -Wait

# アンインストール
Start-Process msiexec.exe -ArgumentList "/x `"$msiPath`" /qn" -Wait
```

### トラブルシューティング

- **「not running from an elevated command shell」エラー**
  → 管理者権限でPowerShellを起動してください（右クリック → '管理者として実行'）

- **「404 Not Found」または「Remote file」エラー**
  → MSIファイルがまだGitHubにリリースされていません。方法2を使用してください

- **「The remote file either doesn't exist」エラー**
  → chocolateyinstall.ps1がGitHubからダウンロードを試みています。先にGitHub Releasesにアップロードするか、方法2でテストしてください

## パッケージの公開

```powershell
# Chocolatey Community Repositoryに公開
choco push .\.choco\drop-compress-image.3.2.1.nupkg --source https://push.chocolatey.org/
```

## 注意事項

- **テンプレートファイルを編集**: `.nuspec.template`ファイルのバージョンは常に`{{VERSION}}`のままにしてください
- **生成ファイルは編集不要**: `drop-compress-image.nuspec`（テンプレートなし）はビルド時に自動生成されるため、直接編集しないでください
- バージョン変更時は、ルートの`.env`ファイルのみを更新してください
- ビルド前に、`app/src-tauri/target/release/bundle/msi`内にMSIファイルが存在することを確認してください
