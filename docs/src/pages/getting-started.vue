<script setup lang="ts">
const { t } = useI18n();
// メタデータ設定
useHead({
  title: t('getting_started.title'),
  meta: [{ name: 'description', content: t('getting_started.meta_description') }]
});

// インストールステップ
const installationSteps = ['1', '2', '3'];

// 使用方法
const usageMethods = [
  { key: 'drag_drop', icon: 'mdi-drag' },
  { key: 'file_menu', icon: 'mdi-file-multiple' },
  { key: 'batch_convert', icon: 'mdi-package-variant' },
  { key: 'settings', icon: 'mdi-cog' }
];

// サポート機能
const supportedFeatures = [
  { key: 'webp_support', icon: 'mdi-check-circle' },
  { key: 'avif_support', icon: 'mdi-check-circle' },
  { key: 'jxl_support', icon: 'mdi-check-circle' },
  { key: 'batch_processing', icon: 'mdi-check-circle' },
  { key: 'quality_control', icon: 'mdi-check-circle' },
  { key: 'cross_platform', icon: 'mdi-check-circle' }
];
</script>

<template>
  <v-card class="mb-6" tag="section">
    <v-card-title class="text-h3 text-center pa-6" tag="h2">
      {{ t('title') }}
    </v-card-title>
    <v-card-text>
      <p class="text-h6 mb-4">{{ t('subtitle') }}</p>
    </v-card-text>
  </v-card>

  <v-card class="mb-6" tag="section">
    <v-card-title class="text-h4 pa-4 bg-primary text-white" tag="h2">
      {{ t('installation.title') }}
    </v-card-title>
    <v-card-text class="pa-6">
      <v-stepper :items="installationSteps" variant="text">
        <template #item.1>
          <div>
            <h3 class="text-h6 mb-3">{{ t('installation.step1_title') }}</h3>
            <p class="mb-3">{{ t('installation.step1_desc') }}</p>
            <v-btn
              color="primary"
              variant="outlined"
              prepend-icon="mdi-download"
              href="https://github.com/logue/DropWebP/releases"
              target="_blank"
            >
              {{ t('download') }}
            </v-btn>
          </div>
        </template>

        <template #item.2>
          <div>
            <h3 class="text-h6 mb-3">{{ t('installation.step2_title') }}</h3>
            <p>{{ t('installation.step2_desc') }}</p>
          </div>
        </template>

        <template #item.3>
          <div>
            <h3 class="text-h6 mb-3">{{ t('installation.step3_title') }}</h3>
            <p>{{ t('installation.step3_desc') }}</p>
          </div>
        </template>
      </v-stepper>
    </v-card-text>
  </v-card>

  <v-card class="mb-6" tag="section">
    <v-card-title class="text-h4 pa-4 bg-secondary text-white" tag="h2">
      {{ t('usage.title') }}
    </v-card-title>
    <v-card-text class="pa-6">
      <v-row>
        <v-col v-for="method in usageMethods" :key="method.key" cols="12" md="6">
          <v-card elevation="2" class="h-100">
            <v-card-text class="text-center pa-6">
              <v-icon :icon="method.icon" size="48" color="primary" class="mb-4" />
              <h4 class="text-h6 mb-3">
                {{ t(`usage.methods.${method.key}.title`) }}
              </h4>
              <p>{{ t(`usage.methods.${method.key}.description`) }}</p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>

  <v-card tag="section">
    <v-card-title class="text-h4 pa-4 bg-success text-white" tag="h2">
      {{ t('features.title') }}
    </v-card-title>
    <v-card-text class="pa-6">
      <v-list>
        <v-list-item v-for="feature in supportedFeatures" :key="feature.key">
          <template #prepend>
            <v-icon :icon="feature.icon" color="success" />
          </template>
          <v-list-item-title>
            {{ t(`features.list.${feature.key}.title`) }}
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ t(`features.list.${feature.key}.description`) }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>

<i18n lang="yaml"></i18n>
