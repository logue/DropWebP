# Windows環境でのDockerによるLinuxビルド対応

## 変更概要

Windows環境からDockerを使用してLinux向けのパッケージ（.deb、.rpm）をビルドできるようになりました。

## 追加されたファイル

### 1. `scripts/build-linux-docker.ps1`

Windows用のPowerShellビルドスクリプト。以下の機能を提供：

- Docker Desktop起動チェック
- x86_64およびARM64ターゲットのサポート
- ビルド設定のカスタマイズ（CPU、メモリ、並列度）
- キャッシュボリュームの管理
- AppImageオプションのサポート

### 2. `DOCKER_BUILD_WINDOWS.md`

Windows環境でのDockerビルドに関する詳細なドキュメント：

- 前提条件とシステム要件
- ビルド方法と手順
- トラブルシューティングガイド
- パフォーマンス最適化のヒント

### 3. `DOCKER_BUILD.md`

すべてのプラットフォームに対応した総合的なDockerビルドドキュメント：

- 対応プラットフォーム一覧
- ビルドプロセスの説明
- キャッシュ管理
- よくある質問

## 変更されたファイル

### 1. `app/package.json`

Linuxビルドスクリプトをクロスプラットフォーム対応に変更：

```json
"build:tauri:linux-docker-x64": "node -e \"require('child_process').execSync(process.platform === 'win32' ? 'pwsh ..\\\\scripts\\\\build-linux-docker.ps1 -Target x64' : 'bash ../scripts/build-linux-docker.sh x64', {stdio: 'inherit'})\"",
"build:tauri:linux-docker-arm64": "node -e \"require('child_process').execSync(process.platform === 'win32' ? 'pwsh ..\\\\scripts\\\\build-linux-docker.ps1 -Target arm64' : 'bash ../scripts/build-linux-docker.sh arm64', {stdio: 'inherit'})\""
```

これにより、`pnpm run build:tauri:linux-docker-x64`がWindows、macOS、Linuxすべてで動作します。

### 2. `package.json`（ルート）

Linuxビルド用のショートカットコマンドを追加：

```json
"build:tauri:linux-x64": "pnpm --filter app build:tauri:linux-docker-x64",
"build:tauri:linux-arm64": "pnpm --filter app build:tauri:linux-docker-arm64"
```

### 3. `.env`

Dockerビルド設定用の環境変数コメントを追加：

```bash
# Docker Build Settings (optional)
# BUILD_CPUS=4
# BUILD_MEMORY=8g
# CARGO_BUILD_JOBS=4
# MAKEFLAGS=-j4
# INCLUDE_APPIMAGE=false
```

### 4. `ReadMe.md`

DockerによるLinuxビルドの情報を追加：

- Windows用のビルドコマンド
- macOS/Linux用のビルドコマンド
- 必要な要件（Docker Desktop）

## 使用方法

### Windows環境でのビルド

```powershell
# プロジェクトルートから
pnpm run build:tauri:linux-x64    # x86_64 Linux
pnpm run build:tauri:linux-arm64  # ARM64 Linux

# または直接スクリプトを実行
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64 -IncludeAppImage
```

### macOS/Linux環境でのビルド（変更なし）

```bash
# 既存のBashスクリプトを継続使用
bash scripts/build-linux-docker.sh x64
bash scripts/build-linux-docker.sh arm64
```

## 前提条件

### Windows

- Windows 10/11 (64-bit)
- Docker Desktop for Windows
- WSL 2（推奨）
- 8GB以上のRAM（16GB推奨）
- PowerShell 5.1以上

### macOS/Linux

- Docker Desktop または Docker Engine
- 8GB以上のRAM（16GB推奨）
- Bash

## ビルド設定のカスタマイズ

`.env`ファイルで設定をカスタマイズ可能：

```bash
BUILD_CPUS=4              # 使用するCPUコア数
BUILD_MEMORY=8g           # メモリ制限
CARGO_BUILD_JOBS=4        # Cargoの並列ジョブ数
MAKEFLAGS=-j4             # Makeの並列度
INCLUDE_APPIMAGE=false    # AppImageを含めるか
```

## 生成される成果物

ビルド完了後、以下の場所に成果物が生成されます：

```
app/src-tauri/target/<target>/release/bundle/
├── deb/
│   └── drop-compress-image_<version>_<arch>.deb
└── rpm/
    └── drop-compress-image-<version>-1.<arch>.rpm
```

例：

- `app/src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/`
- `app/src-tauri/target/aarch64-unknown-linux-gnu/release/bundle/`

## トラブルシューティング

### Docker Desktopが起動していない

スクリプトは自動的にDocker Desktopの起動状態をチェックし、エラーメッセージを表示します：

```
❌ エラー: Docker Desktop が起動していません。
Docker Desktop を起動してから、再度実行してください。
```

### メモリ不足

Docker Desktopの設定でメモリを増やす、または`.env`でメモリ制限を調整してください。

詳細は `DOCKER_BUILD_WINDOWS.md` を参照してください。

## 技術的な詳細

### PowerShellスクリプトの特徴

1. **Docker起動チェック**: ビルド前にDockerの動作を確認
2. **クロスプラットフォーム**: Windowsパスを適切に処理
3. **キャッシュ管理**: Dockerボリュームを使用した高速再ビルド
4. **エラーハンドリング**: わかりやすいエラーメッセージ
5. **進行状況表示**: 色付きの進行状況メッセージ

### package.jsonのクロスプラットフォーム対応

Node.jsの`child_process.execSync`と`process.platform`を使用して、OSに応じて適切なスクリプトを実行：

- Windows: PowerShellスクリプト（`.ps1`）
- macOS/Linux: Bashスクリプト（`.sh`）

これにより、同じ`pnpm run`コマンドがすべてのプラットフォームで動作します。

## 今後の拡張可能性

- GitHub Actionsでの自動ビルド対応
- Windows Dockerfileの追加（現在はLinuxのみ）
- ビルドキャッシュの最適化
- マルチステージビルドによる高速化

## 参考ドキュメント

- [DOCKER_BUILD_WINDOWS.md](./DOCKER_BUILD_WINDOWS.md) - Windows固有の詳細手順
- [DOCKER_BUILD.md](docker-build.md) - すべてのプラットフォーム向けガイド
- [ReadMe.md](./ReadMe.md) - プロジェクト概要
