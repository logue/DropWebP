<script setup lang="ts">
import { useSettingsStore } from '@/store';

const settingsStore = useSettingsStore();
</script>

<template>
  <v-card class="pa-4">
    <v-card-text>
      <v-row class="mb-4">
        <v-col cols="12" sm="6">
          <v-switch v-model="settingsStore.webpOptions.lossless" label="ロスレス" />
        </v-col>
        <v-col cols="12" sm="6">
          <v-slider
            v-model="settingsStore.webpOptions.quality"
            label="品質 (0-100)"
            type="number"
            :min="0"
            :max="100"
            step="1"
            :disabled="settingsStore.webpOptions.lossless"
          />
        </v-col>
      </v-row>
    </v-card-text>
    <v-card-actions>
      <v-btn @click="settingsStore.resetWebpOptions()">WebPオプションをリセット</v-btn>
    </v-card-actions>
  </v-card>
</template>
