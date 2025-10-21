<template>
  <TableOfContents />
  <div class="content-with-toc">
    <h2>{{ t('title') }}</h2>
    <p>{{ t('description') }}</p>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">1. {{ t('sec1.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec1.text[0]') }}</v-list-item-title>
            <pre class="language-bash"><code>git clone https://github.com/logue/DropWebP.git
cd DropWebP</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">2. {{ t('sec2.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[0]') }}</v-list-item-title>
            <pre class="language-bash"><code>xcode-select --install</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[1]') }}</v-list-item-title>
            <pre class="language-bash"><code>xcode-select -p</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">3. {{ t('sec3.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec3.text[0]') }}</v-list-item-title>
            <pre
              class="language-bash"
            ><code>/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">4. {{ t('sec4.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">
              {{ t('sec4.') }}
              <i18n-t keypath="sec4.text[0]" tag="span">
                <template #code>
                  <v-code tag="code">~/.zshrc</v-code>
                </template>
              </i18n-t>
            </v-list-item-title>
            <pre class="language-bash"><code>brew install asdf
echo -e "\n. $(brew --prefix asdf)/libexec/asdf.sh" >> ~/.zshrc
source ~/.zshrc</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[1]') }}</v-list-item-title>
            <pre
              class="language-bash"
            ><code>asdf plugin add nodejs https://github.com/asdf-vm/asdf-nodejs.git
asdf plugin add pnpm https://github.com/asdf-community/asdf-pnpm.git</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[2]') }}</v-list-item-title>
            <pre class="language-bash"><code>asdf install</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[3]') }}</v-list-item-title>
            <pre class="language-bash"><code>node -v
