<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import AvifOptions from './SettingItems/AvifOptions.vue';
import CommonOptions from './SettingItems/CommonOptions.vue';
import JpegOptions from './SettingItems/JpegOptions.vue';
import JxlOptions from './SettingItems/JxlOptions.vue';
import PngOptions from './SettingItems/PngOptions.vue';
import WebpOptions from './SettingItems/WebpOptions.vue';

const { t } = useI18n();

const tab = ref('common');
</script>

<template>
  <v-dialog fullscreen persistent>
    <template #activator="{ props: dialogProps }">
      <v-tooltip :text="t('settings')" location="bottom">
        <template #activator="{ props: tooltipProps }">
          <v-btn
            v-bind="{ ...dialogProps, ...tooltipProps }"
            icon="mdi-cog-outline"
            variant="plain"
          />
        </template>
      </v-tooltip>
    </template>
    <template #default="{ isActive }">
      <v-card flat>
        <v-toolbar>
          <v-toolbar-title>{{ t('settings') }}</v-toolbar-title>
          <v-spacer />
          <v-btn icon="mdi-close" @click="isActive.value = false" />
        </v-toolbar>
        <v-card-text class="d-flex flex-row pa-0" style="height: calc(100vh - 64px)">
          <v-layout>
            <v-navigation-drawer permanent>
              <v-list nav>
                <v-list-item
                  :title="t('common_options')"
                  value="common"
                  :active="tab === 'common'"
                  @click="tab = 'common'"
                />
                <v-list-item
                  :title="t('webp_options')"
                  value="webp"
                  :active="tab === 'webp'"
                  @click="tab = 'webp'"
                />
                <v-list-item
                  :title="t('avif_options')"
                  value="avif"
                  :active="tab === 'avif'"
                  @click="tab = 'avif'"
                />
                <v-list-item
                  :title="t('jxl_options')"
                  value="jxl"
                  :active="tab === 'jxl'"
                  @click="tab = 'jxl'"
                />
                <v-list-item
                  :title="t('png_options')"
                  value="png"
                  :active="tab === 'png'"
                  @click="tab = 'png'"
                />
                <v-list-item
                  :title="t('jpeg_options')"
                  value="jpeg"
                  :active="tab === 'jpeg'"
                  @click="tab = 'jpeg'"
                />
              </v-list>
            </v-navigation-drawer>
            <v-main class="overflow-y-auto">
              <v-card flat class="pa-2">
                <v-window v-model="tab">
                  <v-window-item value="common">
                    <common-options />
                  </v-window-item>
                  <v-window-item value="webp">
                    <webp-options />
                  </v-window-item>
                  <v-window-item value="avif">
                    <avif-options />
                  </v-window-item>
                  <v-window-item value="jxl">
                    <jxl-options />
                  </v-window-item>
                  <v-window-item value="png">
                    <png-options />
                  </v-window-item>
                  <v-window-item value="jpeg">
                    <jpeg-options />
                  </v-window-item>
                </v-window>
              </v-card>
            </v-main>
          </v-layout>
        </v-card-text>
      </v-card>
    </template>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  settings: Settings
  common_options: Common Options
  png_options: PNG Options
  jpeg_options: JPEG Options
  webp_options: WebP Options
  avif_options: AVIF Options
  jxl_options: JPEG XL Options
fr:
  settings: Paramètres
  common_options: Options communes
  png_options: Options PNG
  jpeg_options: Options JPEG
  webp_options: Options WebP
  avif_options: Options AVIF
  jxl_options: Options JPEG XL
ja:
  settings: 設定
  common_options: 共通設定
  png_options: PNG 設定
  jpeg_options: JPEG 設定
  webp_options: WebP 設定
  avif_options: AVIF 設定
  jxl_options: JPEG XL 設定
ko:
  settings: 설정
  common_options: 공통 설정
  png_options: PNG 설정
  jpeg_options: JPEG 설정
  webp_options: WebP 설정
  avif_options: AVIF 설정
  jxl_options: JPEG XL 설정
zhHant:
  settings: 設定
  common_options: 共通設定
  png_options: PNG 設定
  jpeg_options: JPEG 設定
  webp_options: WebP 設定
  avif_options: AVIF 設定
  jxl_options: JPEG XL 設定
zhHans:
  settings: 设置
  common_options: 通用设置
  png_options: PNG 设置
  jpeg_options: JPEG 设置
  webp_options: WebP 设置
  avif_options: AVIF 设置
  jxl_options: JPEG XL 设置
</i18n>
