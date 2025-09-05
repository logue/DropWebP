<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const settingsStore = useSettingsStore();
const { t } = useI18n();
</script>

<template>
  <v-switch
    v-model="settingsStore.commonOptions.ignoreJpeg"
    :label="t('ignore_jpeg')"
    color="primary"
    hide-details
    inline
  />
  <v-switch
    v-model="settingsStore.commonOptions.overwrite"
    :label="t('overwrite')"
    color="primary"
    disabled
    hide-details
    inline
  />
  <v-switch
    v-model="settingsStore.commonOptions.deleteOriginal"
    :label="t('delete_original')"
    color="primary"
    disabled
    hide-details
    inline
  />
  <v-switch
    v-model="settingsStore.commonOptions.recursive"
    :label="t('recursive')"
    color="primary"
    hide-details
    inline
  />
  <v-switch
    v-model="settingsStore.commonOptions.sameDirectory"
    :label="t('same_directory')"
    color="primary"
    hide-details
    inline
  />
  <v-text-field
    v-model="settingsStore.commonOptions.outputPath"
    :disabled="settingsStore.commonOptions.sameDirectory"
    :label="t('output_path')"
    readonly
  >
    <template #append>
      <v-btn icon="mdi-folder-open" variant="plain" @click="settingsStore.browseOutputPath()" />
    </template>
  </v-text-field>
  <v-btn @click="settingsStore.resetCommonOptions()">{{ t('reset') }}</v-btn>
</template>

<i18n lang="yaml">
en:
  ignore_jpeg: Ignore JPEG
  overwrite: Overwrite
  same_directory: Output Same Directory
  delete_original: Delete Original
  recursive: Include Subdirectories
  output_path: Default output path
  browse: Browse
  reset: Reset common options
ja:
  ignore_jpeg: JPEGを無視
  overwrite: 上書きする
  same_directory: 同じディレクトリに出力
  delete_original: 元ファイルを削除する
  recursive: サブディレクトリを含める
  output_path: デフォルトの出力先のパス
  browse: ブラウズ
  reset: 共通オプションをリセット
kr:
  ignore_jpeg: JPEG 무시
  overwrite: 덮어쓰기
  same_directory: 동일 디렉토리에 출력
  delete_original: 원본 파일 삭제
  recursive: 하위 디렉토리 포함
  output_path: 기본 출력 경로
  browse: 찾아보기
  reset: 공통 옵션 재설정
zh:
  ignore_jpeg: 忽略 JPEG
  overwrite: 覆蓋
  same_directory: 輸出到相同目錄
  delete_original: 刪除原文件
  recursive: 包含子目錄
  output_path: 默認輸出路徑
  browse: 瀏覽
  reset: 重置通用選項
</i18n>
