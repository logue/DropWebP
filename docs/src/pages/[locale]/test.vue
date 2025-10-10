<script setup lang="ts">
// ページのメタデータとSEO設定
useHead({
  title: 'Test Page',
  meta: [{ name: 'description', content: 'Vuetify component test page' }]
});

// フォームのリアクティブデータ
const testName = ref('');
const testEmail = ref('');
const testMessage = ref('');
</script>

<template>
  <div>
    <v-card>
      <v-card-title class="text-h4">
        <v-icon start>mdi-test-tube</v-icon>
        {{ t('title') }}
      </v-card-title>
      <v-card-text>
        <p class="mb-4">{{ t('description') }}</p>

        <v-divider class="my-4" />

        <h3 class="text-h5 mb-3">{{ t('button_test') }}</h3>
        <div class="mb-4">
          <v-btn color="primary" class="me-2">Primary</v-btn>
          <v-btn color="secondary" class="me-2">Secondary</v-btn>
          <v-btn color="success" class="me-2">Success</v-btn>
          <v-btn color="error" class="me-2">Error</v-btn>
        </div>

        <h3 class="text-h5 mb-3">{{ t('icon_test') }}</h3>
        <div class="mb-4">
          <v-icon icon="mdi-heart" color="red" size="large" class="me-2" />
          <v-icon icon="mdi-star" color="amber" size="large" class="me-2" />
          <v-icon icon="mdi-thumb-up" color="green" size="large" class="me-2" />
          <v-icon icon="mdi-settings" color="blue" size="large" class="me-2" />
        </div>

        <h3 class="text-h5 mb-3">{{ t('card_test') }}</h3>
        <v-row>
          <v-col cols="12" md="6">
            <v-card>
              <v-card-title>{{ t('card_title') }}</v-card-title>
              <v-card-text>{{ t('card_content') }}</v-card-text>
              <v-card-actions>
                <v-btn color="primary">{{ t('action_button') }}</v-btn>
                <v-spacer />
                <v-btn icon>
                  <v-icon>mdi-heart</v-icon>
                </v-btn>
                <v-btn icon>
                  <v-icon>mdi-share</v-icon>
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>

        <h3 class="text-h5 mb-3 mt-4">{{ t('form_test') }}</h3>
        <v-form>
          <v-text-field
            v-model="testName"
            :label="t('form_name')"
            variant="outlined"
            class="mb-3"
          />
          <v-text-field
            v-model="testEmail"
            :label="t('form_email')"
            type="email"
            variant="outlined"
            class="mb-3"
          />
          <v-textarea
            v-model="testMessage"
            :label="t('form_message')"
            variant="outlined"
            rows="3"
            class="mb-3"
          />
          <v-btn color="primary" size="large">
            {{ t('form_submit') }}
          </v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </div>
</template>
