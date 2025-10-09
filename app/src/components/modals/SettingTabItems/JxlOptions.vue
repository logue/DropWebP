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
    :hint="t('quality_hint', { min: 0.1, max: 15.0 })"
    :label="t('quality')"
    color="primary"
    max="15.0"
    min="0.1"
    persistent-hint
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
        :hint="t('use_container_hint')"
        :label="t('use_container')"
        color="primary"
        persistent-hint
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
    :hint="t('decoding_speed_hint', { min: 0, max: 4 })"
    :label="t('decoding_speed')"
    color="primary"
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
        persistent-hint
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
        :hint="t('color_encoding_hint')"
        :label="t('color_encoding')"
        item-title="text"
        item-value="value"
        persistent-hint
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
  lossless_hint: If the input file is JPEG, it will be lossless regardless of this setting. It works for most images, but in rare cases it may not be applied (issue of jpegxl-rs).
  speed: Encoding speed
  speed_hint: Default is 7, Squirrel. Lower values are faster but lower quality.
  quality: Quality ({min}-{max})
  quality_hint: Higher values mean higher quality. Default is 1, recommended values are ({min}-{max}).
  use_container: Use JPEG XL container format
  use_container_hint: Using the JPEG XL container format allows saving metadata such as JPEG reconstruction. However, even if there is no additional metadata, a few bytes are added to the encoded file for the container header.
  uses_original_profile: Use original color profile
  uses_original_profile_hint: If the input image has a color profile, it will be used for the encoded image. Otherwise, an internal fixed color profile is chosen (which should be smaller).
  decoding_speed: Decoding speed ({min}-{max}).
  decoding_speed_hint: Lower values mean higher quality.
  init_buffer_size: Output buffer
  init_buffer_size_hint: Initial size of output buffer (in kilobytes). Values less than 32KB are rounded up to 32KB.
  color_encoding: Color encoding method
  color_encoding_hint: If you don't know, it's recommended not to choose anything other than Srgb.
  reset_jxl_options: Reset JPEG XL settings
fr:
  lossless: Compression sans perte
  lossless_hint: Si le fichier d'entrée est un JPEG, il sera sans perte quelle que soit cette configuration. Cela fonctionne pour la plupart des images, mais dans de rares cas, cela peut ne pas être appliqué (problème de jpegxl-rs).
  speed: Vitesse d'encodage
  speed_hint: La valeur par défaut est 7, Écureuil. Des valeurs plus basses sont plus rapides mais de moindre qualité.
  quality: Qualité ({min}-{max})
  quality_hint: Des valeurs plus élevées signifient une meilleure qualité. La valeur par défaut est 1, les valeurs recommandées sont ({min}-{max}).
  use_container: Utiliser le format de conteneur JPEG XL
  use_container_hint: L'utilisation du format de conteneur JPEG XL permet de sauvegarder des métadonnées telles que la reconstruction JPEG. Cependant, même s'il n'y a pas de métadonnées supplémentaires, quelques octets sont ajoutés au fichier encodé pour l'en-tête du conteneur.
  uses_original_profile: Utiliser le profil de couleur original
  uses_original_profile_hint: Si l'image d'entrée possède un profil de couleur, il sera utilisé pour l'image encodée. Sinon, un profil de couleur fixe interne est choisi (ce qui devrait être plus petit).
  decoding_speed: Vitesse de décodage ({min}-{max}).
  decoding_speed_hint: Des valeurs plus basses signifient une meilleure qualité.
  init_buffer_size: Tampon de sortie
  init_buffer_size_hint: Taille initiale du tampon de sortie (en kilo-octets). Les valeurs inférieures à 32 Ko sont arrondies à 32 Ko.
  color_encoding: Méthode d'encodage des couleurs
  color_encoding_hint: Si vous ne savez pas, il est recommandé de ne choisir rien d'autre que Srgb.
  reset_jxl_options: Réinitialiser les paramètres JPEG XL
ja:
  lossless: ロスレス圧縮
  lossless_hint: JPEGが入力ファイルだった場合、ここの設定に関わらずロスレスになります。ほとんどの画像で機能しますが、稀に適用されないことがあります（jpegxl-rsの問題）。
  speed: エンコード速度
  speed_hint: デフォルトは7, Squirrelです。値が低いほど高速ですが品質が低くなります。
  quality: 品質（{min}～{max}）
  quality_hint: 値が高いほど高品質です。デフォルトは1で、推奨値は{min}～{max}です。
  use_container: JPEG XLコンテナ形式を使用
  use_container_hint: JPEG XLコンテナ形式を使用すると、JPEG再構成などのメタデータを保存できます。ただし、追加のメタデータがない場合でも、コンテナヘッダー用にエンコードされたファイルに数バイトが追加されます。
  uses_original_profile: 元のカラープロファイルを使用
  uses_original_profile_hint: 入力画像にカラープロファイルがある場合、それがエンコードされた画像に使用されます。そうでない場合は、内部の固定カラープロファイルが選択されます（これはより小さくなるはずです）。
  decoding_speed: デコード速度（{min}～{max}）
  decoding_speed_hint: 値が低いほど高品質
  init_buffer_size: 出力バッファ
  init_buffer_size_hint: 出力バッファの初期サイズ（キロバイト単位）32KB未満は32KBに切り上げられます。
  color_encoding: カラーエンコード方法
  color_encoding_hint: よくわからない場合は、Srgb以外にしないことをお勧めします。
  reset_jxl_options: JPEG XL設定をリセット
