<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import { openUrl } from '@tauri-apps/plugin-opener';

import Meta from '@/Meta';

const { t } = useI18n();

const openGitHub = async () => {
  await openUrl('https://github.com/logue/DropWebP');
};
</script>

<template>
  <v-dialog width="auto">
    <template #activator="{ props: dialogProps }">
      <v-tooltip :text="t('about_title')" location="bottom">
        <template #activator="{ props: tooltipProps }">
          <v-btn
            v-bind="{ ...dialogProps, ...tooltipProps }"
            icon="mdi-information-outline"
            variant="plain"
          />
        </template>
      </v-tooltip>
    </template>
    <template #default="{ isActive }">
      <v-card width="360" :title="t('about_title')">
        <template #actions>
          <v-btn color="primary" class="ms-auto" text="OK" @click="isActive.value = false" />
        </template>
        <v-card-text class="text-center">
          <h3>Drop Compress Image</h3>
          <p>
            Version {{ Meta.version }}
            <br />
            <small>(Build: {{ Meta.date }})</small>
          </p>
          <p>
            <a href="https://github.com/logue/DropWebP" target="_blank" @click="openGitHub">
              https://github.com/logue/DropWebP
            </a>
          </p>
        </v-card-text>
      </v-card>
    </template>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  about_title: 'About'
ja:
  about_title: 'このアプリケーションについて'
kr:
  about_title: '정보'
zh:
  about_title: '關於'
</i18n>
