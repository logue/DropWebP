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
    color="red"
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
  <v-btn
    color="warning"
    prepend-icon="mdi-rotate-left"
    variant="text"
    @click="settingsStore.resetCommonOptions()"
  >
    {{ t('reset_common') }}
  </v-btn>
  <v-btn color="red" prepend-icon="mdi-rotate-left" variant="text" @click="settingsStore.reset()">
    {{ t('reset_all') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  ignore_jpeg: Ignore JPEG
  ignore_jpeg_hint: JPEG images are already compressed, so converting them (except to JPEG XL) may increase file size. This option allows you to ignore JPEG images.
  overwrite: Overwrite
  same_directory: Output Same Directory
  delete_original: Delete Original
  delete_original_hint: ⚠️Be careful, as deleting the original file cannot be undone.
  recursive: Include Subdirectories
  output_path: Default output path
  browse: Browse
  reset_all: Reset All
  reset_common: Reset common options
ja:
  ignore_jpeg: JPEGを無視
  ignore_jpeg_hint: JPEG画像はもともと圧縮されているため、JPEG XL以外の場合、変換すると容量がかえって増えてしまうことがあります。このオプションでJPEG画像を無視できます。
  overwrite: 上書きする
  same_directory: 同じディレクトリに出力
  delete_original: 元ファイルを削除する
  delete_original_hint: ⚠️元ファイルを削除すると元に戻せなくなるので注意してください。
  recursive: サブディレクトリを含める
  output_path: デフォルトの出力先のパス
  browse: ブラウズ
  reset_all: 全てをリセット
  reset_common: 共通オプションをリセット
kr:
  ignore_jpeg: JPEG 무시
  ignore_jpeg_hint: JPEG 이미지는 이미 압축되어 있으므로 JPEG XL을 제외한 다른 형식으로 변환하면 파일 크기가 오히려 커질 수 있습니다. 이 옵션을 사용하면 JPEG 이미지를 무시할 수 있습니다.
  overwrite: 덮어쓰기
  same_directory: 동일 디렉토리에 출력
  delete_original: 원본 파일 삭제
  delete_original_hint: ⚠️원본 파일을 삭제하면 복구할 수 없으니 주의하세요.
  recursive: 하위 디렉토리 포함
  output_path: 기본 출력 경로
  browse: 찾아보기
  reset_all: 모두 재설정
  reset_common: 공통 옵션 재설정
zh:
  ignore_jpeg: 忽略 JPEG
  ignore_jpeg_hint: JPEG 图像已经是压缩格式，除非转换为 JPEG XL，否则转换可能会导致文件大小增加。启用此选项可忽略 JPEG 图像。
  overwrite: 覆蓋
  same_directory: 輸出到相同目錄
  delete_original: 刪除原文件
  delete_original_hint: ⚠️刪除原文件後將無法恢復，請謹慎操作。
  recursive: 包含子目錄
  output_path: 默認輸出路徑
  browse: 瀏覽
  reset_all: 全部重置
  reset_common: 重置通用選項
</i18n>
