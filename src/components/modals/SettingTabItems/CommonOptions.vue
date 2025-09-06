<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const settingsStore = useSettingsStore();
const { t } = useI18n();
</script>

<template>
  <v-switch
    v-model="settingsStore.commonOptions.ignoreJpeg"
    :hint="t('ignore_jpeg_hint')"
    :label="t('ignore_jpeg')"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.commonOptions.overwrite"
    :label="t('overwrite')"
    color="primary"
    hide-details
  />
  <v-switch
    v-model="settingsStore.commonOptions.deleteOriginal"
    :hint="t('delete_original_hint')"
    :label="t('delete_original')"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.commonOptions.recursive"
    :label="t('recursive')"
    color="primary"
    hide-details
  />
  <v-switch
    v-model="settingsStore.commonOptions.sameDirectory"
    :label="t('same_directory')"
    color="primary"
    hide-details
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
  <v-btn prepend-icon="mdi-rotate-left" variant="text" @click="settingsStore.resetCommonOptions()">
    {{ t('reset') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  ignore_jpeg: Ignore JPEG
  ignore_jpeg_hint: JPEG images are already compressed, so converting them may actually increase their size. This option allows you to ignore JPEG images.
  overwrite: Overwrite
  same_directory: Output Same Directory
  delete_original: Delete Original
  delete_original_hint: Be careful, as deleting the original file cannot be undone.
  recursive: Include Subdirectories
  output_path: Default output path
  browse: Browse
  reset: Reset common options
ja:
  ignore_jpeg: JPEGを無視
  ignore_jpeg_hint: JPEG画像はもともと圧縮されているため、変換すると容量がかえって増えてしまうことがあります。このオプションでJPEG画像を無視できます。
  overwrite: 上書きする
  same_directory: 同じディレクトリに出力
  delete_original: 元ファイルを削除する
  delete_original_hint: 元ファイルを削除すると元に戻せなくなるので注意してください。
  recursive: サブディレクトリを含める
  output_path: デフォルトの出力先のパス
  browse: ブラウズ
  reset: 共通オプションをリセット
kr:
  ignore_jpeg: JPEG 무시
  ignore_jpeg_hint: JPEG 이미지는 원래 압축되어 있기 때문에 변환하면 용량이 오히려 늘어날 수 있습니다. 이 옵션으로 JPEG 이미지를 무시할 수 있습니다.
  overwrite: 덮어쓰기
  same_directory: 동일 디렉토리에 출력
  delete_original: 원본 파일 삭제
  delete_original_hint: 원본 파일을 삭제하면 복구할 수 없으니 주의하세요.
  recursive: 하위 디렉토리 포함
  output_path: 기본 출력 경로
  browse: 찾아보기
  reset: 공통 옵션 재설정
zh:
  ignore_jpeg: 忽略 JPEG
  ignore_jpeg_hint: JPEG 图像已经是压缩过的，因此转换后文件大小可能会增加。此选项允许您忽略 JPEG 图像。
  overwrite: 覆蓋
  same_directory: 輸出到相同目錄
  delete_original: 刪除原文件
  delete_original_hint: 刪除原文件後將無法恢復，請謹慎操作。
  recursive: 包含子目錄
  output_path: 默認輸出路徑
  browse: 瀏覽
  reset: 重置通用選項
</i18n>
