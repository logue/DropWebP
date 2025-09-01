<script setup lang="ts">
import ProgressDialog from './ProgressDialog.vue';

import { useWebPConverter } from '@/composables/useWebPConverter';

const {
  isLossless,
  quality,
  recursive,
  ignoreJpeg,
  dialog,
  inProgress,
  currentFile,
  progress,
  convertByDialog,
  convertByDirDialog
} = useWebPConverter();
</script>

<template>
  <v-container>
    <h2>Drag and drop the image you want to compress.</h2>
    <v-row>
      <v-col>
        <v-switch v-model="recursive" label="Include subdirectories" color="primary" hide-details />
      </v-col>
      <v-col>
        <v-switch v-model="ignoreJpeg" label="Ignore Jpeg" color="primary" hide-details />
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="3">
        <v-switch v-model="isLossless" label="Lossless" color="primary" hide-details />
      </v-col>
      <v-col cols="9">
        <v-slider
          v-model="quality"
          :disabled="isLossless"
          :max="100"
          :min="1"
          :step="1"
          color="primary"
          hide-details
          label="Quality"
        >
          <template #append>
            <span>{{ quality }}</span>
          </template>
        </v-slider>
      </v-col>
    </v-row>

    <v-btn prepend-icon="mdi-file-multiple" class="mr-2" @click="convertByDialog">
      Select Files
    </v-btn>
    <v-btn prepend-icon="mdi-folder-open" @click="convertByDirDialog">Select Folder</v-btn>
  </v-container>
  <progress-dialog
    v-model:dialog="dialog"
    v-model:current-file="currentFile"
    v-model:progress="progress"
    v-model:in-progress="inProgress"
    title="Compressing..."
  />
</template>
