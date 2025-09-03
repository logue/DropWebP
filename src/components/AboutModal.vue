<script setup lang="ts">
import { ref, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';

import Meta from '@/Meta';

const { t } = useI18n();

/** Open Dialog */
const open = () => (dialog.value = true);

/** Dialog visibility */
const dialog: Ref<boolean> = ref(false);

defineExpose({
  open
});
</script>

<template>
  <v-dialog v-model="dialog" width="auto">
    <v-card width="360" :title="t('title')">
      <template #actions>
        <v-btn color="primary" class="ms-auto" text="OK" @click="dialog = false" />
      </template>
      <v-card-text class="text-center">
        <h1>DropWebP</h1>
        <p>
          Version {{ Meta.version }}
          <br />
          <small>(Build: {{ Meta.date }})</small>
        </p>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  title: 'About'
ja:
  title: 'このアプリケーションについて'
</i18n>
