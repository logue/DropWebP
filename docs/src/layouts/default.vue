<script setup lang="ts">
import { useTheme } from 'vuetify';

import { useConfigStore } from '../store';

/** Vuetify Theme */
const theme = useTheme();

/** Config Store */
const configStore = useConfigStore();

/** drawer visibility */
const drawer: Ref<boolean> = ref(false);

/** Toggle Dark mode */
const isDark: ComputedRef<string> = computed(() => (configStore.theme ? 'dark' : 'light'));
</script>

<template>
  <v-app :theme="isDark">
    <v-navigation-drawer v-model="drawer" permanent>
      <drawer-menu />
    </v-navigation-drawer>

    <v-app-bar color="primary">
      <v-app-bar-nav-icon @click="drawer = !drawer" />
      <v-app-bar-title>Drop Compress Image</v-app-bar-title>
      <v-spacer />
      <app-bar-menu-component />
    </v-app-bar>

    <v-main>
      <v-container>
        <slot />
      </v-container>
    </v-main>

    <v-footer app elevation="3" color="primary">
      <span class="mr-5">2025-2026 &copy; Logue</span>
    </v-footer>
  </v-app>
  <teleport to="head">
    <meta
      name="theme-color"
      :content="theme.computedThemes.value?.[isDark]?.colors?.primary ?? '#1976D2'"
    />
  </teleport>
</template>
