<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

// TODO:

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <!-- TODO: Add JPEG options
     <v-switch
    v-model="settingsStore.jpegOptions.progressive"
    :hint="t('progressive_hint')"
    :label="t('progressive')"
    color="primary"
    persistent-hint
  />
  <v-slider
    v-model="settingsStore.jpegOptions.quality"
    :hint="t('quality_hint', { min: 1, max: 100 })"
    :label="t('quality')"
    color="primary"
    max="100"
    min="1"
    persistent-hint
    step="1"
    thumb-label="always"
    type="number"
  />
  -->
</template>
