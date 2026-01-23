# Homebrew Formula Configuration

このディレクトリには、Homebrew（macOS用パッケージマネージャー）向けのFormulaが含まれています。

## ファイル構成

```
.homebrew/
└── drop-compress-image.rb  # Homebrew Formula（テンプレート）
```

## バージョン管理

**重要**: `drop-compress-image.rb`内のバージョンとチェックサムは、プレースホルダーを使用しています。

```ruby
version "{{VERSION}}"
sha256 "{{SHA256_UNIVERSAL}}"  # Universal DMG（ARM64 + Intel）のチェックサム
```

実際の値は、ビルド時に**ルートディレクトリの`.env`ファイル**から自動的に読み取られます：

```dotenv
# .env
VERSION=3.2.1
```

チェックサムは、Universalビルド後のDMGファイルから自動計算されます。

## Formulaの生成

```bash
# macOS / Linux
pnpm run package:homebrew

# または、バージョンを明示的に指定
bash scripts/build-homebrew.sh 3.2.1
```

ビルドスクリプト (`scripts/build-homebrew.sh`) は以下の処理を行います：

1. `.env`ファイルからバージョンを読み取り
2. Universal DMGファイルのSHA256チェックサムを計算
3. テンプレート内のプレースホルダーを実際の値に置換
4. 更新されたFormulaファイルを生成

## Formulaのテスト

```bash
# ローカルでインストールテスト
brew install --formula .homebrew/drop-compress-image.rb
```

## Formulaの公開

1. **Tap リポジトリの作成** (初回のみ)

   ```bash
   # GitHubでhomebrew-tapリポジトリを作成
   # 例: https://github.com/logue/homebrew-tap
   ```

2. **Formulaをpush**

   ```bash
   cp .homebrew/drop-compress-image.rb /path/to/homebrew-tap/Formula/
   cd /path/to/homebrew-tap
   git add Formula/drop-compress-image.rb
   git commit -m "Update drop-compress-image to v3.2.1"
   git push
   ```

3. **ユーザーがインストール**
   ```bash
   brew tap logue/tap
   brew install drop-compress-image
   ```

## 注意事項

- **手動でバージョンやチェックサムを編集しないでください**: Formulaファイル内の値は常にプレースホルダーのままにしてください
- バージョン変更時は、ルートの`.env`ファイルのみを更新してください
- Universal DMGファイル(`pnpm run build:tauri:mac`)をビルド後、`build-homebrew.sh`を実行してFormulaを生成してください
- チェックサムは、ビルドされたUniversal DMGファイルから自動的に計算されます
