<template>
  <div>
    <v-card>
      <v-card-title class="text-h4">
        <v-icon start>mdi-test-tube</v-icon>
        {{ $t('test.title') }}
      </v-card-title>
      <v-card-text>
        <p class="mb-4">{{ $t('test.description') }}</p>

        <v-divider class="my-4" />

        <h3 class="text-h5 mb-3">{{ $t('test.button_test') }}</h3>
        <div class="mb-4">
          <v-btn color="primary" class="me-2">Primary</v-btn>
          <v-btn color="secondary" class="me-2">Secondary</v-btn>
          <v-btn color="success" class="me-2">Success</v-btn>
          <v-btn color="error" class="me-2">Error</v-btn>
        </div>

        <h3 class="text-h5 mb-3">{{ $t('test.icon_test') }}</h3>
        <div class="mb-4">
          <v-icon icon="mdi-heart" color="red" size="large" class="me-2" />
          <v-icon icon="mdi-star" color="amber" size="large" class="me-2" />
          <v-icon icon="mdi-thumb-up" color="green" size="large" class="me-2" />
          <v-icon icon="mdi-settings" color="blue" size="large" class="me-2" />
        </div>

        <h3 class="text-h5 mb-3">{{ $t('test.card_test') }}</h3>
        <v-row>
          <v-col v-for="n in 2" :key="n" cols="12" md="6">
            <v-card elevation="2">
              <v-card-title>{{ $t('test.card_title') }} {{ n }}</v-card-title>
              <v-card-text>
                {{ $t('test.card_content') }}
              </v-card-text>
              <v-card-actions>
                <v-btn color="primary" variant="text">{{ $t('test.action_button') }}</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>

        <h3 class="text-h5 mb-3">{{ $t('test.form_test') }}</h3>
        <v-row>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="testInput"
              :label="$t('test.test_input')"
              prepend-inner-icon="mdi-pencil"
              variant="outlined"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-select
              v-model="testSelect"
              :items="selectItems"
              :label="$t('test.test_select')"
              prepend-inner-icon="mdi-format-list-bulleted"
              variant="outlined"
            />
          </v-col>
        </v-row>

        <v-alert type="success" class="mt-4">{{ $t('test.success_message') }}</v-alert>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const testInput = ref('');
const testSelect = ref('');
const selectItems = computed(() => [
  t('test.select_options.option1'),
  t('test.select_options.option2'),
  t('test.select_options.option3')
]);
</script>
