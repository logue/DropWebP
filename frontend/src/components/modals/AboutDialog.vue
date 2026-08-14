<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import { openUrl } from '@tauri-apps/plugin-opener';

import Meta from '@/types/Meta';

const { t } = useI18n();

const PROJECT_SITE = 'https://logue.dev/DropWebP/';
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
      <v-card :title="t('about_title')" width="360">
        <v-card-text class="text-center">
          <!-- eslint-disable @intlify/vue-i18n/no-raw-text -- App brand name and build metadata are not localizable -->
          <h2>Drop Compress Image</h2>
          <p>
            Version {{ Meta.version }}
            <br />
            <small>(Build: {{ Meta.date }})</small>
          </p>
          <!-- eslint-enable @intlify/vue-i18n/no-raw-text -->
          <p>
            <a :href="PROJECT_SITE" target="_blank" @click.prevent="openUrl(PROJECT_SITE)">
              {{ PROJECT_SITE }}
            </a>
          </p>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="primary" class="ms-auto" text="OK" @click="isActive.value = false" />
        </v-card-actions>
      </v-card>
    </template>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  about_title: About this application
fr:
  about_title: À propos de cette application
ja:
  about_title: このアプリケーションについて
ko:
  about_title: 이 애플리케이션에 대하여
zhHant:
  about_title: 關於這個應用程式
zhHans:
  about_title: 关于这个应用程序
</i18n>
