# パッケージの公開

このガイドでは、Tauri Vue3 AppをChocolateyとHomebrewに公開する方法を説明します。

## 前提条件

### Chocolatey（Windows）

- Chocolateyをインストール: <https://chocolatey.org/install>
- <https://community.chocolatey.org/>でアカウントを作成
- アカウントページからAPIキーを取得

### Homebrew（macOS）

- Homebrew tap用のGitHubリポジトリを作成（例：`homebrew-tap`）
- `repo`スコープを持つGitHub Personal Access Tokenを生成

## パッケージのビルド

### 1. Tauriアプリケーションのビルド

```bash
pnpm build:tauri
```

これにより、以下の場所にプラットフォーム固有のインストーラーが作成されます：

- Windows: `backend/target/release/bundle/msi/`
- macOS: `backend/target/release/bundle/dmg/`
- Linux: `backend/target/release/bundle/deb/` または `appimage/`

### 2. Chocolateyパッケージの生成（Windows）

```powershell
pnpm package:chocolatey
```

または手動で：

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

これにより以下が実行されます：

- MSIファイルのSHA256チェックサムを計算
- 正しいチェックサムで`chocolateyinstall.ps1`を更新
- `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg`を作成

### 3. Homebrewフォーミュラの生成（macOS）

```bash
pnpm package:homebrew
```

または手動で：

```bash
./scripts/build-homebrew.sh {VERSION}
```

これにより以下が実行されます：

- ARM64とx64の両方のDMGファイルのSHA256チェックサムを計算
- チェックサムで`.homebrew/{APP_NAME_KEBAB}.rb`を更新

## 公開

### Chocolatey

1. **まずローカルでテスト：**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **Chocolatey Community Repositoryにプッシュ：**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **モデレーションキューを監視：**

- <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>にアクセス
- パッケージはモデレーターによってレビューされます（通常48時間以内）

### Homebrew

1. **tapリポジトリを作成**（初回のみ）：

```bash
# GitHubで新しいリポジトリを作成：homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **フォーミュラをコピー：**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **ユーザーは以下でインストール可能：**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### GitHub Actionsによる自動公開

リポジトリには、プロセス全体を自動化するGitHub Actionsワークフロー（`.github/workflows/release.yml`）が含まれています。

#### シークレットの設定

GitHubリポジトリ設定に以下のシークレットを追加します：

1. **CHOCOLATEY_API_KEY**: ChocolateyのAPIキー
2. **HOMEBREW_TAP_TOKEN**: tapリポジトリ用のGitHub Personal Access Token

#### リリースのトリガー

1. **`tauri.conf.json`と`package.json`のバージョンを更新**

2. **gitタグを作成してプッシュ：**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **ワークフローが自動的に以下を実行：**
   - Windows、macOS（ARM64とx64）、Linux向けにビルド
   - GitHub Releaseを作成
   - Chocolateyパッケージを生成してコミュニティリポジトリにプッシュ
   - Homebrewフォーミュラを生成してtapリポジトリにPRを作成

## バージョン管理

常に以下のバージョンを同期させてください：

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## トラブルシューティング

### Chocolatey

**問題**：パッケージがモデレーション失敗

- <https://community.chocolatey.org/>でモデレーションコメントを確認
- 一般的な問題：不正なチェックサム、依存関係の欠落、インストールの問題
- 修正してバージョンを上げて再送信

**問題**：チェックサムの不一致

- チェックサム計算後にMSIファイルが変更されていないことを確認
- `pnpm package:chocolatey`でパッケージを再ビルド

### Homebrew

**問題**：インストール時のSHA256不一致

- DMGファイルがGitHub releasesにアップロードされていることを確認
- フォーミュラ内のURLが実際のリリースアセットと一致することを確認
- チェックサムを再計算：`shasum -a 256 *.dmg`

**問題**：macOSでアプリが開かない

- アプリが適切に署名され公証されていることを確認
- Gatekeeperがブロックしているか確認：`xattr -d com.apple.quarantine /Applications/Tauri\ Vue3\ App.app`

## リソース

- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Tauri Bundling](https://tauri.app/v1/guides/building/)
