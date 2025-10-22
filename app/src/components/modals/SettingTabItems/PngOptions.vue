<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-switch
    v-model="settingsStore.pngOptions.embedIccProfile"
    :hint="t('embed_icc_profile_hint')"
    :label="t('embed_icc_profile')"
    color="primary"
    persistent-hint
  />
  <v-number-input
    v-model="settingsStore.pngOptions.zopfliIterations"
    :label="t('zopfli_iterations')"
    :hint="t('zopfli_iterations_hint', { min: 15, max: 100 })"
    :min="15"
    :max="100"
    color="primary"
    persistent-hint
  />
</template>

<i18n lang="yaml">
en:
  embed_icc_profile: Embed ICC Profile
  embed_icc_profile_hint: If the image contains an ICC profile, it will be embedded in the output image.
  zopfli_iterations: Zopfli Iterations
  zopfli_iterations_hint: Specify the number of iterations for Zopfli compression. It can be set in the range of {min} to {max}. A higher value improves compression ratio but increases processing time.
fr:
  embed_icc_profile: Intégrer le profil ICC
  embed_icc_profile_hint: Si l'image contient un profil ICC, il sera intégré dans l'image de sortie.
  zopfli_iterations: Itérations Zopfli
  zopfli_iterations_hint: Spécifiez le nombre d'itérations pour la compression Zopfli. Il peut être défini dans la plage de {min} à {max}. Une valeur plus élevée améliore le ratio de compression mais augmente le temps de traitement.
ja:
  embed_icc_profile: ICCプロファイルを埋め込む
  embed_icc_profile_hint: 画像にICCプロファイルが含まれている場合、出力画像に埋め込みます。
  zopfli_iterations: Zopfliの反復回数
  zopfli_iterations_hint: Zopfli圧縮の反復回数を指定します。{min}〜{max}の範囲で設定可能です。値が大きいほど圧縮率が向上しますが、処理時間も長くなります。
  interlace: インターレース設定
  interlace_none: インターレスなし
  interlace_none_hint: デフォルト。ファイルサイズ最小
  interlace_adam7: Adam7インターレース
  interlace_adam7_hint: 画像のプレビューを早く表示できますが、ファイルサイズが大きくなります。
  bit_depth_reduction: ビット深度の削減
  bit_depth_reduction_hint: 可能な場合、ビット深度を削減してファイルサイズを小さくします。
  color_type_reduction: カラーモードの削減
  color_type_reduction_hint: 可能な場合、カラーモードを削減してファイルサイズを小さくします。
  palette_reduction: パレットの削減
  palette_reduction_hint: 可能な場合、パレットを使用してファイルサイズを小さくします。
  fast_evaluation: 高速評価モード
  fast_evaluation_hint: 高速評価モードを有効にすると、圧縮処理が高速化されますが、圧縮率が低下する可能性があります。
ko:
  embed_icc_profile: ICC 프로필 포함
  embed_icc_profile_hint: 이미지에 ICC 프로필이 포함된 경우 출력 이미지에 포함합니다.
  zopfli_iterations: Zopfli 반복 횟수
  zopfli_iterations_hint: Zopfli 압축의 반복 횟수를 지정합니다. {min}~{max} 범위 내에서 설정할 수 있습니다. 값이 높을수록 압축률이 향상되지만 처리 시간이 길어집니다.
zh-tw:
  embed_icc_profile: 嵌入 ICC 配置文件
  embed_icc_profile_hint: 如果圖像包含 ICC 配置文件，則將其嵌入輸出圖像中。
  zopfli_iterations: Zopfli 迭代次數
  zopfli_iterations_hint: 指定 Zopfli 壓縮的迭代次數。可以在 {min} 到 {max} 的範圍內設置。值越高，壓縮率越高，但處理時間也會增加。
zh-cn:
  embed_icc_profile: 嵌入 ICC 配置文件
  embed_icc_profile_hint: 如果图像包含 ICC 配置文件，则将其嵌入输出图像中。
  zopfli_iterations: Zopfli 迭代次数
  zopfli_iterations_hint: 指定 Zopfli 压缩的迭代次数。可以在 {min} 到 {max} 的范围内设置。值越高，压缩率越高，但处理时间也会增加。
</i18n>
