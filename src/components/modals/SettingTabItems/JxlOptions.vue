<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { EncoderSpeed, ColorEncoding } from '@/types/JxlTypes'; // Adjust the import path as needed

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-switch
    v-model="settingsStore.jxlOptions.lossless"
    :hint="t('lossless_hint')"
    :label="t('lossless')"
    color="primary"
    persistent-hint
  />
  <v-slider
    v-model="settingsStore.jxlOptions.quality"
    :disabled="settingsStore.jxlOptions.lossless"
    :hint="t('quality_hint')"
    :label="t('quality')"
    color="primary"
    max="15.0"
    min="0.1"
    step="0.1"
    thumb-label="always"
    type="number"
  />
  <v-select
    v-model="settingsStore.jxlOptions.speed"
    :items="[
      { text: '1, Lightning', value: EncoderSpeed.Lightning },
      { text: '2, Thunder', value: EncoderSpeed.Thunder },
      { text: '3, Falcon', value: EncoderSpeed.Falcon },
      { text: '4, Cheetah', value: EncoderSpeed.Cheetah },
      { text: '5, Hare', value: EncoderSpeed.Hare },
      { text: '6, Wombat', value: EncoderSpeed.Wombat },
      { text: '7, Squirrel', value: EncoderSpeed.Squirrel },
      { text: '8, Tortoise', value: EncoderSpeed.Tortoise },
      { text: '9, Kitten', value: EncoderSpeed.Kitten },
      { text: '10, Glacier', value: EncoderSpeed.Glacier }
    ]"
    :hint="t('speed_hint')"
    :label="t('speed')"
    item-title="text"
    item-value="value"
    persistent-hint
  />
  <v-row>
    <v-col>
      <v-switch
        v-model="settingsStore.jxlOptions.useContainer"
        :label="t('use_container')"
        color="primary"
        hide-details
      />
    </v-col>
    <v-col>
      <v-switch
        v-model="settingsStore.jxlOptions.usesOriginalProfile"
        :disabled="settingsStore.jxlOptions.lossless"
        :hint="t('uses_original_profile_hint')"
        :label="t('uses_original_profile')"
        color="primary"
        persistent-hint
      />
    </v-col>
  </v-row>
  <v-slider
    v-model="settingsStore.jxlOptions.decodingSpeed"
    :hint="t('decoding_speed_hint')"
    :label="t('decoding_speed')"
    color="primary"
    hide-details
    max="4"
    min="0"
    persistent-hint
    step="1"
    thumb-label="always"
    type="number"
  />
  <v-row>
    <v-col>
      <v-number-input
        v-model="settingsStore.jxlOptions.initBufferSize"
        :hint="t('init_buffer_size_hint')"
        :label="t('init_buffer_size')"
        :min="32"
        :step="32"
        clearable
        type="number"
      />
    </v-col>
    <v-col>
      <v-select
        v-model="settingsStore.jxlOptions.colorEncoding"
        :items="[
          { text: 'Srgb', value: ColorEncoding.Srgb },
          { text: 'LinearSrgb', value: ColorEncoding.LinearSrgb },
          { text: 'SrgbLuma', value: ColorEncoding.SrgbLuma },
          { text: 'LinearSrgbLuma', value: ColorEncoding.LinearSrgbLuma }
        ]"
        :label="t('color_encoding')"
        item-title="text"
        item-value="value"
      />
    </v-col>
  </v-row>
  <v-btn
    color="warning"
    prepend-icon="mdi-rotate-left"
    variant="text"
    @click="settingsStore.resetJxlOptions()"
  >
    {{ t('reset_jxl_options') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  lossless: Lossless compression
  lossless_hint: If the input file is JPEG, it will be lossless regardless of this setting.
  speed: Encoding speed
  speed_hint: Default is Squirrel
  quality: Quality (0.1-15.0)
  quality_hint: Higher values mean higher quality. Default is 1, recommended values are 0.5-3.0.
  use_container: Use JPEG XL container format
  uses_original_profile: Use original color profile
  uses_original_profile_hint: Always used when lossless is enabled.
  decoding_speed: Decoding speed (0-4).
  decoding_speed_hint: Lower values mean higher quality.
  init_buffer_size: Output buffer
  init_buffer_size_hint: Initial size of output buffer (in bytes). Values less than 32 are rounded up to 32kb.
  color_encoding: Color encoding method
  reset_jxl_options: Reset JPEG XL settings
ja:
  lossless: ロスレス圧縮
  lossless_hint: JPEGが入力ファイルだった場合、ここの設定に関わらずロスレスになります。
  speed: エンコード速度
  speed_hint: デフォルトはSquirrelです
  quality: 品質（0.1〜15.0）
  quality_hint: 値が高いほど高品質です。デフォルトは1で、推奨値は0.5〜3.0です。
  use_container: JPEG XLコンテナ形式を使用
  uses_original_profile: 元のカラープロファイルを使用
  uses_original_profile_hint: ロスレス時は常に使用になります。
  decoding_speed: デコード速度（0~4）
  decoding_speed_hint: 値が低いほど高品質
  init_buffer_size: 出力バッファ
  init_buffer_size_hint: 出力バッファの初期サイズ（バイト単位）32未満は32kbに切り上げられます。
  color_encoding: カラーエンコード方法（よくわからない場合は、Srgb以外にしないこと）
  reset_jxl_options: JPEG XL設定をリセット
ko:
  lossless: 무손실 압축
  lossless_hint: 입력 파일이 JPEG인 경우 이 설정과 관계없이 무손실이 됩니다.
  speed: 인코딩 속도
  speed_hint: 기본값은 Squirrel입니다.
  quality: 품질(0.1~15.0)
  quality_hint: 값이 높을수록 품질이 높아집니다. 기본값은 1이며 권장 값은 0.5~3.0입니다.
  use_container: JPEG XL 컨테이너 형식 사용
  uses_original_profile: 원래 컬러 프로파일 사용
  uses_original_profile_hint: 무손실시 항상 사용됩니다.
  decoding_speed: 디코딩 속도(0~4).
  decoding_speed_hint: 값이 낮을수록 고품질
  init_buffer_size: 출력 버퍼
  init_buffer_size_hint: 출력 버퍼의 초기 크기 (바이트) 32 미만은 32kb로 반올림됩니다.
  color_encoding: 컬러 인코딩 방법
  reset_jxl_options: JPEG XL 설정 재설정
zh:
  lossless: 無損壓縮
  lossless_hint: 如果輸入文件是 JPEG，則無論此設置如何，都將是無損的。
  speed: 編碼速度
  speed_hint: 默認為 Squirrel
  quality: 質量 (0.1-15.0)
  quality_hint: 值越高，質量越高。默認值為 1，建議值為 0.5-3.0。
  use_container: 使用 JPEG XL 容器格式
  uses_original_profile: 使用原始顏色設定檔
  uses_original_profile_hint: 啟用無損壓縮時一律使用。
  decoding_speed: 解碼速度 (0-4)
  decoding_speed_hint: 數值越低，品質越高。
  init_buffer_size: 輸出緩衝區
  init_buffer_size_hint: 輸出緩衝區的初始大小（以位元組為單位）。小於 32 的值將向上舍入為 32kb。
  color_encoding: 顏色編碼法
  reset_jxl_options: 重設 JPEG XL 設定
</i18n>
