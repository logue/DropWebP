<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import AvifOptions from './SettingTabItems/AvifOptions.vue';
import CommonOptions from './SettingTabItems/CommonOptions.vue';
import JxlOptions from './SettingTabItems/JxlOptions.vue';
import PngOptions from './SettingTabItems/PngOptions.vue';
import WebpOptions from './SettingTabItems/WebpOptions.vue';

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
      <v-card>
        <v-toolbar>
          <v-toolbar-title>{{ t('settings') }}</v-toolbar-title>
          <v-spacer />
          <v-btn icon="mdi-close" @click="isActive.value = false" />
        </v-toolbar>
        <v-card-text>
          <v-tabs v-model="tab" color="primary">
            <v-tab value="common">{{ t('common_options') }}</v-tab>
            <v-tab value="png">{{ t('png_options') }}</v-tab>
            <v-tab value="webp">{{ t('webp_options') }}</v-tab>
            <v-tab value="avif">{{ t('avif_options') }}</v-tab>
            <v-tab value="jxl">{{ t('jxl_options') }}</v-tab>
          </v-tabs>
          <v-divider />
          <v-window v-model="tab" class="mt-4">
            <v-window-item value="common">
              <common-options />
            </v-window-item>
            <v-window-item value="png">
              <png-options />
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
          </v-window>
        </v-card-text>
      </v-card>
    </template>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  settings: Settings
  common_options: Common Options
  png_options: PNG(Zopfli) Options
  webp_options: WebP Options
  avif_options: AVIF Options
  jxl_options: JPEG XL Options
fr:
  settings: Paramètres
  common_options: Options communes
  png_options: Options PNG(Zopfli)
  webp_options: Options WebP
  avif_options: Options AVIF
  jxl_options: Options JPEG XL
ja:
  settings: 設定
  common_options: 共通設定
  png_options: PNG(Zopfli)設定
  webp_options: WebP設定
  avif_options: AVIF設定
  jxl_options: JPEG XL設定
ko:
  settings: 설정
  common_options: 공통 설정
  png_options: PNG(Zopfli) 설정
  webp_options: WebP 설정
  avif_options: AVIF 설정
  jxl_options: JPEG XL 설정
zhHant:
  settings: 設定
  common_options: 共通設定
  png_options: PNG(Zopfli)設定
  webp_options: WebP設定
  avif_options: AVIF設定
  jxl_options: JPEG XL設定
zhHans:
  settings: 设置
  common_options: 通用设置
  png_options: PNG(Zopfli)设置
  webp_options: WebP设置
  avif_options: AVIF设置
  jxl_options: JPEG XL设置
</i18n>
