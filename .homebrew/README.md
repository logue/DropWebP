# Homebrew Formula Configuration

このディレクトリには、Homebrew（macOS用パッケージマネージャー）向けのFormulaが含まれています。

## ファイル構成

```
.homebrew/
├── app.rb.template              # Homebrew Formulaテンプレート
└── {APP_NAME_KEBAB}.rb          # 生成されたFormula（.gitignore対象）
```

**注意**: `{APP_NAME_KEBAB}.rb`はビルド時に自動生成されます。編集する場合は`app.rb.template`ファイルを編集してください。

## バージョン管理

**重要**: `app.rb.template`内の値は、プレースホルダーを使用しています。

```ruby
class {{CLASS_NAME}} < Formula
  desc "{{HOMEBREW_DESC}}"
  homepage "{{HOMEPAGE_URL}}"
  version "{{VERSION}}"
  url "{{PROJECT_URL}}/releases/download/v#{version}/{{APP_NAME}}_#{version}_universal.dmg"
  sha256 "{{SHA256_UNIVERSAL}}"
```

実際の値は、ビルド時に**ルートディレクトリの`.env`ファイル**から自動的に読み取られます：

```dotenv
# .env
VERSION=1.0.0
APP_NAME=Your App Name
APP_NAME_KEBAB=your-app-name
HOMEBREW_DESC=Your app description
PROJECT_URL=https://github.com/username/repository
```

チェックサムは、Universalビルド後のDMGファイルから自動計算されます。

## Formulaの生成

```bash
# macOS / Linux
pnpm run package:homebrew

# または、バージョンを明示的に指定
bash scripts/build-homebrew.sh 1.0.0
```

ビルドスクリプト (`scripts/build-homebrew.sh`) は以下の処理を行います：

1. `.env`ファイルからバージョンとアプリケーション情報を読み取り
2. Universal DMGファイルのSHA256チェックサムを計算
3. `app.rb.template`から`{APP_NAME_KEBAB}.rb`を生成
4. 全てのプレースホルダーを実際の値に置換

## Formulaのテスト

```bash
# ローカルでインストールテスト
brew install --formula .homebrew/{APP_NAME_KEBAB}.rb
```

## Formulaの公開

1. **Tap リポジトリの作成** (初回のみ)

   ```bash
   # GitHubでhomebrew-tapリポジトリを作成
   # 例: https://github.com/{GITHUB_USER}/homebrew-tap
   ```

2. **Formulaをpush**

   ```bash
   cp .homebrew/{APP_NAME_KEBAB}.rb /path/to/homebrew-tap/Formula/
   cd /path/to/homebrew-tap
   git add Formula/{APP_NAME_KEBAB}.rb
   git commit -m "Update {APP_NAME_KEBAB} to v{VERSION}"
   git push
   ```

3. **ユーザーがインストール**
   ```bash
   brew tap {GITHUB_USER}/tap
   brew install {APP_NAME_KEBAB}
   ```

## 注意事項

- **手動でバージョンやチェックサムを編集しないでください**: Formulaファイル内の値は常にプレースホルダーのままにしてください
- バージョン変更時は、ルートの`.env`ファイルのみを更新してください
- Universal DMGファイル(`pnpm run build:tauri:mac`)をビルド後、`build-homebrew.sh`を実行してFormulaを生成してください
- チェックサムは、ビルドされたUniversal DMGファイルから自動的に計算されます
