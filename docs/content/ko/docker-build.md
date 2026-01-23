# DockerによるLinuxビルド

このプロジェクトでは、macOS、Linux、WindowsからDockerを使用してLinux向けのパッケージをビルドできます。

## 対応プラットフォーム

| ホストOS              | x86_64ビルド | ARM64ビルド |
| --------------------- | ------------ | ----------- |
| Windows               | ✅           | ✅          |
| macOS (Intel)         | ✅           | ✅          |
| macOS (Apple Silicon) | ✅           | ✅          |
| Linux                 | ✅           | ✅          |

## クイックスタート

### Windows

```powershell
# x86_64 Linux
pnpm run build:tauri:linux-x64

# ARM64 Linux
pnpm run build:tauri:linux-arm64
```

詳細は [DOCKER_BUILD_WINDOWS.md](./DOCKER_BUILD_WINDOWS.md) を参照してください。

### macOS / Linux

```bash
# x86_64 Linux
bash scripts/build-linux-docker.sh x64

# ARM64 Linux
bash scripts/build-linux-docker.sh arm64
```

## 前提条件

### すべてのプラットフォーム共通

- Docker Desktop または Docker Engine
- pnpm (v10.2.0以上)
- 8GB以上のRAM（16GB推奨）
- 20GB以上のディスク空き容量

### プラットフォーム別

#### Windows

- Windows 10/11 (64-bit)
- WSL 2（推奨）
- PowerShell 5.1以上

#### macOS

- macOS 10.15以上
- Bash
- Docker Desktop for Mac

#### Linux

- 64-bit Linux distribution
- Docker Engine 20.10以上
- Bash

## ビルドプロセス

ビルドは以下の手順で実行されます：

1. **Dockerイメージの構築**: 必要な依存関係を含むLinux環境を構築
2. **依存関係のインストール**: pnpmでNode.js依存関係をインストール
3. **Rustビルド**: Cargo経由でRustコードをコンパイル
4. **フロントエンドビルド**: Vite経由でVue.jsアプリをビルド
5. **Tauriバンドル**: .deb、.rpm、AppImageパッケージを生成
6. **成果物のコピー**: ホストシステムに成果物をコピー

## 生成されるパッケージ

### デフォルト（AppImage無効）

- `.deb` パッケージ（Debian/Ubuntu用）
- `.rpm` パッケージ（Fedora/RHEL/openSUSE用）

### AppImage有効時

上記に加えて：

- `.AppImage` パッケージ（ディストリビューション非依存）

## ビルドオプション

### 環境変数（.envファイル）

```bash
# CPUコア数（並列ビルド）
BUILD_CPUS=4

# メモリ制限
BUILD_MEMORY=8g

# Cargo並列ジョブ数
CARGO_BUILD_JOBS=4

# Make並列度
MAKEFLAGS=-j4

# AppImageを含める（デフォルト: false）
INCLUDE_APPIMAGE=false
```

### コマンドラインオプション

#### Bashスクリプト (macOS/Linux)

```bash
# ターゲットアーキテクチャ
bash scripts/build-linux-docker.sh [x64|arm64]

# AppImageを有効化
INCLUDE_APPIMAGE=true bash scripts/build-linux-docker.sh x64
```

#### PowerShellスクリプト (Windows)

```powershell
# ターゲットアーキテクチャ
pwsh scripts\build-linux-docker.ps1 -Target [x64|arm64]

# AppImageを有効化
pwsh scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

## キャッシュ管理

Dockerボリュームを使用してビルドキャッシュを永続化し、再ビルドを高速化します。

### キャッシュボリューム

**重要**: ビルド中間ファイルはDockerボリューム内に保存され、ホスト環境とは完全に分離されています。これにより、異なるOS間でのバイナリ混在を防ぎます。

#### x86_64用ボリューム

- `dropwebp-cargo-cache-linux-amd64`: Cargoレジストリキャッシュ
- `dropwebp-pnpm-cache-linux-amd64`: pnpmストアキャッシュ
- `dropwebp-target-cache-linux-amd64`: Rustビルド中間ファイル
- `dropwebp-node-modules-linux-amd64`: Node.js依存関係（ホストから分離）

#### ARM64用ボリューム

- `dropwebp-cargo-cache-linux-arm64`: Cargoレジストリキャッシュ
- `dropwebp-pnpm-cache-linux-arm64`: pnpmストアキャッシュ
- `dropwebp-target-cache-linux-arm64`: Rustビルド中間ファイル
- `dropwebp-node-modules-linux-arm64`: Node.js依存関係（ホストから分離）

**ホスト環境への影響**: `node_modules`と`target`ディレクトリはDockerボリュームにマウントされるため、ホストのWindows/macOS環境を汚染しません。

### キャッシュのクリア

#### Windows

```powershell
# x86_64キャッシュをクリア
docker volume rm dropwebp-cargo-cache-linux-amd64
docker volume rm dropwebp-pnpm-cache-linux-amd64
docker volume rm dropwebp-target-cache-linux-amd64

