# macOS用Drop Compress Imageのビルド

このガイドでは、macOSシステムでの開発環境のセットアップとDrop Compress Imageのビルド手順を説明します。

## 前提条件

開始する前に、以下が必要です：

- macOS 10.15 (Catalina) 以降
- ソフトウェアインストール用の管理者権限
- ターミナルコマンドの基本的な知識

## ステップ 1: Xcode Command Line Toolsのインストール

まず、`clang`や`make`などの重要な開発ツールを提供するXcode Command Line Toolsをインストールします：

```bash
xcode-select --install
```

これにより、コマンドライン開発者ツールをインストールするかどうかを尋ねるダイアログが表示されます。**インストール**をクリックして、インストールが完了するまで待ちます。

### インストールの確認

ツールが正しくインストールされたことを確認します：

```bash
clang --version
```

以下のような出力が表示されます：

```text
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0
Thread model: posix
```

## ステップ 2: Homebrewのインストール

HomebrewはmacOS用のパッケージマネージャーで、開発ツールやライブラリのインストールを簡単にします。

### Homebrewのインストール

ターミナルを開いて実行します：

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### HomebrewをPATHに追加

Apple Silicon Mac（M1/M2/M3）の場合、HomebrewをPATHに追加します：

```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

Intel Macの場合、Homebrewは`/usr/local`にインストールされ、既にPATHに含まれているはずです。

### Homebrewインストールの確認

```bash
brew --version
```

## ステップ 3: Rustのインストール

Drop Compress ImageはRustで構築されているため、Rustツールチェインをインストールする必要があります。

### rustup経由でRustをインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

プロンプトが表示されたら、オプション1（デフォルトインストール）を選択します。

### シェルの設定

```bash
source ~/.cargo/env
```

### Rustインストールの確認

```bash
rustc --version
cargo --version
```

`rustc`と`cargo`の両方でバージョン情報が表示されます。

## ステップ 4: Node.jsのインストール

Drop Compress ImageのフロントエンドはVue.jsで構築されており、Node.jsが必要です。

### Homebrew経由でNode.jsをインストール

```bash
brew install node
```

### Node.jsインストールの確認

```bash
node --version
npm --version
```

## ステップ 5: pnpmのインストール

Drop Compress Imageは、パフォーマンスとディスク効率を向上させるためにpnpmをパッケージマネージャーとして使用します。

### pnpmのインストール

```bash
npm install -g pnpm
```

### pnpmインストールの確認

```bash
pnpm --version
```

## ステップ 6: 追加の依存関係のインストール

ビルドに必要な追加ツールをインストールします：

```bash
# CMakeのインストール（一部のネイティブ依存関係に必要）
brew install cmake

# pkg-configのインストール（ライブラリのリンクに必要）
brew install pkg-config
```

## ステップ 7: Drop Compress Imageのクローンとビルド

これでDrop Compress Imageをクローンしてビルドする準備が整いました。

### リポジトリのクローン

```bash
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

### フロントエンド依存関係のインストール

```bash
# すべてのワークスペース依存関係をインストール
pnpm install
```

### Tauri CLI v2のインストール

```bash
# Tauri CLI v2をグローバルにインストール
pnpm add -g @tauri-apps/cli@next
```

### アプリケーションのビルド

開発用：

```bash
# 開発モードで実行
pnpm dev:tauri
```

本番用：

```bash
# 本番用にビルド
pnpm build:tauri
```

## ステップ 8: プラットフォーム固有の考慮事項

### Apple Silicon (M1/M2/M3) Mac

Apple Silicon Macを使用している場合、一部の依存関係は`arm64`アーキテクチャ用に特別にコンパイルする必要があります。最新のパッケージはこれを自動的に処理しますが、問題が発生した場合：

```bash
# アーキテクチャを確認
uname -m
# 出力: arm64

# 必要に応じて、Rustに正しいターゲット用にビルドを強制
rustup target add aarch64-apple-darwin
```

### Intel Mac

Intel Macの場合、デフォルトの`x86_64`ターゲットが問題なく動作するはずです：

```bash
# アーキテクチャを確認
uname -m
# 出力: x86_64

# 正しいRustターゲットがインストールされていることを確認
rustup target add x86_64-apple-darwin
```

### コード署名（オプション）

ビルドしたアプリケーションを配布したい場合は、Apple Developer証明書で署名する必要があります：

```bash
# 利用可能な署名ID を確認
security find-identity -v -p codesigning

# 開発者証明書を持っている場合、Tauriが自動的に署名できます
# tauri.conf.jsonに以下を追加：
{
  "bundle": {
    "macOS": {
      "signing": {
        "identity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

## トラブルシューティング

### よくある問題

1. **権限拒否エラー**

   ```bash
   # Homebrewの権限を修正
   sudo chown -R $(whoami) /opt/homebrew
   ```

2. **インストール後にコマンドが見つからない**

   ```bash
   # シェルプロファイルを再読み込み
   source ~/.zshrc
   # またはターミナルを再起動
   ```

3. **ネイティブ依存関係でのビルド失敗**

   ```bash
   # ビルドキャッシュをクリア
   cargo clean
   pnpm clean

   # すべてを再ビルド
   pnpm install
   pnpm tauri build
   ```

4. **Rustターゲットの問題**

   ```bash
   # インストール済みターゲットをリスト表示
   rustup target list --installed

   # システムに適切なターゲットを追加
   rustup target add aarch64-apple-darwin  # Apple Silicon
   rustup target add x86_64-apple-darwin   # Intel
   ```

### ヘルプを得る

ここでカバーされていない問題が発生した場合：

1. [Drop Compress Imageリポジトリ](https://github.com/logue/DropWebP)で既知の問題を確認
2. macOS固有のガイダンスについて[Tauri v2ドキュメント](https://v2.tauri.app/start/prerequisites/)を確認
3. 既存のGitHub Issueを検索するか、新しいIssueを作成

## 次のステップ

Drop Compress Imageのビルドが成功したら：

1. **テストの実行**: `pnpm test`を実行してすべてが正しく動作することを確認
2. **開発**: ホットリロードでの開発には`pnpm dev:tauri`を使用
3. **カスタマイズ**: コードベースを探索して変更を加える
4. **配布**: 配布可能なパッケージを作成するには`pnpm build:tauri`を使用

これでmacOSでDrop Compress Imageを開発およびビルドする準備が整いました！
