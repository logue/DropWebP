<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { EncoderSpeed, ColorEncoding } from '@/types/JxlTypes'; // Adjust the import path as needed

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-card flat :title="t('title')">
    <v-card-text>
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
        :hint="t('quality_hint', { min: 0.5, max: 3.0 })"
        :label="t('quality', { min: 0.1, max: 15.0 })"
        color="primary"
        max="15.0"
        min="0.1"
        persistent-hint
        step="0.1"
        thumb-label
        type="number"
      >
        <template #append>
          <v-text-field v-model="settingsStore.jxlOptions.quality" readonly variant="underlined" />
        </template>
      </v-slider>
      <v-select
        v-model="settingsStore.jxlOptions.speed"
        :items="[
          { text: t('speed_items[0]'), value: EncoderSpeed.Lightning },
          { text: t('speed_items[1]'), value: EncoderSpeed.Thunder },
          { text: t('speed_items[2]'), value: EncoderSpeed.Falcon },
          { text: t('speed_items[3]'), value: EncoderSpeed.Cheetah },
          { text: t('speed_items[4]'), value: EncoderSpeed.Hare },
          { text: t('speed_items[5]'), value: EncoderSpeed.Wombat },
          { text: t('speed_items[6]'), value: EncoderSpeed.Squirrel },
          { text: t('speed_items[7]'), value: EncoderSpeed.Tortoise },
          { text: t('speed_items[8]'), value: EncoderSpeed.Kitten },
          { text: t('speed_items[9]'), value: EncoderSpeed.Glacier }
        ]"
        :hint="t('speed_hint')"
        :label="t('speed')"
        item-title="text"
        item-value="value"
        persistent-hint
      />
      <v-switch
        v-model="settingsStore.jxlOptions.useContainer"
        :hint="t('use_container_hint')"
        :label="t('use_container')"
        color="primary"
        persistent-hint
      />
      <v-switch
        v-model="settingsStore.jxlOptions.usesOriginalProfile"
        :disabled="settingsStore.jxlOptions.lossless"
        :hint="t('uses_original_profile_hint')"
        :label="t('uses_original_profile')"
        color="primary"
        persistent-hint
      />
      <v-select
        v-model="settingsStore.jxlOptions.colorEncoding"
        :items="[
          { text: t('color_encoding_items[0]'), value: ColorEncoding.Srgb },
          { text: t('color_encoding_items[1]'), value: ColorEncoding.LinearSrgb },
          { text: t('color_encoding_items[2]'), value: ColorEncoding.SrgbLuma },
          { text: t('color_encoding_items[3]'), value: ColorEncoding.LinearSrgbLuma }
        ]"
        :hint="t('color_encoding_hint')"
        :label="t('color_encoding')"
        item-title="text"
        item-value="value"
        persistent-hint
      />
      <v-slider
        v-model="settingsStore.jxlOptions.decodingSpeed"
        :hint="t('decoding_speed_hint')"
        :label="t('decoding_speed', { min: 0, max: 4 })"
        color="primary"
        max="4"
        min="0"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.jxlOptions.decodingSpeed"
            readonly
            variant="underlined"
          />
        </template>
      </v-slider>
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
    </v-card-text>
    <v-card-actions>
      <v-btn
        :text="t('reset_jxl_options')"
        color="warning"
        prepend-icon="mdi-rotate-left"
        variant="tonal"
        @click="settingsStore.resetJxlOptions()"
      />
    </v-card-actions>
  </v-card>
</template>

<i18n lang="yaml">
en:
  title: JPEG XL Options
  lossless: Lossless compression
  lossless_hint: This works for most images, but in rare cases, it may not be applied.
  speed: Encoding speed
  speed_hint: Default is 7, Squirrel. Lower values are faster but lower quality.
  speed_items:
    - 1, Lightning (Fastest speed, lowest quality)
    - 2, Thunder (Very fast, low quality)
    - 3, Falcon (Fast, slightly low quality)
    - 4, Cheetah (Balanced speed and quality)
    - 5, Hare (Slightly slow, good quality)
    - 6, Wombat (Slow, very good quality)
    - 7, Squirrel (Very slow, highest quality)
    - 8, Kitten (Best quality, very slow)
    - 9, Tortoise (Best quality, very slow)
    - 10, Glacier (Best quality, very slow, for archival use)
  quality: Quality / Distance ({min}-{max})
  quality_hint: Distance parameter. LOWER values mean HIGHER quality. 0.0=lossless, 1.0=visually lossless (recommended), 3.0=high quality. Recommended range is ({min}-{max}).
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
  color_encoding_items:
    - Srgb (Default, Standard sRGB color space)
    - LinearSrgb (Linear sRGB color space)
    - SrgbLuma (sRGB color space with luminance information)
    - LinearSrgbLuma (Linear sRGB color space with luminance information)
  reset_jxl_options: Reset JPEG XL settings
