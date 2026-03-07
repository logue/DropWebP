# Chocolatey Package Configuration

このディレクトリには、Chocolatey（Windows用パッケージマネージャー）向けのパッケージ定義が含まれています。

## ファイル構成

```
.choco/
├── app.nuspec.template                      # パッケージメタデータ（テンプレート）
├── {APP_NAME_KEBAB}.nuspec                  # 生成されたファイル（.gitignore対象）
└── tools/
    ├── chocolateyinstall.ps1.template       # インストールスクリプト（テンプレート）
    ├── chocolateyuninstall.ps1.template     # アンインストールスクリプト（テンプレート）
    ├── chocolateyinstall.ps1                # 生成されたファイル（.gitignore対象）
    └── chocolateyuninstall.ps1              # 生成されたファイル（.gitignore対象）
```

**注意**: `{APP_NAME_KEBAB}.nuspec`および`tools/*.ps1`ファイルはビルド時に自動生成されます。編集する場合は各`.template`ファイルを編集してください。

## バージョン管理

**重要**: `app.nuspec.template`内の値は、プレースホルダーを使用しています。

```xml
<id>{{APP_NAME_KEBAB}}</id>
<version>{{VERSION}}</version>
<title>{{APP_NAME}}</title>
<description>{{APP_DESCRIPTION}}</description>
```

実際の値は、ビルド時に**ルートディレクトリの`.env`ファイル**から自動的に読み取られます：

```dotenv
# .env
VERSION=1.0.0
APP_NAME=Your App Name
APP_NAME_KEBAB=your-app-name
APP_DESCRIPTION=Your app description
```

ビルドスクリプトは、テンプレートから`{APP_NAME_KEBAB}.nuspec`を生成し、プレースホルダーを実際の値に置換します。

## パッケージのビルド

```powershell
# .envからバージョンを自動読み取り
pnpm run package:chocolatey

# または、バージョンを明示的に指定
.\scripts\build-chocolatey.ps1 -Version 1.0.0
```

ビルドスクリプト (`scripts/build-chocolatey.ps1`) は以下の処理を行います：

1. `.env`ファイルからバージョンとアプリケーション情報を読み取り
2. MSIファイル（`backend/target/release/bundle/msi/*.msi`）を検索
3. MSIファイルのSHA256チェックサムを計算
4. `app.nuspec.template`から`{APP_NAME_KEBAB}.nuspec`を生成
5. 全てのプレースホルダーを実際の値に置換
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
choco install {APP_NAME_KEBAB} -source .\.choco -y

# アンインストール
choco uninstall {APP_NAME_KEBAB} -y
```

#### 方法2: ローカルMSIファイルで直接テスト

GitHubにリリースする前にローカルでテストする場合：

```powershell
# 管理者権限でMSIを直接インストール
$msiPath = "backend\target\release\bundle\msi\{APP_NAME_KEBAB}_{VERSION}_x64_en-US.msi"
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
choco push .\.choco\{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

## 注意事項

- **テンプレートファイルを編集**: `.nuspec.template`ファイルのバージョンは常に`{{VERSION}}`のままにしてください
- **生成ファイルは編集不要**: `{APP_NAME_KEBAB}.nuspec`（テンプレートなし）はビルド時に自動生成されるため、直接編集しないでください
- バージョン変更時は、ルートの`.env`ファイルのみを更新してください
- ビルド前に、`backend/target/release/bundle/msi`内にMSIファイルが存在することを確認してください