# ARM64キャッシュをクリア
docker volume rm dropwebp-cargo-cache-linux-arm64
docker volume rm dropwebp-pnpm-cache-linux-arm64
docker volume rm dropwebp-target-cache-linux-arm64
```

#### macOS / Linux

```bash
# x86_64キャッシュをクリア
docker volume rm dropwebp-cargo-cache-linux-amd64
docker volume rm dropwebp-pnpm-cache-linux-amd64
docker volume rm dropwebp-target-cache-linux-amd64

# ARM64キャッシュをクリア
docker volume rm dropwebp-cargo-cache-linux-arm64
docker volume rm dropwebp-pnpm-cache-linux-arm64
docker volume rm dropwebp-target-cache-linux-arm64
```

## トラブルシューティング

### Dockerが見つからない

**症状**: `docker: command not found` または `Docker Desktop が起動していません`

**解決方法**:

- Docker DesktopまたはDocker Engineがインストールされているか確認
- Dockerが起動しているか確認
- パスが通っているか確認

### メモリ不足

**症状**: ビルド中にメモリエラー

**解決方法**:

1. `.env`でメモリ制限を増やす
2. Docker Desktopのリソース設定でメモリを増やす
3. 並列ビルド数を減らす（`BUILD_CPUS`を減らす）

### ビルドが遅い

**解決方法**:

- `.env`で並列度を増やす
- Docker Desktopのリソース（CPU、メモリ）を増やす
- SSDを使用する
- キャッシュボリュームを活用する

### AppImageビルドエラー

**症状**: AppImageのビルドに失敗

**解決方法**:

- Docker環境ではFUSEの制限があるため、デフォルトではAppImageは無効
- どうしても必要な場合は`-IncludeAppImage`オプションを使用
- または、ネイティブLinux環境でビルドする

### プラットフォームエラー

**症状**: `platform linux/arm64 does not match the detected host platform`

**解決方法**:

- Docker Desktopで "Use multi-platform images" が有効か確認
- Docker Buildxが有効か確認：`docker buildx version`
- 必要に応じてBuildxをインストール

## パフォーマンス最適化

### 高性能マシン向け設定

```bash
# .env
BUILD_CPUS=12
BUILD_MEMORY=16g
CARGO_BUILD_JOBS=12
MAKEFLAGS=-j12
```

### 一般的なマシン向け設定

```bash
# .env
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
```

### 低スペックマシン向け設定

```bash
# .env
BUILD_CPUS=2
BUILD_MEMORY=4g
CARGO_BUILD_JOBS=2
MAKEFLAGS=-j2
```

## CI/CDでの使用

GitHub ActionsなどのCI環境でも同じスクリプトが使用できます：

```yaml
- name: Build Linux packages
  run: |
    pnpm run build:tauri:linux-x64
```

## 参考資料

### ドキュメント

- [DOCKER_BUILD_WINDOWS.md](./DOCKER_BUILD_WINDOWS.md) - Windows固有の手順
- [Tauri Documentation](https://tauri.app/v1/guides/building/)
- [Docker Documentation](https://docs.docker.com/)

### Dockerfiles

- [Dockerfile.linux-x64](../../docker/Dockerfile.linux-x64) - x86_64 Linux用
- [Dockerfile.linux-arm64](../../docker/Dockerfile.linux-arm64) - ARM64 Linux用

### スクリプト

- [scripts/build-linux-docker.sh](./scripts/build-linux-docker.sh) - macOS/Linux用
- [scripts/build-linux-docker.ps1](./scripts/build-linux-docker.ps1) - Windows用

## よくある質問

### Q: Windowsから両方のアーキテクチャをビルドできますか？

A: はい、可能です：

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

### Q: ビルド時間はどのくらいかかりますか？

A: 環境によりますが：

- 初回ビルド: 20-40分
- 2回目以降（キャッシュあり）: 5-15分

### Q: Dockerなしでビルドできますか？

A: はい、ネイティブLinux環境でビルドできます：

```bash
cd app
pnpm install
pnpm run build:tauri:linux-x64
```

### Q: 生成されたパッケージはどこにありますか？

A: `app/src-tauri/target/<target>/release/bundle/` ディレクトリに生成されます。

### Q: WSL 2とHyper-Vの違いは？

A: WSL 2（推奨）:

- より高速
- ファイルI/Oが速い
- メモリ使用量が少ない

Hyper-V:

- レガシーモード
- WSL 2が使えない場合の代替

### Q: macOS Apple Siliconから x86_64 をビルドできますか？

A: はい、Dockerのマルチアーキテクチャサポートにより可能です（ただし、Rosetta 2経由で少し遅くなります）。
