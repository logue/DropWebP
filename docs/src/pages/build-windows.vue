<template>
  <TableOfContents />
  <div class="content-with-toc">
    <h2>{{ t('title') }}</h2>
    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">1. {{ t('sec1.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec1.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">
Set-ExecutionPolicy Bypass -Scope Process -Force;
`[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;`
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
              </pre>
            </v-card>
          </li>
          <li>
            {{ t('sec1.text.2') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">choco -v</pre>
            </v-card>
          </li>
        </ol>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">2. {{ t('sec2.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec2.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">choco install visualstudio2022community -y</pre>
            </v-card>
          </li>
          <li>
            {{ t('sec2.text.1') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">choco install visualstudio2022-workload-nativedesktop -y</pre>
            </v-card>
          </li>
          <li>{{ t('sec2.text.2') }}</li>
        </ol>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}</strong>
          {{ t('sec2.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">3. {{ t('sec3.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec3.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">choco install nasm ninja -y</pre>
            </v-card>
          </li>
          <li>
            {{ t('sec3.text.1') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">
nasm -v
ninja --version
              </pre>
            </v-card>
          </li>
        </ol>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}</strong>
          <template v-html="t('sec3.notice')" />
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">4. {{ t('sec4.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec4.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">choco install nodejs pnpm -y</pre>
            </v-card>
          </li>
          <li>
            {{ t('sec4.text.1') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">
node -v
pnpm -v
              </pre>
            </v-card>
          </li>
        </ol>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">5. {{ t('sec5.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec5.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</pre
              >
            </v-card>
          </li>
          <li>
            {{ t('sec5.text.1') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">rustc --version</pre>
            </v-card>
          </li>
        </ol>
        <v-alert type="warning" variant="tonal" class="mt-4">
          <strong>{{ t('warning') }}</strong>
          {{ t('sec5.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">6. {{ t('sec6.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec6.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">git clone https://github.com/microsoft/vcpkg.git</pre>
            </v-card>
          </li>
          <li>
            {{ t('sec6.text.1') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">
cd vcpkg
.\bootstrap-vcpkg.bat
              </pre>
            </v-card>
          </li>
          <li>
            <template v-html="t('sec6.text.2')" />
          </li>
          <li>
            <template v-html="t('sec6.text.3')" />
          </li>
          <li>
            {{ t('sec6.text.4') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">vcpkg version</pre>
            </v-card>
          </li>
        </ol>
        <v-alert variant="tonal" type="warning" class="mt-4">
          <strong>{{ t('warning') }}</strong>
          {{ t('sec6.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">7. {{ t('sec7.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec7.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">choco install git -y</pre>
            </v-card>
          </li>
          <li>
            {{ t('sec7.text.1') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">git --version</pre>
            </v-card>
          </li>
        </ol>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">8. {{ t('sec8.title') }}</v-card-title>
      <v-card-text>
        <ol>
          <li>
            {{ t('sec8.text.0') }}
            <v-card variant="outlined" class="pa-2 mb-2">
              <pre style="margin: 0">
vcpkg install libavif libheif libjxl libwebp
              </pre>
            </v-card>
          </li>
        </ol>
        <v-alert variant="tonal" type="info" class="mt-4">
          <strong>{{ t('notice') }}</strong>
          {{ t('sec8.notice') }}
        </v-alert>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import TableOfContents from '@/components/TableOfContents.vue';

const { t } = useI18n();
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
