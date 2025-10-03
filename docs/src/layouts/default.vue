<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<template>
  <v-app>
    <v-app-bar color="primary">
      <v-app-bar-title>{{ t('header.title') }}</v-app-bar-title>
      <v-spacer />
      <v-btn to="/" variant="text">{{ t('header.home') }}</v-btn>
      <v-btn to="/test" variant="text">{{ t('header.test') }}</v-btn>
      <LanguageSelector />
    </v-app-bar>

    <v-main>
      <v-container fluid>
        <slot />
      </v-container>
    </v-main>

    <v-footer class="bg-grey-lighten-1">
      <div class="text-center w-100">
        <span>{{ t('footer.copyright') }}</span>
      </div>
    </v-footer>
  </v-app>
</template>
