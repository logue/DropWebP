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
            <pre class="language-powershell"><code>Set-ExecutionPolicy Bypass -Scope Process -Force;
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec1.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco -v</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my3">
      <v-card-title tag="h3">2. {{ t('sec2.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco install git -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>git --version</code></pre>
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
            <pre class="language-powershell"><code>git clone https://github.com/logue/DropWebP.git
cd DropWebP</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">4. {{ t('sec4.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>choco install visualstudio2022community -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[1]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>choco install visualstudio2022-workload-nativedesktop -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[2]') }}</v-list-item-title>
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
            <pre class="language-powershell"><code>choco install nasm ninja -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec5.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>nasm -v
ninja --version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}:</strong>
          <i18n-t keypath="sec5.notice">
            <template #nasm>
              <abbr title="Netwide Assembler">NASM</abbr>
            </template>
          </i18n-t>
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">6. {{ t('sec6.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco install nodejs pnpm -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>node -v
pnpm -v</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">7. {{ t('sec7.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec7.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec7.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>rustc --version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert type="warning" variant="tonal" class="mt-4">
          <strong>{{ t('warning') }}:</strong>
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
            <pre
              class="language-powershell"
            ><code>git clone https://github.com/microsoft/vcpkg.git</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec8.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>cd vcpkg
.\bootstrap-vcpkg.bat</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">
              <i18n-t keypath="sec8.text[2]">
                <template #code1>
                  <v-code tag="code">C:\path\to\vcpkg</v-code>
                </template>
                <template #code2>
                  <v-code tag="code">PATH</v-code>
                </template>
              </i18n-t>
            </v-list-item-title>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">
              <i18n-t keypath="sec8.text[3]">
                <template #code1>
                  <v-code tag="code">VCPKG_DEFAULT_TRIPLET</v-code>
                </template>
                <template #code2>
                  <v-code tag="code">x64-windows-static-md</v-code>
                </template>
              </i18n-t>
            </v-list-item-title>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec8.text[4]') }}</v-list-item-title>
            <pre class="language-powershell"><code>vcpkg version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="warning" class="mt-4">
          <strong>{{ t('warning') }}:</strong>
          {{ t('sec8.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">9. {{ t('sec9.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec9.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>vcpkg install libavif libheif libjxl libwebp libjpeg-turbo libpng</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}:</strong>
          {{ t('sec9.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="my-3">
      <v-card-title tag="h3">10. {{ t('sec10.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec10.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>pnpm install</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec10.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>cd app
pnpm tauri dev</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="success" class="mt-4">
          <strong>{{ t('complete') }}:</strong>
          {{ t('sec10.notice') }}
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
  warning: Warning
  complete: Complete
  title: Setting Up Development Environment (Windows)
  description: Guide to setting up the development environment on Windows.
  meta_description: Guide to setting up the development environment for Drop Compress Image on Windows.
  sec1:
    title: Install Chocolatey
    text:
      - Open PowerShell as Administrator and run the following command to install Chocolatey.
      - After installation, you can check the version with the command below.
  sec2:
    title: Install Git
    text:
      - Install Git using Chocolatey.
      - After installation, verify the version.
  sec3:
    title: Clone the Project
    text:
      - Clone the project from GitHub and navigate to the project directory.
  sec4:
    title: Install Visual Studio Community 2022
    text:
      - Install Visual Studio Community 2022.
      - Next, install the C++ Desktop Development workload.
      - Once installation is complete, you can verify the installed components using the Visual Studio Installer.
    notice: The C++ Desktop Development workload includes tools necessary for building Rust native extensions, such as MSVC (Microsoft's compiler), Windows SDK, and CMake.
  sec5:
    title: Install NASM and Ninja
    text:
      - Install NASM and Ninja, which are required for building image codec libraries.
      - After installation, verify the versions.
    notice: '{nasm} is an assembler used for building optimized codec libraries like libavif. Ninja is a fast build system often used in conjunction with CMake.'
  sec6:
    title: Install Node.js and pnpm
    text:
      - Install Node.js and pnpm.
      - After installation, verify the versions.
  sec7:
    title: Install Rust (Official Method)
    text:
      - Install Rust using the official method by running the following command in PowerShell or Command Prompt.
      - After installation, verify the version.
    notice: While it's possible to install Rust via Chocolatey, it installs with the MinGW toolchain, which may lead to compatibility issues with libraries.
  sec8:
    title: Set Up vcpkg (Official Instructions)
    text:
      - Run the following command in your desired directory to clone vcpkg.
      - Navigate to the vcpkg directory and run the setup command.
      - 'Add the path to vcpkg.exe (e.g., {code1}) to your environment variable {code2}. How to: Add the vcpkg directory to the "Path" in your system environment variables.'
      - 'Additionally, add an environment variable {code1} and set it to {code2}. This ensures that static libraries for 64-bit Windows are installed by default.'
      - After installation, verify that the path is set and check the version with the command below.
    notice: vcpkg may not function correctly if its path contains non-alphanumeric characters. It's recommended to clone it to a location like the root of a drive.
  sec9:
    title: Install Required Libraries
    text:
      - Run the following command to install the libraries required for image conversion.
    notice: Installation may take some time, and some libraries might fail to build. In such cases, please refer to the vcpkg documentation.
  sec10:
    title: Install Dependencies and Build
    text:
      - Install dependencies in the project directory.
      - Start the application in development mode.
    success: Complete
    notice: The first startup may take some time to compile Rust dependencies. Once the application window appears, the development environment setup is complete.
fr:
  notice: Remarque
  warning: Avertissement
  complete: Terminé
  title: Configuration de l'environnement de développement (Windows)
  description: Guide pour la configuration de l'environnement de développement sur Windows.
  meta_description: Guide pour la configuration de l'environnement de développement de Drop Compress Image sur Windows.
  sec1:
    title: Installer Chocolatey
    text:
      - Ouvrez PowerShell en tant qu'administrateur et exécutez la commande suivante pour installer Chocolatey.
      - Après l'installation, vous pouvez vérifier la version avec la commande ci-dessous.
  sec2:
    title: Installer Git
    text:
      - Installez Git en utilisant Chocolatey.
      - Après l'installation, vérifiez la version.
  sec3:
    title: Cloner le projet
    text:
      - Clonez le projet depuis GitHub et naviguez vers le répertoire du projet.
  sec4:
    title: Installer Visual Studio Community 2022
    text:
      - Installez Visual Studio Community 2022.
      - Ensuite, installez la charge de travail de développement de bureau C++.
      - Une fois l'installation terminée, vous pouvez vérifier les composants installés à l'aide de l'installateur Visual Studio.
    notice: La charge de travail de développement de bureau C++ comprend les outils nécessaires pour construire des extensions natives Rust, tels que MSVC (le compilateur de Microsoft), le SDK Windows et CMake.
  sec5:
    title: Installer NASM et Ninja
    text:
      - Installez NASM et Ninja, qui sont nécessaires pour construire des bibliothèques de codecs d'images.
      - Après l'installation, vérifiez les versions.
    notice: '{nasm} est un assembleur utilisé pour construire des bibliothèques de codecs optimisées comme libavif. Ninja est un système de construction rapide souvent utilisé en conjonction avec CMake.'
    notice_abbr: Netwide Assembler
  sec6:
    title: Installer Node.js et pnpm
    text:
      - Installez Node.js et pnpm.
      - Après l'installation, vérifiez les versions.
  sec7:
    title: Installer Rust (Méthode officielle)
    text:
      - Installez Rust en utilisant la méthode officielle en exécutant la commande suivante dans PowerShell ou l'invite de commandes.
      - Après l'installation, vérifiez la version.
    notice: Bien qu'il soit possible d'installer Rust via Chocolatey, il s'installe avec la chaîne d'outils MinGW, ce qui peut entraîner des problèmes de compatibilité avec les bibliothèques.
  sec8:
    title: Configurer vcpkg (Instructions officielles)
    text:
      - Exécutez la commande suivante dans le répertoire de votre choix pour cloner vcpkg.
      - Naviguez vers le répertoire vcpkg et exécutez la commande de configuration.
      - "Ajoutez le chemin vers vcpkg.exe (par exemple {code1}) à votre variable d'environnement {code2}. Comment faire : Ajoutez le répertoire contenant vcpkg.exe à la variable d'environnement Path."
      - "De plus, ajoutez une variable d'environnement {code1} et définissez-la sur {code2}. Cela garantit que les bibliothèques statiques pour Windows 64 bits sont installées par défaut."
      - Après l'installation, vérifiez que le chemin est défini et vérifiez la version avec la commande ci-dessous.
    notice: vcpkg peut ne pas fonctionner correctement si son chemin contient des caractères non alphanumériques. Il est recommandé de le cloner à un emplacement comme la racine d'un lecteur.
  sec9:
    title: Installer les bibliothèques requises
    text:
      - Exécutez la commande suivante pour installer les bibliothèques requises pour la conversion d'images.
    notice: L'installation peut prendre un certain temps, et certaines bibliothèques peuvent échouer à se construire. Dans ce cas, veuillez vous référer à la documentation de vcpkg.
  sec10:
    title: Installer les dépendances et construire
    text:
      - Installez les dépendances dans le répertoire du projet.
      - Démarrez l'application en mode développement.
    notice: Le premier démarrage peut prendre un certain temps pour compiler les dépendances Rust. Une fois que la fenêtre de l'application apparaît, la configuration de l'environnement de développement est terminée.
ja:
  notice: 補足
  warning: 注意
  complete: 完了
  title: 開発環境を構築する（Windows）
  description: Windowsでの開発環境構築ガイド。
  meta_description: WindowsでのDrop Compress Imageの開発環境構築ガイド。
  sec1:
    title: Chocolateyのインストール
    text:
      - 管理者権限でPowerShellを開き、以下のコマンドを実行してChocolateyをインストールします。
      - インストール後、下記コマンドでバージョンを確認できます。
  sec2:
    title: Gitのインストール
    text:
      - ChocolateyでGitをインストールします。
      - インストール後、バージョンを確認しましょう。
  sec3:
    title: プロジェクトのクローン
    text:
      - GitHubからプロジェクトをクローンし、プロジェクトディレクトリに移動します。
  sec4:
    title: Visual Studio Community 2022のインストール
    text:
      - Visual Studio Community 2022をインストールします。
      - 続いて、C++デスクトップ開発ワークロードをインストールします。
      - インストールが完了したら、Visual Studio Installerでインストール内容を確認できます。
    notice: C++デスクトップ開発ワークロードには、MSVC（Microsoftのコンパイラ）、Windows SDK、CMakeなど、Rustのネイティブ拡張ビルドに必要なツールが含まれています。
  sec5:
    title: NASMとNinjaのインストール
    text:
      - NASMとNinjaをインストールします。これらは画像コーデックライブラリのビルドに必要です。
      - インストール後、バージョンを確認しましょう。
    notice: '{nasm}はアセンブラで、libavifなどの高速化されたコーデックライブラリのビルドに使用されます。Ninjaは高速なビルドシステムで、CMakeと組み合わせて使用されます。'
    notice_abbr: Netwide Assembler
  sec6:
    title: Node.jsとpnpmのインストール
    text:
      - Node.jsとpnpmをインストールします。
      - インストール後、バージョンを確認しましょう。
  sec7:
    title: Rustのインストール（公式推奨）
    text:
      - 公式の方法でRustをインストールします。PowerShellまたはコマンドプロンプトで以下を実行します。
      - インストール後、バージョンを確認しましょう。
    notice: ChocolateyによるRustインストールも可能ですが、MinGWツールチェインでインストールされてしまうためライブラリの互換性で問題が生じます。
  sec8:
    title: vcpkgのセットアップ（公式手順）
    text:
      - 任意のディレクトリで以下のコマンドを実行してvcpkgをクローンします。
      - vcpkgディレクトリに移動し、以下を実行してセットアップします。
      - 'vcpkg.exeのパス（例：{code1}）を環境変数{code2}に追加します。方法：システム環境変数の「Path」にvcpkgディレクトリを追加してください。'
      - 'また、環境変数に{code1}を追加し、値を{code2}に設定します。これにより、64ビットWindows向けの静的ライブラリがデフォルトでインストールされます。'
      - インストール後、下記コマンドでパスが通っていることとバージョンを確認しましょう。
    notice: vcpkgのパスに英数字以外が含まれていると正常に動作しないことがあります。ドライブルートなどにクローンすることを推奨します。
  sec9:
    title: 必要なライブラリのインストール
    text:
      - 以下のコマンドを実行して、画像変換に必要なライブラリをインストールします。
    notice: インストールには時間がかかる場合があります。また、ライブラリによってはビルドに失敗する場合があります。その際はvcpkgのドキュメントを参照してください。
  sec10:
    title: 依存関係のインストールとビルド
    text:
      - プロジェクトディレクトリで依存関係をインストールします。
      - 開発モードでアプリケーションを起動します。
    success: 完了
    notice: 初回起動時は、Rustの依存関係のコンパイルに時間がかかる場合があります。アプリケーションウィンドウが表示されれば、開発環境の構築は完了です。
ko:
  notice: 참고
  warning: 경고
  complete: 완료
  title: 개발 환경 설정 (Windows)
  description: Windows에서 개발 환경을 설정하는 가이드입니다.
  meta_description: Windows에서 Drop Compress Image의 개발 환경을 설정하는 가이드입니다.
  sec1:
    title: Chocolatey 설치
    text:
      - 관리자 권한으로 PowerShell을 열고 다음 명령어를 실행하여 Chocolatey를 설치합니다.
      - 설치 후 아래 명령어로 버전을 확인할 수 있습니다.
  sec2:
    title: Git 설치
    text:
      - Chocolatey를 사용하여 Git을 설치합니다.
      - 설치 후 버전을 확인합니다.
  sec3:
    title: 프로젝트 클론
    text:
      - GitHub에서 프로젝트를 클론하고 프로젝트 디렉토리로 이동합니다.
  sec4:
    title: Visual Studio Community 2022 설치
    text:
      - Visual Studio Community 2022를 설치합니다.
      - 다음으로 C++ 데스크톱 개발 워크로드를 설치합니다.
      - 설치가 완료되면 Visual Studio Installer를 사용하여 설치된 구성 요소를 확인할 수 있습니다.
    notice: C++ 데스크톱 개발 워크로드에는 MSVC(마이크로소프트 컴파일러), Windows SDK 및 CMake와 같은 Rust 네이티브 확장 빌드에 필요한 도구가 포함되어 있습니다.
  sec5:
    title: NASM 및 Ninja 설치
    text:
      - 이미지 코덱 라이브러리 빌드에 필요한 NASM 및 Ninja를 설치합니다.
      - 설치 후 버전을 확인합니다.
    notice: '{nasm}은 libavif와 같은 최적화된 코덱 라이브러리 빌드에 사용되는 어셈블러입니다. Ninja는 CMake와 함께 자주 사용되는 빠른 빌드 시스템입니다.'
    notice_abbr: Netwide Assembler
  sec6:
    title: Node.js 및 pnpm 설치
    text:
      - Node.js 및 pnpm을 설치합니다.
      - 설치 후 버전을 확인합니다.
  sec7:
    title: Rust 설치 (공식 방법)
    text:
      - PowerShell 또는 명령 프롬프트에서 다음 명령어를 실행하여 공식 방법으로 Rust를 설치합니다.
      - 설치 후 버전을 확인합니다.
    notice: Chocolatey를 통해 Rust를 설치할 수도 있지만 MinGW 툴체인으로 설치되어 라이브러리와의 호환성 문제가 발생할 수 있습니다.
  sec8:
    title: vcpkg 설정 (공식 지침)
    text:
      - 원하는 디렉토리에서 다음 명령어를 실행하여 vcpkg를 클론합니다.
      - vcpkg 디렉토리로 이동하여 설정 명령어를 실행합니다.
      - 'vcpkg.exe의 경로(예: {code1})를 환경 변수 {code2}에 추가합니다. 방법: 시스템 환경 변수의 "Path"에 vcpkg 디렉토리를 추가하세요.'
      - '또한 환경 변수 {code1}을 추가하고 값을 {code2}로 설정합니다. 이렇게 하면 64비트 Windows용 정적 라이브러리가 기본적으로 설치됩니다.'
      - 설치 후 아래 명령어로 경로가 설정되었는지와 버전을 확인합니다.
    notice: vcpkg 경로에 영숫자가 아닌 문자가 포함되어 있으면 제대로 작동하지 않을 수 있습니다. 드라이브 루트와 같은 위치에 클론하는 것이 좋습니다.
  sec9:
    title: 필요한 라이브러리 설치
    text:
      - 다음 명령어를 실행하여 이미지 변환에 필요한 라이브러리를 설치합니다.
    notice: 설치하는 데 시간이 걸릴 수 있으며 일부 라이브러리는 빌드에 실패할 수 있습니다. 이 경우 vcpkg 문서를 참조하세요.
  sec10:
    title: 종속성 설치 및 빌드
    text:
      - 프로젝트 디렉토리에서 종속성을 설치합니다.
      - 개발 모드에서 애플리케이션을 시작합니다.
    notice: 첫 번째 시작 시 Rust 종속성 컴파일에 시간이 걸릴 수 있습니다. 애플리케이션 창이 나타나면 개발 환경 설정이 완료된 것입니다.
zhHans:
  notice: 注意
  warning: 警告
  complete: 完成
  title: 设置开发环境（Windows）
  description: 在Windows上设置开发环境的指南。
  meta_description: 在Windows上为Drop Compress Image设置开发环境的指南。
  sec1:
    title: 安装Chocolatey
    text:
      - 以管理员身份打开PowerShell并运行以下命令以安装Chocolatey。
      - 安装后，您可以使用以下命令检查版本。
  sec2:
    title: 安装Git
    text:
      - 使用Chocolatey安装Git。
      - 安装后，验证版本。
  sec3:
    title: 克隆项目
    text:
      - 从GitHub克隆项目并导航到项目目录。
  sec4:
    title: 安装Visual Studio Community 2022
    text:
      - 安装Visual Studio Community 2022。
      - 接下来，安装C++桌面开发工作负载。
      - 安装完成后，您可以使用Visual Studio安装程序验证已安装的组件。
    notice: C++桌面开发工作负载包括构建Rust本机扩展所需的工具，例如MSVC（Microsoft的编译器）、Windows SDK和CMake。
  sec5:
    title: 安装NASM和Ninja
    text:
      - 安装NASM和Ninja，这些是构建图像编解码器库所必需的。
      - 安装后，验证版本。
    notice: '{nasm}是一种汇编器，用于构建优化的编解码器库，如libavif。Ninja是一种快速的构建系统，通常与CMake一起使用。'
    notice_abbr: Netwide Assembler
  sec6:
    title: 安装Node.js和pnpm
    text:
      - 安装Node.js和pnpm。
      - 安装后，验证版本。
  sec7:
    title: 安装Rust（官方方法）
    text:
      - 在PowerShell或命令提示符中运行以下命令，使用官方方法安装Rust。
      - 安装后，验证版本。
    notice: 虽然可以通过Chocolatey安装Rust，但它会使用MinGW工具链进行安装，这可能会导致与库的兼容性问题。
  sec8:
    title: 设置vcpkg（官方说明）
    text:
      - 在所需目录中运行以下命令以克隆vcpkg。
      - 导航到vcpkg目录并运行设置命令。
      - '将vcpkg.exe的路径（例如{code1}）添加到环境变量{code2}中。方法：将vcpkg目录添加到系统环境变量中的“Path”。'
      - '此外，添加环境变量{code1}并将其设置为{code2}。这确保默认安装适用于64位Windows的静态库。'
      - 安装后，验证路径是否设置，并使用以下命令检查版本。
    notice: 如果vcpkg的路径包含非字母数字字符，可能无法正常工作。建议将其克隆到驱动器根目录等位置。
  sec9:
    title: 安装所需库
    text:
      - 运行以下命令以安装图像转换所需的库。
    notice: 安装可能需要一些时间，并且某些库可能会构建失败。在这种情况下，请参阅vcpkg文档。
  sec10:
    title: 安装依赖项并构建
    text:
      - 在项目目录中安装依赖项。
      - 以开发模式启动应用程序。
    notice: 第一次启动可能需要一些时间来编译Rust依赖项。一旦应用程序窗口出现，开发环境设置就完成了。
zhHant:
  notice: 注意
  warning: 警告
  complete: 完成
  title: 設置開發環境（Windows）
  description: 在Windows上設置開發環境的指南。
  meta_description: 在Windows上為Drop Compress Image設置開發環境的指南。
  sec1:
    title: 安裝Chocolatey
    text:
      - 以管理員身份打開PowerShell並運行以下命令以安裝Chocolatey。
      - 安裝後，您可以使用以下命令檢查版本。
  sec2:
    title: 安裝Git
    text:
      - 使用Chocolatey安裝Git。
      - 安裝後，驗證版本。
  sec3:
    title: 克隆項目
    text:
      - 從GitHub克隆項目並導航到項目目錄。
  sec4:
    title: 安裝Visual Studio Community 2022
    text:
      - 安裝Visual Studio Community 2022。
      - 接下來，安裝C++桌面開發工作負載。
      - 安裝完成後，您可以使用Visual Studio安裝程序驗證已安裝的組件。
    notice: C++桌面開發工作負載包括構建Rust本機擴展所需的工具，例如MSVC（Microsoft的編譯器）、Windows SDK和CMake。
  sec5:
    title: 安裝NASM和Ninja
    text:
      - 安裝NASM和Ninja，這些是構建圖像編解碼器庫所必需的。
      - 安裝後，驗證版本。
    notice: '{nasm}是一種彙編器，用於構建優化的編解碼器庫，如libavif。Ninja是一種快速的構建系統，通常與CMake一起使用。'
    notice_abbr: Netwide Assembler
  sec6:
    title: 安裝Node.js和pnpm
    text:
      - 安裝Node.js和pnpm。
      - 安裝後，驗證版本。
  sec7:
    title: 安裝Rust（官方方法）
    text:
      - 在PowerShell或命令提示符中運行以下命令，使用官方方法安裝Rust。
      - 安裝後，驗證版本。
    notice: 雖然可以通過Chocolatey安裝Rust，但它會使用MinGW工具鏈進行安裝，這可能會導致與庫的兼容性問題。
  sec8:
    title: 設置vcpkg（官方說明）
    text:
      - 在所需目錄中運行以下命令以克隆vcpkg。
      - 導航到vcpkg目錄並運行設置命令。
      - '將vcpkg.exe的路徑（例如{code1}）添加到環境變量{code2}中。方法：將vcpkg目錄添加到系統環境變量中的“Path”。'
      - '此外，添加環境變量{code1}並將其設置為{code2}。這確保默認安裝適用於64位Windows的靜態庫。'
      - 安裝後，驗證路徑是否設置，並使用以下命令檢查版本。
    notice: 如果vcpkg的路徑包含非字母數字字符，可能無法正常工作。建議將其克隆到驅動器根目錄等位置。
  sec9:
    title: 安裝所需庫
    text:
      - 運行以下命令以安裝圖像轉換所需的庫。
    notice: 安裝可能需要一些時間，並且某些庫可能會構建失敗。在這種情況下，請參閱vcpkg文檔。
  sec10:
    title: 安裝依賴項並構建
    text:
      - 在項目目錄中安裝依賴項。
      - 以開發模式啟動應用程序。
    notice: 第一次啟動可能需要一些時間來編譯Rust依賴項。一旦應用程序窗口出現，開發環境設置就完成了。
</i18n>