fr:
  title: Options JPEG XL
  lossless: Compression sans perte
  lossless_hint: Cela fonctionne pour la plupart des images, mais dans de rares cas, cela peut ne pas être appliqué.
  speed: Vitesse d'encodage
  speed_hint: La valeur par défaut est 7, Écureuil. Des valeurs plus basses sont plus rapides mais de moindre qualité.
  speed_items:
    - 1, Lightning (Vitesse la plus rapide, qualité la plus basse)
    - 2, Thunder (Très rapide, qualité faible)
    - 3, Falcon (Rapide, qualité légèrement faible)
    - 4, Cheetah (Équilibre entre vitesse et qualité)
    - 5, Hare (Légèrement lent, bonne qualité)
    - 6, Wombat (Lent, très bonne qualité)
    - 7, Squirrel (Très lent, qualité la plus élevée)
    - 8, Kitten (Meilleure qualité, très lent)
    - 9, Tortoise (Meilleure qualité, très lent)
    - 10, Glacier (Meilleure qualité, très lent, pour usage archivistique)
  quality: Qualité / Distance ({min}-{max})
  quality_hint: Paramètre de distance. Des valeurs PLUS BASSES signifient une qualité PLUS ÉLEVÉE. 0.0=sans perte, 1.0=visuellement sans perte (recommandé), 3.0=haute qualité. La plage recommandée est ({min}-{max}).
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
  color_encoding_items:
    - Srgb (Par défaut, espace colorimétrique sRGB standard)
    - LinearSrgb (Espace colorimétrique sRGB linéaire)
    - SrgbLuma (Espace colorimétrique sRGB avec information de luminance)
    - LinearSrgbLuma (Espace colorimétrique sRGB linéaire avec information de luminance)
  reset_jxl_options: Réinitialiser les paramètres JPEG XL
ja:
  title: JPEG XL 設定
  lossless: ロスレス圧縮
  lossless_hint: これはほとんどの画像で機能しますが、稀に適用されないことがあります。
  speed: エンコード速度
  speed_hint: デフォルトは7, Squirrelです。値が低いほど高速ですが品質が低くなります。
  speed_items:
    - 1, Lightning（最速、最低品質）
    - 2, Thunder（非常に高速、低品質）
    - 3, Falcon（高速、やや低品質）
    - 4, Cheetah（速度と品質のバランス）
    - 5, Hare（やや遅い、良好な品質）
    - 6, Wombat（遅い、非常に良好な品質）
    - 7, Squirrel（非常に遅い、最高品質）
    - 8, Kitten（最高品質、非常に遅い）
    - 9, Tortoise（最高品質、非常に遅い）
    - 10, Glacier（最高品質、非常に遅い、アーカイブ用）
  quality: 品質 / 距離（{min}～{max}）
  quality_hint: 距離パラメータです。値が低いほど品質が高くなります。0.0=ロスレス、1.0=視覚的にロスレス（推奨）、3.0=高品質。推奨範囲は{min}～{max}です。
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
  color_encoding_items:
    - Srgb（デフォルト、標準的なsRGBカラースペース）
    - LinearSrgb（リニアsRGBカラースペース）
    - SrgbLuma（輝度情報を含むsRGBカラースペース）
    - LinearSrgbLuma（輝度情報を含むリニアsRGBカラースペース）
  reset_jxl_options: JPEG XL設定をリセット
ko:
  title: JPEG XL 옵션
  lossless: 무손실 압축
  lossless_hint: 대부분의 이미지에서 작동하지만 드문 경우에 적용되지 않을 수 있습니다.
  speed: 인코딩 속도
  speed_hint: 기본값은 7, Squirrel입니다. 값이 낮을수록 빠르지만 품질이 낮아집니다.
  speed_items:
    - 1, Lightning (가장 빠른 속도, 가장 낮은 품질)
    - 2, Thunder (매우 빠름, 낮은 품질)
    - 3, Falcon (빠름, 약간 낮은 품질)
    - 4, Cheetah (속도와 품질의 균형)
    - 5, Hare (약간 느림, 좋은 품질)
    - 6, Wombat (느림, 매우 좋은 품질)
    - 7, Squirrel (매우 느림, 최고 품질)
    - 8, Kitten (최고 품질, 매우 느림)
    - 9, Tortoise (최고 품질, 매우 느림)
    - 10, Glacier (최고 품질, 매우 느림, 보관용)
  quality: 품질 / 거리 ({min}-{max})
  quality_hint: 거리 매개변수입니다. 값이 낮을수록 품질이 높아집니다. 0.0=무손실, 1.0=시각적 무손실(권장), 3.0=고품질. 권장 범위는 ({min}-{max})입니다.
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
  color_encoding_items:
    - Srgb (기본값, 표준 sRGB 색상 공간)
    - LinearSrgb (선형 sRGB 색상 공간)
    - SrgbLuma (휘도 정보가 포함된 sRGB 색상 공간)
    - LinearSrgbLuma (휘도 정보가 포함된 선형 sRGB 색상 공간)
  reset_jxl_options: JPEG XL 설정 재설정
