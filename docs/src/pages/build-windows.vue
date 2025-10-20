<template>
  <TableOfContents />

  <div class="content-with-toc">
    <h2>{{ t('title') }}</h2>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">1. {{ t('sec1.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec1.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>Set-ExecutionPolicy Bypass -Scope Process -Force;
`[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;`
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec1.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco -v</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">2. {{ t('sec2.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>choco install visualstudio2022community -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[1]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>choco install visualstudio2022-workload-nativedesktop -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec2.text[2]') }}</v-list-item-title>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}</strong>
          {{ t('sec2.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">3. {{ t('sec3.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec3.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco install nasm ninja -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec3.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>nasm -v
ninja --version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}</strong>
          {{ t('sec3.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">4. {{ t('sec4.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco install nodejs pnpm -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec4.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>node -v
pnpm -v</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">5. {{ t('sec5.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec5.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec5.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>rustc --version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert type="warning" variant="tonal" class="mt-4">
          <strong>{{ t('warning') }}</strong>
          {{ t('sec5.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">6. {{ t('sec6.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>git clone https://github.com/microsoft/vcpkg.git</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>cd vcpkg
.\bootstrap-vcpkg.bat</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[2]') }}</v-list-item-title>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[3]') }}</v-list-item-title>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec6.text[4]') }}</v-list-item-title>
            <pre class="language-powershell"><code>vcpkg version</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="warning" class="mt-4">
          <strong>{{ t('warning') }}</strong>
          {{ t('sec6.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">7. {{ t('sec7.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec7.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>choco install git -y</code></pre>
          </v-list-item>
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec7.text[1]') }}</v-list-item-title>
            <pre class="language-powershell"><code>git --version</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">8. {{ t('sec8.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec8.text[0]') }}</v-list-item-title>
            <pre class="language-powershell"><code>git --version</code></pre>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">8. {{ t('sec8.title') }}</v-card-title>
      <v-card-text>
        <v-list tag="ol">
          <v-list-item tag="li">
            <v-list-item-title class="text-wrap">{{ t('sec8.text[0]') }}</v-list-item-title>
            <pre
              class="language-powershell"
            ><code>vcpkg install libavif libheif libjxl libwebp</code></pre>
          </v-list-item>
        </v-list>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}</strong>
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

onMounted(() => {
  Prism.highlightAll();
});
</script>

<style scoped>
.content-with-toc {
  margin-right: 280px;
  padding: 24px;
}

@media (max-width: 1264px) {
  .content-with-toc {
    margin-right: 0;
  }
}
</style>

<i18n lang="yaml">
en:
  notice: Note
  warning: Warning
  title: Setting Up Development Environment (Windows)
  sec1:
    title: Install Chocolatey
    text:
      - Open PowerShell as Administrator and run the following command to install Chocolatey.
      - After installation, you can check the version with the command below.
  sec2:
    title: Install Visual Studio Community 2022
    text:
      - Open PowerShell or Command Prompt as Administrator and run the following command to install Visual Studio Community 2022.
      - Next, install the C++ Desktop Development workload.
      - Once installation is complete, you can verify the installed components using the Visual Studio Installer.
    notice: The C++ Desktop Development workload includes tools necessary for building Rust native extensions, such as MSVC (Microsoft's compiler), Windows SDK, and CMake.
  sec3:
    title: Install NASM and Ninja
    text:
      - Install NASM and Ninja, which are required for building image codec libraries.
      - After installation, verify the versions.
    notice: <abbr title="Netwide Assembler">NASM</abbr> is an assembler used for building optimized codec libraries like libavif. Ninja is a fast build system often used in conjunction with CMake.
  sec4:
    title: Install Node.js and pnpm
    text:
      - Open PowerShell or Command Prompt as Administrator and run the following command to install Node.js and pnpm.
      - After installation, verify the versions.
  sec5:
    title: Install Rust (Official Method)
    text:
      - Install Rust using the official method by running the following command in PowerShell or Command Prompt.
      - After installation, verify the version.
    notice: While it's possible to install Rust via Chocolatey, it installs with the MinGW toolchain, which may lead to compatibility issues with libraries.
  sec6:
    title: Set Up vcpkg (Official Instructions)
    text:
      - Run the following command in your desired directory to clone vcpkg.
      - Navigate to the vcpkg directory and run the setup command.
      - 'Add the path to vcpkg.exe (e.g., <code>C:\path\to\vcpkg</code>) to your environment variable PATH.<br />How to: Add the vcpkg directory to the "Path" in your system environment variables.'
      - Additionally, add an environment variable VCPKG_DEFAULT_TRIPLET and set it to <code>x64-windows-static-md</code>. This ensures that static libraries for 64-bit Windows are installed by default.
      - After installation, verify that the path is set and check the version with the command below.
    notice: vcpkg may not function correctly if its path contains non-alphanumeric characters. It's recommended to clone it to a location like the root of a drive.
  sec7:
    title: Install Git
    text:
      - Install Git using Chocolatey. Optionally, install GUI clients like Tortoise Git if needed.
      - After installation, verify the version.
  sec8:
    title: Install Required Libraries
    text:
      - Run the following command to install the libraries required for image conversion.
    notice: Installation may take some time, and some libraries might fail to build. In such cases, please refer to the vcpkg documentation.
ja:
  notice: 補足
  warning: 注意
  title: 開発環境を構築する（Windows）
  sec1:
    title: Chocolateyのインストール
    text:
      - 管理者権限でPowerShellを開き、以下のコマンドを実行してChocolateyをインストールします。
      - インストール後、下記コマンドでバージョンを確認できます。
  sec2:
    title: Visual Studio Community 2022のインストール
    text:
      - 管理者権限でPowerShellまたはコマンドプロンプトを開き、以下のコマンドを実行してVisual Studio Community 2022をインストールします。
      - 続いて、C++デスクトップ開発ワークロードをインストールします。
      - インストールが完了したら、Visual Studio Installerでインストール内容を確認できます。
    notice: C++デスクトップ開発ワークロードには、MSVC（Microsoftのコンパイラ）、Windows SDK、CMakeなど、Rustのネイティブ拡張ビルドに必要なツールが含まれています。
  sec3:
    title: NASMとNinjaのインストール
    text:
      - NASMとNinjaをインストールします。これらは画像コーデックライブラリのビルドに必要です。
      - インストール後、バージョン確認しましょう。
    notice: <abbr title="Netwide Assembler">NASM</abbr>はアセンブラで、libavifなどの高速化されたコーデックライブラリのビルドに使用されます。Ninjaは高速なビルドシステムで、CMakeと組み合わせて使用されます。
  sec4:
    title: Node.jsとpnpmのインストール
    text:
      - 管理者権限でPowerShellまたはコマンドプロンプトを開き、以下のコマンドを実行してNode.jsと、pnpmをインストールします。
      - インストール後、バージョン確認しましょう。
  sec5:
    title: Rustのインストール（公式推奨）
    text:
      - 公式の方法でRustをインストールします。PowerShellまたはコマンドプロンプトで以下を実行します。
      - インストール後、バージョン確認しましょう。
    notice: ChocolateyによるRustインストールも可能ですが、MinGWツールチェインでインストールされてしまうためライブラリの互換性で問題が生じます。
  sec6:
    title: vcpkgのセットアップ（公式手順）
    text:
      - 任意のディレクトリで以下のコマンドを実行してvcpkgをクローンします。
      - vcpkgディレクトリに移動し、以下を実行してセットアップします。
      - vcpkg.exeのパス（例：<code>C:\path\to\vcpkg</code>）を環境変数PATHに追加します。<br />方法：システム環境変数の「Path」にvcpkgディレクトリを追加してください。
      - また、環境変数にVCPKG_DEFAULT_TRIPLETを追加し、値を<code>x64-windows-static-md</code>に設定します。これにより、64ビットWindows向けの静的ライブラリがデフォルトでインストールされます。
      - インストール後、下記コマンドでパスが通っていることとバージョン確認しましょう。
    notice: vcpkgのパスに英数字以外が含まれていると正常に動作しないことがあります。ドライブルートなどにクローンすることを推奨します。
  sec7:
    title: Gitのインストール
    text:
      - ChocolateyでGitをインストールします。必要に応じてTortoise GitなどのGUIクライアントもインストールしてください。
      - インストール後、バージョン確認しましょう。
  sec8:
    title: 必要なライブラリのインストール
    text:
      - 以下のコマンドを実行して、画像変換に必要なライブラリをインストールします。
    notice: インストールには時間がかかる場合があります。また、ライブラリによってはビルドに失敗する場合があります。その際はvcpkgのドキュメントを参照してください。
</i18n>
