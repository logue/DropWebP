<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
const features = [
  {
    icon: 'mdi-image-multiple',
    key: 'multiple_formats'
  },
  {
    icon: 'mdi-lightning-bolt',
    key: 'high_speed'
  },
  {
    icon: 'mdi-drag',
    key: 'drag_drop'
  }
];
</script>

<template>
  <div>
    <v-card class="mb-6">
      <v-card-title class="text-h4 text-center pa-6">
        {{ t('home.title') }}
      </v-card-title>
      <v-card-text class="text-center">
        <p class="text-h6 mb-4">{{ t('home.subtitle') }}</p>
        <v-btn color="primary" size="large" to="/test">
          <v-icon start>mdi-rocket</v-icon>
          {{ t('home.start_button') }}
        </v-btn>
      </v-card-text>
    </v-card>

    <v-row>
      <v-col v-for="item in features" :key="item.key" cols="12" md="4">
        <v-card class="h-100">
          <v-card-text class="text-center">
            <v-icon :icon="item.icon" size="64" color="primary" class="mb-4" />
            <h3 class="text-h6 mb-2">{{ t(`home.features.${item.key}.title`) }}</h3>
            <p>{{ t(`home.features.${item.key}.description`) }}</p>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>