pnpm -v</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}:</strong>
          {{ t('sec4.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">5. {{ t('sec5.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec5.text[0]') }}</v-list-item-title>
            <pre
              class="language-bash"
            ><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec5.text[1]') }}</v-list-item-title>
            <pre class="language-bash"><code>rustc --version
cargo --version</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">6. {{ t('sec6.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[0]') }}</v-list-item-title>
            <pre class="language-bash"><code>brew install nasm ninja cmake pkg-config</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[1]') }}</v-list-item-title>
            <pre class="language-bash"><code>nasm --version
ninja --version
cmake --version
pkg-config --version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}:</strong>
          <i18n-t keypath="sec6.notice">
            <template #nasm>
              <abbr title="Netwide Assembler">NASM</abbr>
            </template>
          </i18n-t>
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">7. {{ t('sec7.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec7.text[0]') }}</v-list-item-title>
            <pre
              class="language-bash"
            ><code>brew install libavif libheif libjxl libwebp libjpeg-turbo libpng</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec7.text[1]') }}</v-list-item-title>
            <pre class="language-bash"><code>pkg-config --modversion libavif
pkg-config --modversion libheif
pkg-config --modversion libjxl
pkg-config --modversion libwebp</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}:</strong>
          {{ t('sec7.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">8. {{ t('sec8.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec8.text[0]') }}</v-list-item-title>
            <pre class="language-bash"><code>pnpm install</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec8.text[1]') }}</v-list-item-title>
            <pre class="language-bash"><code>cd app
pnpm tauri dev</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="success" class="mt-4">
          <strong>{{ t('complete') }}:</strong>
          {{ t('sec8.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import Prism from 'prismjs';

import TableOfContents from '@/components/TableOfContents.vue';

const { t } = useI18n();

useHead({
  title: t('title'),
  meta: [{ name: 'description', content: t('meta_description') }]
});

onMounted(() => {
  Prism.highlightAll();
});
</script>

<style scoped>
.content-with-toc {
  margin-left: 280px;
  padding: 24px;
}

@media (max-width: 1264px) {
  .content-with-toc {
    margin-left: 0;
  }
}
</style>

<i18n lang="yaml">
en:
  notice: Note
  complete: Complete
  title: Building Environment Setup on macOS
  description: Instructions for setting up the build environment on macOS.
  meta_description: Instructions for setting up the build environment for Drop Compress Image on macOS.
  sec1:
    title: Clone the Project
    text:
      - First, clone the project from GitHub and navigate to the project directory.
  sec2:
    title: Install Xcode Command Line Tools
    text:
      - Install the Xcode command line tools by running the following command in the terminal.
      - After installation, verify the version.
  sec3:
    title: Install Homebrew
    text:
      - Next, install Homebrew.
  sec4:
    title: Install asdf
    text:
      - Install asdf via Homebrew and add the initialization script to your shell configuration file ({code}).
      - Add plugins for Node.js and pnpm.
      - Install the versions specified in the .tool-versions file within the project directory.
      - Verify that the installation was successful by checking the versions.
    notice: The .tool-versions file in the project root defines the required versions of Node.js and pnpm.
  sec5:
    title: Set Up Rust
    text:
      - Install Rust using rustup.
      - Verify that the installation was successful by checking the versions.
  sec6:
    title: Install Other Tools
    text:
      - Install other necessary tools (NASM, Ninja, CMake, pkg-config).
      - Verify that the installation was successful by checking the versions of each tool.
    notice: '{nasm} is an assembler used for building optimized codec libraries like libavif. Ninja is a fast build system often used in conjunction with CMake.'
    notice_abbr: Netwide Assembler
  sec7:
    title: Install Image Codec Libraries
    text:
      - Install major image codec libraries via Homebrew.
      - Verify that the installation was successful by checking the versions of each library.
    notice: These libraries are used for image conversion processing in Rust. Managing them with Homebrew ensures consistent versions across the system.
  sec8:
    title: Install Dependencies and Build
    text:
      - Install dependencies in the project directory.
      - Start the application in development mode.
    notice: The first launch may take some time to compile Rust dependencies. Once the application window appears, the development environment setup is complete.
fr:
  notice: Remarque
  complete: Terminé
  title: Configuration de l'environnement de construction sur macOS
  description: Instructions pour configurer l'environnement de construction sur macOS.
  meta_description: Instructions pour configurer l'environnement de construction de Drop Compress Image sur macOS.
  sec1:
    title: Cloner le projet
    text:
      - Tout d'abord, clonez le projet depuis GitHub et accédez au répertoire du projet.
  sec2:
    title: Installer les outils en ligne de commande Xcode
    text:
      - Installez les outils en ligne de commande Xcode en exécutant la commande suivante dans le terminal.
      - Après l'installation, vérifiez la version.
  sec3:
    title: Installer Homebrew
    text:
      - Ensuite, installez Homebrew.
  sec4:
    title: Installer asdf
    text:
      - Installez asdf via Homebrew et ajoutez le script d'initialisation à votre fichier de configuration de shell ({code}).
      - Ajoutez des plugins pour Node.js et pnpm.
      - Installez les versions spécifiées dans le fichier .tool-versions dans le répertoire du projet.
      - Vérifiez que l'installation a réussi en vérifiant les versions.
    notice: Le fichier .tool-versions à la racine du projet définit les versions requises de Node.js et pnpm.
  sec5:
    title: Configurer Rust
    text:
      - Installez Rust en utilisant rustup.
      - Vérifiez que l'installation a réussi en vérifiant les versions.
  sec6:
    title: Installer d'autres outils
    text:
      - Installez d'autres outils nécessaires (NASM, Ninja, CMake, pkg-config).
      - Vérifiez que l'installation a réussi en vérifiant les versions de chaque outil.
    notice: '{nasm} est un assembleur utilisé pour construire des bibliothèques de codecs optimisées comme libavif. Ninja est un système de construction rapide souvent utilisé avec CMake.'
    notice_abbr: Assembleur Netwide
  sec7:
    title: Installer les bibliothèques de codecs d'images
    text:
      - Installez les principales bibliothèques de codecs d'images via Homebrew.
      - Vérifiez que l'installation a réussi en vérifiant les versions de chaque bibliothèque.
    notice: Ces bibliothèques sont utilisées pour le traitement de la conversion d'images en Rust. Les gérer avec Homebrew garantit des versions cohérentes sur l'ensemble du système.
  sec8:
    title: Installer les dépendances et construire
    text:
      - Installez les dépendances dans le répertoire du projet.
      - Démarrez l'application en mode développement.
    notice: Le premier lancement peut prendre du temps pour compiler les dépendances Rust. Une fois que la fenêtre de l'application apparaît, la configuration de l'environnement de développement est terminée.
ja:
  notice: 補足
  complete: 完了
  title: macOS でのビルド環境構築
  description: macOS でのビルド環境構築手順を解説します。
  meta_description: Drop Compress Image の macOS 上でのビルド環境構築手順を解説します。
  sec1:
    title: プロジェクトのクローン
    text:
      - まず、GitHub からプロジェクトをクローンし、プロジェクトディレクトリに移動します。
  sec2:
    title: Xcode コマンドラインツールのインストール
    text:
      - Xcode コマンドラインツールをインストールします。ターミナルで以下のコマンドを実行し、インストールを開始します。
      - インストールが完了したら、バージョンを確認します。
  sec3:
    title: Homebrew のインストール
    text:
      - 次に、Homebrew をインストールします。
  sec4:
    title: asdf のインストール
    text:
      - asdf を Homebrew 経由でインストールし、シェル設定ファイル（{code}）に初期化スクリプトを追加します。
      - Node.js と pnpm のプラグインを追加します。
      - プロジェクトディレクトリ内で、.tool-versions ファイルで指定されたバージョンをインストールします。
      - インストールが成功したか、バージョンを確認します。
    notice: プロジェクトルートの .tool-versions ファイルには、必要な Node.js と pnpm のバージョンが定義されています。
  sec5:
    title: Rust のセットアップ
    text:
      - rustup を使って Rust をインストールします。
      - インストールが成功したか、バージョンを確認します。
  sec6:
    title: その他のツールのインストール
    text:
      - その他、必要なツール（NASM, Ninja, CMake, pkg-config）をインストールします。
      - インストールが成功したか、各ツールのバージョンを確認します。
    notice: '{nasm} は、libavif のような最適化されたコーデックライブラリのビルドに使用されるアセンブラです。Ninja は CMake と組み合わせてよく使用される高速なビルドシステムです。'
    notice_abbr: Netwide Assembler
  sec7:
    title: 画像コーデックライブラリのインストール
    text:
      - 主要な画像コーデックライブラリを Homebrew 経由でインストールします。
      - インストールが成功したか、各ライブラリのバージョンを確認します。
    notice: これらのライブラリは、Rust の画像変換処理で使用されます。Homebrew で管理することで、システム全体で一貫したバージョンを利用できます。
  sec8:
    title: 依存関係のインストールとビルド
    text:
      - プロジェクトディレクトリで依存関係をインストールします。
      - 開発モードでアプリケーションを起動します。
    notice: 初回起動時は、Rust の依存関係のコンパイルに時間がかかる場合があります。アプリケーションウィンドウが表示されれば、開発環境の構築は完了です。
ko:
  notice: 참고
  complete: 완료
  title: macOS에서의 빌드 환경 설정
  description: macOS에서의 빌드 환경 설정 방법을 설명합니다.
  meta_description: Drop Compress Image의 macOS에서의 빌드 환경 설정 방법을 설명합니다.
  sec1:
    title: 프로젝트 클론
    text:
      - 먼저, GitHub에서 프로젝트를 클론하고 프로젝트 디렉토리로 이동합니다.
  sec2:
    title: Xcode 명령줄 도구 설치
    text:
      - Xcode 명령줄 도구를 설치합니다. 터미널에서 다음 명령어를 실행하여 설치를 시작합니다.
      - 설치가 완료되면 버전을 확인합니다.
  sec3:
    title: Homebrew 설치
    text:
      - 다음으로, Homebrew를 설치합니다.
  sec4:
    title: asdf 설치
    text:
      - Homebrew를 통해 asdf를 설치하고 셸 설정 파일({code})에 초기화 스크립트를 추가합니다.
      - Node.js와 pnpm 플러그인을 추가합니다.
      - 프로젝트 디렉토리 내의 .tool-versions 파일에 지정된 버전을 설치합니다.
      - 설치가 성공했는지 버전을 확인합니다.
    notice: 프로젝트 루트의 .tool-versions 파일에는 필요한 Node.js와 pnpm 버전이 정의되어 있습니다.
  sec5:
    title: Rust 설정
    text:
      - rustup을 사용하여 Rust를 설치합니다.
      - 설치가 성공했는지 버전을 확인합니다.
  sec6:
    title: 기타 도구 설치
    text:
      - 기타 필요한 도구(NASM, Ninja, CMake, pkg-config)를 설치합니다.
      - 각 도구의 버전을 확인하여 설치가 성공했는지 확인합니다.
    notice: 이는 이후 이미지 코덱 라이브러리 빌드에 필요합니다.
  sec7:
    title: 이미지 코덱 라이브러리 설치
    text:
      - 주요 이미지 코덱 라이브러리를 Homebrew를 통해 설치합니다.
      - 각 라이브러리의 버전을 확인하여 설치가 성공했는지 확인합니다.
    notice: 이들 라이브러리는 Rust의 이미지 변환 처리에 사용됩니다. Homebrew로 관리하면 시스템 전체에서 일관된 버전을 사용할 수 있습니다.
  sec8:
    title: 종속성 설치 및 빌드
    text:
      - 프로젝트 디렉토리에서 종속성을 설치합니다.
      - 개발 모드에서 애플리케이션을 시작합니다.
    notice: 첫 실행 시 Rust 종속성 컴파일에 시간이 걸릴 수 있습니다. 애플리케이션 창이 나타나면 개발 환경 설정이 완료된 것입니다.
zhHans:
  notice: 注意
  complete: 完成
  title: 在 macOS 上设置构建环境
  description: 介绍在 macOS 上设置构建环境的步骤。
  meta_description: 介绍在 macOS 上为 Drop Compress Image 设置构建环境的步骤。
  sec1:
    title: 克隆项目
    text:
      - 首先，从 GitHub 克隆项目并导航到项目目录。
  sec2:
    title: 安装 Xcode 命令行工具
    text:
      - 通过在终端中运行以下命令来安装 Xcode 命令行工具。
      - 安装完成后，验证版本。
  sec3:
    title: 安装 Homebrew
    text:
      - 接下来，安装 Homebrew。
  sec4:
    title: 安装 asdf
    text:
      - 通过 Homebrew 安装 asdf 并将初始化脚本添加到您的 shell 配置文件（{code}）。
      - 为 Node.js 和 pnpm 添加插件。
      - 安装项目目录中的 .tool-versions 文件中指定的版本。
      - 通过检查版本来验证安装是否成功。
    notice: 项目根目录中的 .tool-versions 文件定义了所需的 Node.js 和 pnpm 版本。
  sec5:
    title: 设置 Rust
    text:
      - 使用 rustup 安装 Rust。
      - 通过检查版本来验证安装是否成功。
  sec6:
    title: 安装其他工具
    text:
      - 安装其他必要的工具（NASM、Ninja、CMake、pkg-config）。
      - 通过检查每个工具的版本来验证安装是否成功。
    notice: 这些工具稍后用于构建图像编解码器库。
  sec7:
    title: 安装图像编解码器库
    text:
      - 通过 Homebrew 安装主要的图像编解码器库。
      - 通过检查每个库的版本来验证安装是否成功。
    notice: 这些库用于 Rust 中的图像转换处理。通过 Homebrew 管理它们可确保系统范围内版本的一致性。
  sec8:
    title: 安装依赖项并构建
    text:
      - 在项目目录中安装依赖项。
      - 在开发模式下启动应用程序。
    notice: 首次启动可能需要一些时间来编译 Rust 依赖项。一旦应用程序窗口出现，开发环境设置即完成。
zhHant:
  notice: 注意
  complete: 完成
  title: 在 macOS 上設置構建環境
  description: 介紹在 macOS 上設置構建環境的步驟。
  meta_description: 介紹在 macOS 上為 Drop Compress Image 設置構建環境的步驟。
  sec1:
    title: 克隆項目
    text:
      - 首先，從 GitHub 克隆項目並導航到項目目錄。
  sec2:
    title: 安裝 Xcode 命令行工具
    text:
      - 通過在終端中運行以下命令來安裝 Xcode 命令行工具。
      - 安裝完成後，驗證版本。
  sec3:
    title: 安裝 Homebrew
    text:
      - 接下來，安裝 Homebrew。
  sec4:
    title: 安裝 asdf
    text:
      - 通過 Homebrew 安裝 asdf 並將初始化腳本添加到您的 shell 配置文件（{code}）。
      - 為 Node.js 和 pnpm 添加插件。
      - 安裝項目目錄中的 .tool-versions 文件中指定的版本。
      - 通過檢查版本來驗證安裝是否成功。
    notice: 項目根目錄中的 .tool-versions 文件定義了所需的 Node.js 和 pnpm 版本。
  sec5:
    title: 設置 Rust
    text:
      - 使用 rustup 安裝 Rust。
      - 通過檢查版本來驗證安裝是否成功。
  sec6:
    title: 安裝其他工具
    text:
      - 安裝其他必要的工具（NASM、Ninja、CMake、pkg-config）。
      - 通過檢查每個工具的版本來驗證安裝是否成功。
    notice: 這些工具稍後用於構建圖像編解碼器庫。
  sec7:
    title: 安裝圖像編解碼器庫
    text:
      - 通過 Homebrew 安裝主要的圖像編解碼器庫。
      - 通過檢查每個庫的版本來驗證安裝是否成功。
    notice: 這些庫用於 Rust 中的圖像轉換處理。通過 Homebrew 管理它們可確保系統範圍內版本的一致性。
  sec8:
    title: 安裝依賴項並構建
    text:
      - 在項目目錄中安裝依賴項。
      - 在開發模式下啟動應用程序。
    notice: 首次啟動可能需要一些時間來編譯 Rust 依賴項。一旦應用程序窗口出現，開發環境設置即完成。
</i18n>
