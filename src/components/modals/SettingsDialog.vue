<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import AvifOptions from './SettingTabItems/AvifOptions.vue';
import CommonOptions from './SettingTabItems/CommonOptions.vue';
import WebpOptions from './SettingTabItems/WebpOptions.vue';

const { t } = useI18n();

const tab = ref('common');
</script>

<template>
  <v-dialog fullscreen persistent>
    <template #activator="{ props: activatorProps }">
      <v-btn icon="mdi-cog" variant="plain" v-bind="activatorProps" />
    </template>
    <template #default="{ isActive }">
      <v-card>
        <v-toolbar>
          <v-btn icon="mdi-close" @click="isActive.value = false" />
          <v-toolbar-title>{{ t('settings') }}</v-toolbar-title>
        </v-toolbar>
        <v-card-text>
          <v-tabs v-model="tab" grow>
            <v-tab value="common">{{ t('common_options') }}</v-tab>
            <v-tab value="webp">{{ t('webp_options') }}</v-tab>
            <v-tab value="avif">{{ t('avif_options') }}</v-tab>
          </v-tabs>
          <v-divider />
          <v-window v-model="tab" class="mt-4">
            <v-window-item value="common">
              <common-options />
            </v-window-item>
            <v-window-item value="webp">
              <webp-options />
            </v-window-item>
            <v-window-item value="avif">
              <avif-options />
            </v-window-item>
          </v-window>
        </v-card-text>
      </v-card>
    </template>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  settings: 'Settings'
  common_options: 'Common Options'
  webp_options: 'WebP Options'
  avif_options: 'AVIF Options'
ja:
  settings: '設定'
  common_options: '共通設定'
  webp_options: 'WebP設定'
  avif_options: 'AVIF設定'
</i18n>