ko:
  lossless: 무손실 압축
  lossless_hint: 입력 파일이 JPEG인 경우 이 설정과 관계없이 무손실이 됩니다. 대부분의 이미지에서 작동하지만 드문 경우에 적용되지 않을 수 있습니다(jpegxl-rs의 문제).
  speed: 인코딩 속도
  speed_hint: 기본값은 7, Squirrel입니다. 값이 낮을수록 빠르지만 품질이 낮아집니다.
  quality: 품질 ({min}-{max})
  quality_hint: 값이 높을수록 품질이 높아집니다. 기본값은 1이며 권장 값은 ({min}-{max})입니다.
  use_container: JPEG XL 컨테이너 형식 사용
  use_container_hint: JPEG XL 컨테이너 형식을 사용하면 JPEG 재구성 등의 메타데이터를 저장할 수 있습니다. 그러나 추가 메타데이터가 없더라도 컨테이너 헤더를 위해 인코딩된 파일에 몇 바이트가 추가됩니다.
  uses_original_profile: 원본 색상 프로필 사용
  uses_original_profile_hint: 입력 이미지에 색상 프로필이 있는 경우 인코딩된 이미지에 사용됩니다. 그렇지 않으면 내부 고정 색상 프로필이 선택됩니다(더 작아야 함).
  decoding_speed: 디코딩 속도 ({min}-{max}).
  decoding_speed_hint: 값이 낮을수록 품질이 높아집니다.
  init_buffer_size: 출력 버퍼
  init_buffer_size_hint: 출력 버퍼의 초기 크기(킬로바이트 단위). 32KB 미만의 값은 32KB로 반올림됩니다.
  color_encoding: 색상 인코딩 방법
  color_encoding_hint: 잘 모르는 경우 Srgb 이외의 다른 항목을 선택하지 않는 것이 좋습니다.
  reset_jxl_options: JPEG XL 설정 재설정
zh-tw:
  lossless: 無損壓縮
  lossless_hint: 如果輸入文件是 JPEG，則無論此設置如何，都將是無損的。大多數圖像均可使用，但極少數情況下可能無法應用（jpegxl-rs 的問題）。
  speed: 編碼速度
  speed_hint: 默認為 7, Squirrel。值越低，速度越快，但質量越低。
  quality: 質量 ({min}-{max})
  quality_hint: 值越高，質量越高。默認值為 1，建議值為 ({min}-{max})。
  use_container: 使用 JPEG XL 容器格式
  use_container_hint: 使用 JPEG XL 容器格式可以保存 JPEG 重建等元數據。但是，即使沒有其他元數據，編碼文件也會為容器標頭添加幾個字節。
  uses_original_profile: 使用原始色彩配置文件
  uses_original_profile_hint: 如果輸入圖像具有色彩配置文件，則該配置文件將用於編碼圖像。否則，將選擇內部固定色彩配置文件（應該更小）。
  decoding_speed: 解碼速度 ({min}-{max})。
  decoding_speed_hint: 值越低，質量越高。
  init_buffer_size: 輸出緩衝區
  init_buffer_size_hint: 輸出緩衝區的初始大小（以千字節為單位）。小於 32KB 的值將四捨五入為 32KB。
  color_encoding: 色彩編碼方法
  color_encoding_hint: 如果您不知道，建議不要選擇 Srgb 以外的任何選項。
  reset_jxl_options: 重置 JPEG XL 設置
zh-cn:
  lossless: 无损压缩
  lossless_hint: 如果输入文件是 JPEG，则无论此设置如何，都会是无损的。大多数图像均可使用，但极少数情况下可能无法应用（jpegxl-rs 的问题）。
  speed: 编码速度
  speed_hint: 默认值为 7, Squirrel。值越低，速度越快，但质量越低。
  quality: 质量 ({min}-{max})
  quality_hint: 值越高，质量越高。默认值为 1，建议值为 ({min}-{max})。
  use_container: 使用 JPEG XL 容器格式
  use_container_hint: 使用 JPEG XL 容器格式可以保存 JPEG 重建等元数据。但是，即使没有其他元数据，编码文件也会为容器头添加几个字节。
  uses_original_profile: 使用原始色彩配置文件
  uses_original_profile_hint: 如果输入图像具有色彩配置文件，则该配置文件将用于编码图像。否则，将选择内部固定色彩配置文件（应该更小）。
  decoding_speed: 解码速度 ({min}-{max})。
  decoding_speed_hint: 值越低，质量越高。
  init_buffer_size: 输出缓冲区
  init_buffer_size_hint: 输出缓冲区的初始大小（以千字节为单位）。小于 32KB 的值将四舍五入为 32KB。
</i18n>