zhHants:
  title: JPEG XL 選項
  lossless: 無損壓縮
  lossless_hint: 大多數圖像均可使用，但極少數情況下可能無法應用。
  speed: 編碼速度
  speed_hint: 默認為 7, Squirrel。值越低，速度越快，但質量越低。
  speed_items:
    - 1, Lightning（最快速度，最低質量）
    - 2, Thunder（非常快，低質量）
    - 3, Falcon（快，稍低質量）
    - 4, Cheetah（速度和質量的平衡）
    - 5, Hare（稍慢，良好質量）
    - 6, Wombat（慢，非常好質量）
    - 7, Squirrel（非常慢，最高質量）
    - 8, Kitten（最佳質量，非常慢）
    - 9, Tortoise（最佳質量，非常慢）
    - 10, Glacier（最佳質量，非常慢，用於存檔）
  quality: 質量 / 距離 ({min}-{max})
  quality_hint: 距離參數。值越低，質量越高。0.0=無損，1.0=視覺上無損（建議），3.0=高質量。建議範圍為 ({min}-{max})。
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
  color_encoding_items:
    - Srgb (默認，標準 sRGB 色彩空間)
    - LinearSrgb (線性 sRGB 色彩空間)
    - SrgbLuma (具有亮度信息的 sRGB 色彩空間)
    - LinearSrgbLuma (具有亮度信息的線性 sRGB 色彩空間)
  reset_jxl_options: 重置 JPEG XL 設置
zhHans:
  title: JPEG XL 选项
  lossless: 无损压缩
  lossless_hint: 大多数图像均可使用，但极少数情况下可能无法应用。
  speed: 编码速度
  speed_hint: 默认值为 7, Squirrel。值越低，速度越快，但质量越低。
  speed_items:
    - 1, Lightning（最快速度，最低质量）
    - 2, Thunder（非常快，低质量）
    - 3, Falcon（快，稍低质量）
    - 4, Cheetah（速度和质量的平衡）
    - 5, Hare（稍慢，良好质量）
    - 6, Wombat（慢，非常好质量）
    - 7, Squirrel（非常慢，最高质量）
    - 8, Kitten（最佳质量，非常慢）
    - 9, Tortoise（最佳质量，非常慢）
    - 10, Glacier（最佳质量，非常慢，用于存档）
  quality: 质量 / 距离 ({min}-{max})
  quality_hint: 距离参数。值越低，质量越高。0.0=无损，1.0=视觉上无损（建议），3.0=高质量。建议范围为 ({min}-{max})。
  use_container: 使用 JPEG XL 容器格式
  use_container_hint: 使用 JPEG XL 容器格式可以保存 JPEG 重建等元数据。但是，即使没有其他元数据，编码文件也会为容器头添加几个字节。
  uses_original_profile: 使用原始色彩配置文件
  uses_original_profile_hint: 如果输入图像具有色彩配置文件，则该配置文件将用于编码图像。否则，将选择内部固定色彩配置文件（应该更小）。
  decoding_speed: 解码速度 ({min}-{max})。
  decoding_speed_hint: 值越低，质量越高。
  init_buffer_size: 输出缓冲区
  init_buffer_size_hint: 输出缓冲区的初始大小（以千字节为单位）。小于 32KB 的值将四舍五入为 32KB。
  color_encoding: 色彩编码方法
  color_encoding_hint: 如果您不知道，建议不要选择 Srgb 以外的任何选项。
  color_encoding_items:
    - Srgb (默认，标准 sRGB 色彩空间)
    - LinearSrgb (线性 sRGB 色彩空间)
    - SrgbLuma (具有亮度信息的 sRGB 色彩空间)
    - LinearSrgbLuma (具有亮度信息的线性 sRGB 色彩空间)
  reset_jxl_options: 重置 JPEG XL 设置
</i18n>
