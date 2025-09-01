<script setup lang="ts">
import { useGlobalStore } from '@/store';
import { computed, nextTick, ref } from 'vue';

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';

/** Global Store */
const globalStore = useGlobalStore();

const inputPath = ref('');
const outputPath = ref('');
const isLossless = ref(true);
const quality = ref(80);
const progress = computed({
  get: () => globalStore.progress,
  set: value => globalStore.setProgress(value)
});

async function browseInput() {
  inputPath.value = (await open({
    multiple: false,
    directory: true
  })) as string;
}

async function browseOutput() {
  outputPath.value = (await open({
    multiple: false,
    directory: true
  })) as string;
}

const startConversion = async () => {
  if (!inputPath.value) {
    globalStore.setMessage('Input directory is not selected');
    return;
  }
  if (!outputPath.value) {
    globalStore.setMessage('Output directory is not selected');
    return;
  }
  const files = (await invoke('list_full_paths', { path: inputPath.value })) as string[];
  if (files.length === 0) {
    globalStore.setMessage('No image files found in the specified directory');
    return;
  }

  convert(files, outputPath.value);
};

listen('tauri://drag-drop', async e => {
  const paths = (e.payload as { paths: string[] }).paths;
  await convert(paths);
});

const convertByDialog = async () => {
  const files = await open({
    multiple: true,
    directory: false
  });
  if (!files || files.length === 0) return;
  await convert(files);
};

/**
 * 変換処理
 * @param files 変換するファイルのパス一覧
 */
const convert = async (files: string[], output: string | null = null) => {
  if (!files || files.length === 0) return;
  if (progress.value) {
    globalStore.setMessage('現在の処理が終了するまでお待ちください');
    return;
  }
  progress.value = 0;
  let i;
  try {
    for (i = 0; i < files.length; i++) {
      await invoke('convert_image', {
        input: files[i],
        output,
        quality: isLossless.value ? 100 : quality.value
      });
      progress.value = Math.floor(((i + 1) / files.length) * 100);
      await nextTick(); // UI更新を挟む
    }
  } catch (error) {
    globalStore.setMessage(error as string);
  } finally {
    progress.value = null; // 完了
  }
  globalStore.setMessage(`${i}件の画像の変換が完了しました`);
};
</script>

<template>
  <v-container>
    <v-text-field
      v-model="inputPath"
      prepend-icon="mdi-import"
      label="Input Directory"
      readonly
      webkitdirectory
      multiple
      clearable
    >
      <template #append>
        <v-btn icon="mdi-folder-open" variant="plain" @click="browseInput()" />
      </template>
    </v-text-field>
    <v-text-field
      v-model="outputPath"
      prepend-icon="mdi-export"
      label="Output Directory"
      readonly
      webkitdirectory
      multiple
      clearable
    >
      <template #append>
        <v-btn icon="mdi-folder-open" variant="plain" @click="browseOutput()" />
      </template>
    </v-text-field>
    <v-switch v-model="isLossless" label="Lossless" color="primary" />
    <v-slider
      v-model="quality"
      color="primary"
      :max="100"
      :min="1"
      :step="1"
      :disabled="isLossless"
      label="Quality"
    >
      <template #append>
        <span>{{ quality }}</span>
      </template>
    </v-slider>
    <v-btn prepend-icon="mdi-arrow-collapse-vertical" @click="startConversion">
      Start Conversion
    </v-btn>
    <v-btn prepend-icon="mdi-arrow-collapse-vertical" @click="convertByDialog">Browse</v-btn>
  </v-container>
</template>
