<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { PngFilter } from '@/types/PngTypes';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-slider
    v-model="settingsStore.pngOptions.zopfliIterations"
    :label="t('zopfli_iterations')"
    :hint="t('zopfli_iterations_hint', { min: 15, max: 100 })"
    :min="15"
    :max="100"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.pngOptions.embedIccProfile"
    :hint="t('embed_icc_profile_hint')"
    :label="t('embed_icc_profile')"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.pngOptions.bitDepthReduction"
    :label="t('bit_depth_reduction')"
    :hint="t('bit_depth_reduction_hint')"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.pngOptions.colorTypeReduction"
    :label="t('color_type_reduction')"
    :hint="t('color_type_reduction_hint')"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.pngOptions.paletteReduction"
    :label="t('palette_reduction')"
    :hint="t('palette_reduction_hint')"
    color="primary"
    persistent-hint
  />
  <v-radio-group v-model="settingsStore.pngOptions.interlace" :label="t('interlace')">
    <v-radio :label="t('interlace_item.none')" color="primary" persistent-hint value="none" />
    <v-radio :label="t('interlace_item.adam7')" color="primary" persistent-hint value="adam7" />
  </v-radio-group>
  <v-select
    v-model="settingsStore.pngOptions.filter"
    :label="t('filter')"
    :hint="t('filter_hint')"
    :items="[
      { text: t('filter_item.none'), value: PngFilter.None },
      { text: t('filter_item.sub'), value: PngFilter.Sub },
      { text: t('filter_item.up'), value: PngFilter.Up },
      { text: t('filter_item.average'), value: PngFilter.Average },
      { text: t('filter_item.paeth'), value: PngFilter.Paeth },
      { text: t('filter_item.minSum'), value: PngFilter.MinSum },
      { text: t('filter_item.entropy'), value: PngFilter.Entropy },
      { text: t('filter_item.bigrams'), value: PngFilter.Bigrams },
      { text: t('filter_item.bigEnt'), value: PngFilter.BigEnt },
      { text: t('filter_item.brute'), value: PngFilter.Brute }
    ]"
    color="primary"
    item-title="text"
    item-value="value"
    persistent-hint
  />
</template>

<i18n lang="yaml">
en:
  zopfli_iterations: Zopfli Iterations
  zopfli_iterations_hint: Specify the number of iterations for Zopfli compression. It can be set in the range of {min} to {max}. A higher value improves compression ratio but increases processing time.
  embed_icc_profile: Embed ICC Profile
  embed_icc_profile_hint: If the image contains an ICC profile, it will be embedded in the output image.
  interlace: Interlace Setting
  interlace_item:
    none: No Interlace
    none_hint: Default. Minimum file size.
    adam7: Adam7 Interlace
    adam7_hint: Allows for a quicker preview of the image but results in a larger file size.
  bit_depth_reduction: Bit Depth Reduction
  bit_depth_reduction_hint: Reduces the bit depth where possible to decrease file size.
  color_type_reduction: Color Type Reduction
  color_type_reduction_hint: Reduces the color type where possible to decrease file size.
  palette_reduction: Palette Reduction
  palette_reduction_hint: Uses a palette where possible to decrease file size.
  filter: PNG Filter
  filter_hint: Select the PNG filter type to be used during compression.
  filter_item:
    none: None
    sub: Sub (left pixel difference)
    up: Up (above pixel difference)
    average: Average (left and above pixel difference)
    paeth: Paeth (predicted vs actual pixel difference)
    minSum: MinSum (try all filters and choose minimum)
    entropy: Entropy (choose filter with minimum entropy)
    bigrams: Bigrams (2-gram frequency analysis)
    bigEnt: BigEnt (combination of bigrams and entropy)
    brute: Brute (tries all combinations, slowest but best compression)
