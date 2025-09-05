<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import AvifOptions from './SettingTabItems/AvifOptions.vue';
import CommonOptions from './SettingTabItems/CommonOptions.vue';
import WebpOptions from './SettingTabItems/WebpOptions.vue';

const { t } = useI18n();

const tab = ref('common');
const dialog = ref(false);
</script>

<template>
  <v-dialog v-model="dialog" width="800">
    <v-card :title="t('settings')">
      <template #actions>
        <v-btn color="primary" class="ms-auto" text="OK" @click="dialog = false" />
      </template>
      <v-card-text>
        <v-tabs v-model="tab" grow>
          <v-tab value="common">{{ t('common_options') }}</v-tab>
          <v-tab value="webp">WebP</v-tab>
          <v-tab value="avif">AVIF</v-tab>
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
  </v-dialog>
</template>
