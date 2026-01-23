# Chocolatey Package Configuration

このディレクトリには、Chocolatey（Windows用パッケージマネージャー）向けのパッケージ定義が含まれています。

## ファイル構成

```
.choco/
├── drop-compress-image.nuspec  # パッケージメタデータ（テンプレート）
└── tools/
    └── chocolateyinstall.ps1   # インストールスクリプト
```

## バージョン管理

**重要**: `drop-compress-image.nuspec`内のバージョンは、プレースホルダー `{{VERSION}}` を使用しています。

```xml
<version>{{VERSION}}</version>
```

実際のバージョンは、ビルド時に**ルートディレクトリの`.env`ファイル**から自動的に読み取られます：

```dotenv
# .env
VERSION=3.2.1
```

## パッケージのビルド

```powershell
# .envからバージョンを自動読み取り
pnpm run package:chocolatey

# または、バージョンを明示的に指定
.\scripts\build-chocolatey.ps1 -Version 3.2.1
```

ビルドスクリプト (`scripts/build-chocolatey.ps1`) は以下の処理を行います：

1. `.env`ファイルからバージョンを読み取り
2. MSIファイルのSHA256チェックサムを計算
3. `{{VERSION}}`プレースホルダーを実際のバージョンに置換
4. `tools/chocolateyinstall.ps1`のチェックサムとバージョンを更新
5. `.nupkg`パッケージファイルを生成

## パッケージのテスト

```powershell
# ローカルでインストールテスト
choco install drop-compress-image -source .\.choco
```

## パッケージの公開

```powershell
# Chocolatey Community Repositoryに公開
choco push .\.choco\drop-compress-image.3.2.1.nupkg --source https://push.chocolatey.org/
```

## 注意事項

- **手動でバージョンを編集しないでください**: `.nuspec`ファイルのバージョンは常に`{{VERSION}}`のままにしてください
- バージョン変更時は、ルートの`.env`ファイルのみを更新してください
- ビルドスクリプトが自動的にすべてのファイルを更新します
