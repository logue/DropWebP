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

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

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

<i18n lang="yaml">
en:
  title: Vuetify Test Page
  description: A page to test whether vuetify-nuxt-module works properly.
  button_test: Button Test
  icon_test: Icon Test
  card_test: Card Test
  form_test: Form Test
  card_title: Card
  card_content: This is a test card. Vuetify's Material Design is applied.
  action_button: Action
  form_name: Test Input
  form_email: Test Email
  form_message: Test Message
  form_submit: Submit
  success_message: vuetify-nuxt-module is working properly!
ja:
  title: Vuetify テストページ
  description: vuetify-nuxt-moduleが正常に動作するかテストするページです。
  button_test: ボタンテスト
  icon_test: アイコンテスト
  card_test: カードテスト
  form_test: フォームテスト
  card_title: カード
  card_content: これはテストカードです。Vuetifyのマテリアルデザインが適用されています。
  action_button: アクション
  form_name: テスト入力
  form_email: テストメール
  form_message: テストメッセージ
  form_submit: 送信
  success_message: vuetify-nuxt-moduleが正常に動作しています！
ko:
  title: Vuetify 테스트 페이지
  description: vuetify-nuxt-module이 정상적으로 작동하는지 테스트하는 페이지입니다.
  button_test: 버튼 테스트
  icon_test: 아이콘 테스트
  card_test: 카드 테스트
  form_test: 폼 테스트
  card_title: 카드
  card_content: 이것은 테스트 카드입니다. Vuetify의 머테리얼 디자인이 적용되어 있습니다.
  action_button: 액션
  form_name: 테스트 입력
  form_email: 테스트 이메일
  form_message: 테스트 메시지
  form_submit: 제출
  success_message: vuetify-nuxt-module이 정상적으로 작동하고 있습니다!
zhHant:
  title: Vuetify 測試頁面
  description: 測試 vuetify-nuxt-module 是否正常運作的頁面。
  button_test: 按鈕測試
  icon_test: 圖示測試
  card_test: 卡片測試
  form_test: 表單測試
  card_title: 卡片
  card_content: 這是一個測試卡片。應用了 Vuetify 的 Material Design。
  action_button: 動作
  form_name: 測試輸入
  form_email: 測試電子郵件
  form_message: 測試訊息
  form_submit: 提交
  success_message: vuetify-nuxt-module 正常運作中！
zhHans:
  title: Vuetify 测试页面
  description: 测试 vuetify-nuxt-module 是否正常运作的页面。
  button_test: 按钮测试
  icon_test: 图标测试
  card_test: 卡片测试
  form_test: 表单测试
  card_title: 卡片
  card_content: 这是一个测试卡片。应用了 Vuetify 的 Material Design。
  action_button: 动作
  form_name: 测试输入
  form_email: 测试邮件
  form_message: 测试消息
  form_submit: 提交
  success_message: vuetify-nuxt-module 正常运作中！
</i18n>