fr:
  zopfli_iterations: Itérations Zopfli
  zopfli_iterations_hint: Spécifiez le nombre d'itérations pour la compression Zopfli. Il peut être défini dans la plage de {min} à {max}. Une valeur plus élevée améliore le ratio de compression mais augmente le temps de traitement.
  embed_icc_profile: Intégrer le profil ICC
  embed_icc_profile_hint: Si l'image contient un profil ICC, il sera intégré dans l'image de sortie.
  interlace: Paramètre d'entrelacement
  interlace_item:
    none: Pas d'entrelacement
    none_hint: Par défaut. Taille de fichier minimale.
    adam7: Entrelacement Adam7
    adam7_hint: Permet un aperçu plus rapide de l'image mais entraîne une taille de fichier plus grande.
  bit_depth_reduction: Réduction de la profondeur de bits
  bit_depth_reduction_hint: Réduit la profondeur de bits lorsque cela est possible pour diminuer la taille du fichier.
  color_type_reduction: Réduction du type de couleur
  color_type_reduction_hint: Réduit le type de couleur lorsque cela est possible pour diminuer la taille du fichier.
  palette_reduction: Réduction de la palette
  palette_reduction_hint: Utilise une palette lorsque cela est possible pour diminuer la taille du fichier.
  filter: Filtre PNG
  filter_hint: Sélectionnez le type de filtre PNG à utiliser lors de la compression.
  filter_item:
    none: Aucun
    sub: Sub (différence avec le pixel de gauche)
    up: Up (différence avec le pixel du dessus)
    average: Moyenne (différence avec la moyenne des pixels de gauche et du dessus)
    paeth: Paeth (différence entre le pixel prédit et le pixel réel)
    minSum: MinSum (essaye tous les filtres et choisit le minimum)
    entropy: Entropie (choisit le filtre avec l'entropie minimale)
    bigrams: Bigrammes (analyse de fréquence des 2-grammes)
    bigEnt: BigEnt (combinaison de bigrammes et d'entropie)
    brute: Brute (essaye toutes les combinaisons, le plus lent mais la meilleure compression)
ja:
  zopfli_iterations: Zopfliの反復回数
  zopfli_iterations_hint: Zopfli圧縮の反復回数を指定します。{min}〜{max}の範囲で設定可能です。値が大きいほど圧縮率が向上しますが、処理時間も長くなります。
  embed_icc_profile: ICCプロファイルを埋め込む
  embed_icc_profile_hint: 画像にICCプロファイルが含まれている場合、出力画像に埋め込みます。
  interlace: インターレース設定
  interlace_item:
    none: インターレースなし
    none_hint: デフォルト。ファイルサイズが最小になります。
    adam7: Adam7インターレース
    adam7_hint: 画像のプレビューを迅速に表示できますが、ファイルサイズは大きくなります。
  bit_depth_reduction: ビット深度の削減
  bit_depth_reduction_hint: 可能な場合、ビット深度を削減してファイルサイズを小さくします。
  color_type_reduction: カラータイプの削減
  color_type_reduction_hint: 可能な場合、カラータイプを削減してファイルサイズを小さくします。
  palette_reduction: パレットの削減
  palette_reduction_hint: 可能な場合、パレットを使用してファイルサイズを小さくします。
  filter: PNGフィルター
  filter_hint: 圧縮時に使用するPNGフィルタータイプを選択します。
  filter_item:
    none: なし
    sub: サブ（左のピクセルとの差分）
    up: アップ（上のピクセルとの差分）
    average: 平均（左と上の平均との差分）
    paeth: ピース（予測値と実際の値の差分）
    minSum: 最小和（すべてのフィルターを試して最小を選択）
    entropy: エントロピー（最小エントロピーのフィルターを選択）
    bigrams: バイグラム（2グラム頻度分析）
    bigEnt: BigEnt（バイグラムとエントロピーの組み合わせ）
    brute: ブルートフォース（すべての組み合わせを試行、最も遅いが最良の圧縮）
ko:
  zopfli_iterations: Zopfli 반복 횟수
  zopfli_iterations_hint: Zopfli 압축의 반복 횟수를 지정합니다. {min}~{max} 범위 내에서 설정할 수 있습니다. 값이 높을수록 압축률이 향상되지만 처리 시간이 길어집니다.
  embed_icc_profile: ICC 프로필 포함
  embed_icc_profile_hint: 이미지에 ICC 프로필이 포함된 경우 출력 이미지에 포함합니다.
  interlace: 인터레이스 설정
  interlace_item:
    none: 인터레이스 없음
    none_hint: 기본값. 최소 파일 크기.
    adam7: Adam7 인터레이스
    adam7_hint: 이미지 미리보기를 더 빠르게 할 수 있지만 파일 크기가 커집니다.
  bit_depth_reduction: 비트 심도 감소
  bit_depth_reduction_hint: 가능한 경우 비트 심도를 줄여 파일 크기를 줄입니다.
  color_type_reduction: 색상 유형 감소
  color_type_reduction_hint: 가능한 경우 색상 유형을 줄여 파일 크기를 줄입니다.
  palette_reduction: 팔레트 감소
  palette_reduction_hint: 가능한 경우 팔레트를 사용하여 파일 크기를 줄입니다.
  filter: PNG 필터
  filter_hint: 압축 중에 사용할 PNG 필터 유형을 선택합니다.
  filter_item:
    none: 없음
    sub: Sub (왼쪽 픽셀 차이)
    up: Up (위 픽셀 차이)
    average: Average (왼쪽 및 위 픽셀 차이)
    paeth: Paeth (예측값과 실제 픽셀 차이)
    minSum: MinSum (모든 필터를 시도하고 최소값 선택)
    entropy: Entropy (최소 엔트로피 필터 선택)
    bigrams: Bigrams (2-그램 빈도 분석)
    bigEnt: BigEnt (바이그램과 엔트로피의 조합)
    brute: Brute (모든 조합 시도, 가장 느리지만 최고의 압축)
zh-tw:
  zopfli_iterations: Zopfli 迭代次數
  zopfli_iterations_hint: 指定 Zopfli 壓縮的迭代次數。可以在 {min} 到 {max} 的範圍內設置。值越高，壓縮率越高，但處理時間也會增加。
  embed_icc_profile: 嵌入 ICC 配置文件
  embed_icc_profile_hint: 如果圖像包含 ICC 配置文件，則將其嵌入輸出圖像中。
  interlace: 交錯設置
  interlace_item:
    none: 無交錯
    none_hint: 默認。最小文件大小。
    adam7: Adam7 交錯
    adam7_hint: 允許更快地預覽圖像，但會導致文件大小增加。
  bit_depth_reduction: 位深度減少
  bit_depth_reduction_hint: 在可能的情況下減少位深度以減小文件大小。
  color_type_reduction: 顏色類型減少
  color_type_reduction_hint: 在可能的情況下減少顏色類型以減小文件大小。
  palette_reduction: 調色板減少
  palette_reduction_hint: 在可能的情況下使用調色板以減小文件大小。
  filter: PNG 過濾器
  filter_hint: 選擇在壓縮過程中使用的 PNG 過濾器類型。
  filter_item:
    none: 無
    sub: Sub（左側像素差異）
    up: Up（上方像素差異）
    average: Average（左側和上方像素差異）
    paeth: Paeth（預測值與實際像素差異）
    minSum: MinSum（嘗試所有過濾器並選擇最小值）
    entropy: Entropy（選擇具有最小熵的過濾器）
    bigrams: Bigrams（2-gram 頻率分析）
    bigEnt: BigEnt（bigrams 和 entropy 的組合）
    brute: Brute（暴力破解）
zh-cn:
  zopfli_iterations: Zopfli 迭代次数
  zopfli_iterations_hint: 指定 Zopfli 压缩的迭代次数。可以在 {min} 到 {max} 的范围内设置。值越高，压缩率越高，但处理时间也会增加。
  embed_icc_profile: 嵌入 ICC 配置文件
  embed_icc_profile_hint: 如果图像包含 ICC 配置文件，则将其嵌入输出图像中。
  interlace: 交错设置
  interlace_item:
    none: 无交错
    none_hint: 默认。最小文件大小。
    adam7: Adam7 交错
    adam7_hint: 允许更快地预览图像，但会导致文件大小增加。
  bit_depth_reduction: 位深度减少
  bit_depth_reduction_hint: 在可能的情况下减少位深度以减小文件大小。
  color_type_reduction: 颜色类型减少
  color_type_reduction_hint: 在可能的情况下减少颜色类型以减小文件大小 。
  palette_reduction: 调色板减少
  palette_reduction_hint: 在可能的情况下使用调色板以减小文件大小。
  filter: PNG 过滤器
  filter_hint: 选择在压缩过程中使用的 PNG 过滤器类型。
  filter_item:
    none: 无
    sub: Sub（左侧像素差异）
    up: Up（上方像素差异）
    average: Average（左侧和上方像素差异）
    paeth: Paeth（预测值与实际像素差异）
    minSum: MinSum（尝试所有过滤器并选择最小值）
    entropy: Entropy（选择具有最小熵的过滤器）
    bigrams: Bigrams（2-gram 频率分析）
    bigEnt: BigEnt（bigrams 和 entropy 的组合）
    brute: Brute（暴力破解）
</i18n>
