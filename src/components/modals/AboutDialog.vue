<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import Meta from '@/Meta';

const { t } = useI18n();
</script>

<template>
  <v-dialog width="auto">
    <template #activator="{ props: activatorProps }">
      <v-btn v-bind="activatorProps" icon="mdi-information-outline" variant="plain" />
    </template>
    <template #default="{ isActive }">
      <v-card width="360" :title="t('title')">
        <template #actions>
          <v-btn color="primary" class="ms-auto" text="OK" @click="isActive.value = false" />
        </template>
        <v-card-text class="text-center">
          <h1>Drop Compress Image</h1>
          <p>
            Version {{ Meta.version }}
            <br />
            <small>(Build: {{ Meta.date }})</small>
          </p>
        </v-card-text>
      </v-card>
    </template>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  title: 'About'
ja:
  title: 'このアプリケーションについて'
</i18n>
