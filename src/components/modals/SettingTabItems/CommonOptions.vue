<script setup lang="ts">
import { useSettingsStore } from '@/store';

const settingsStore = useSettingsStore();
</script>

<template>
  <v-card class="pa-4">
    <v-card-text>
      <v-row class="mb-4">
        <v-col cols="12" sm="6">
          <v-switch
            v-model="settingsStore.commonOptions.recursive"
            label="サブディレクトリを含める"
          />
        </v-col>
        <v-col cols="12" sm="6">
          <v-text-field
            v-model="settingsStore.commonOptions.outputPath"
            label="出力先フォルダ"
            readonly
          >
            <template #append>
              <v-btn icon="mdi-folder" @click="settingsStore.browseOutputPath()" />
            </template>
          </v-text-field>
        </v-col>
        <v-col cols="12" sm="6">
          <v-text-field
            v-model="settingsStore.commonOptions.outputPath"
            label="出力先フォルダ"
            readonly
          >
            <template #append>
              <v-btn icon @click="settingsStore.browseOutputPath()">
                <v-icon>mdi-folder</v-icon>
              </v-btn>
            </template>
          </v-text-field>
        </v-col>
      </v-row>
    </v-card-text>
    <v-card-actions>
      <v-btn @click="settingsStore.resetCommonOptions()">共通オプションをリセット</v-btn>
    </v-card-actions>
  </v-card>
</template>
