<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const settingsStore = useSettingsStore();
const { t } = useI18n();
</script>

<template>
  <v-col>
    <v-row>
      <v-switch
        v-model="settingsStore.commonOptions.sound"
        :label="t('play_sound')"
        color="primary"
      />
    </v-row>
    <v-row>
      <v-slider
        v-model="settingsStore.commonOptions.volume"
        :label="t('volume')"
        :append-icon="
          settingsStore.commonOptions.volume === 0 ? 'mdi-volume-off' : 'mdi-volume-high'
        "
        :disabled="!settingsStore.commonOptions.sound"
        :min="0"
        :max="1.0"
        :step="0.1"
        class="mt-0"
      />
    </v-row>
  </v-col>
  <v-switch
    v-model="settingsStore.commonOptions.ignoreJpeg"
    :hint="t('notify_hint')"
    :label="t('notify')"
    color="primary"
    persistent-hint
  />
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
  play_sound: Play sound when done
  volume: Sound Volume
  notify: Notify in Desktop
  notify_hint: You may not receive notifications if you have disabled them in your system settings.
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
fr:
  play_sound: Lire un son à la fin
  volume: Volume du son
  notify: Notification sur le bureau
  notify_hint: Vous risquez de ne pas recevoir de notifications si vous les avez désactivées dans les paramètres de votre système.
  ignore_jpeg: Ignorer le JPEG
  ignore_jpeg_hint: Les images JPEG sont déjà compressées, donc les convertir (sauf en JPEG XL) peut augmenter la taille du fichier. Cette option vous permet d'ignorer les images JPEG.
  overwrite: Écraser
  same_directory: Sortir dans le même répertoire
  delete_original: Supprimer l'original
  delete_original_hint: ⚠️Faites attention, car la suppression du fichier original ne peut pas être annulée.
  recursive: Inclure les sous-répertoires
  output_path: Chemin de sortie par défaut
  browse: Parcourir
  reset_all: Réinitialiser tout
  reset_common: Réinitialiser les options communes
ja:
  play_sound: 完了時にサウンドを再生
  volume: サウンドの音量
  notify: デスクトップで通知
  notify_hint: システム設定で通知を無効にしている場合、通知が届かないことがあります。
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
ko:
  play_sound: 완료 시 소리 재생
  volume: 소리 크기
  notify: 데스크탑 알림
  notify_hint: 시스템 설정에서 알림을 비활성화한 경우 알림을 받지 못할 수 있습니다.
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
zh-tw:
  play_sound: 完成時播放聲音
  volume: 聲音音量
  notify: 桌面通知
  notify_hint: 如果您在系統設置中禁用了通知，則可能無法收到通知。
  ignore_jpeg: 忽略 JPEG
  ignore_jpeg_hint: JPEG 圖像已經過壓縮，因此將它們轉換為除 JPEG XL 之外的格式可能會增加文件大小。此選項允許您忽略 JPEG 圖像。
  overwrite: 覆蓋
  same_directory: 輸出到相同目錄
  delete_original: 刪除原始文件
  delete_original_hint: ⚠️請小心，因為刪除原始文件後無法恢復。
  recursive: 包括子目錄
  output_path: 預設輸出路徑
  browse: 瀏覽
  reset_all: 重置所有設定
  reset_common: 重置常用選項
zh-cn:
  play_sound: 完成时播放声音
  volume: 声音音量
  notify: 桌面通知
  notify_hint: 如果您在系统设置中禁用了通知，则可能无法收到通知。
  ignore_jpeg: 忽略 JPEG
  ignore_jpeg_hint: JPEG 图像已经过压缩，因此将它们转换为除 JPEG XL 之外的格式可能会增加文件大小。此选项允许您忽略 JPEG 图像。
  overwrite: 覆盖
  same_directory: 输出到相同目录
  delete_original: 删除原始文件
  delete_original_hint: ⚠️请小心，因为删除原始文件后无法恢复。
  recursive: 包括子目录
  output_path: 默认输出路径
  browse: 浏览
  reset_all: 重置所有设置
  reset_common: 重置常用选项
</i18n>
