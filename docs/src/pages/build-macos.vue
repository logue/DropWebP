<template>
  <TableOfContents />
  <div class="content-with-toc">
    <h2>macOS でのビルド環境構築</h2>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">Homebrew のインストール</v-card-title>
      <v-card-text>
        <p>まず、Homebrew をインストールします。</p>
        <v-card variant="outlined" class="pa-2 mb-2">
          <pre style="margin: 0">
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
          </pre>
        </v-card>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">asdf のインストール</v-card-title>
      <v-card-text>
        <p>asdf を Homebrew 経由でインストールします。</p>
        <v-card variant="outlined" class="pa-2 mb-2">
          <pre style="margin: 0">
brew install asdf
echo -e "\n. $(brew --prefix asdf)/libexec/asdf.sh" >> ~/.zshrc
source ~/.zshrc
          </pre>
        </v-card>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">Node.js のインストール</v-card-title>
      <v-card-text>
        <p>asdf を使って Node.js をインストールします。</p>
        <v-card variant="outlined" class="pa-2 mb-2">
          <pre style="margin: 0">
asdf plugin add nodejs
asdf install nodejs latest
asdf global nodejs latest
node --version
          </pre>
        </v-card>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">pnpm のインストール</v-card-title>
      <v-card-text>
        <p>asdf を使って pnpm をインストールします。</p>
        <v-card variant="outlined" class="pa-2 mb-2">
          <pre style="margin: 0">
asdf plugin add pnpm
asdf install pnpm latest
asdf global pnpm latest
pnpm --version
          </pre>
        </v-card>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">Rust のセットアップ</v-card-title>
      <v-card-text>
        <p>rustup を使って Rust をインストールします。</p>
        <v-card variant="outlined" class="pa-2 mb-2">
          <pre style="margin: 0">
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustc --version
cargo --version
          </pre>
        </v-card>
      </v-card-text>
    </v-card>

    <v-card tag="section" class="mb-6">
      <v-card-title tag="h3">環境確認</v-card-title>
      <v-card-text>
        <p>すべてのツールが正しくインストールされているか確認します。</p>
        <v-card variant="outlined" class="pa-2 mb-2">
          <pre style="margin: 0">
node --version
pnpm --version
rustc --version
cargo --version
          </pre>
        </v-card>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import TableOfContents from '@/components/TableOfContents.vue';
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
