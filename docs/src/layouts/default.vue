<script setup lang="ts">
import { useTheme } from 'vuetify';

import logo from '../assets/favicon.ico';
import { useConfigStore, useGlobalStore } from '../store';

/** Vuetify Theme */
const theme = useTheme();

/** Global Store */
const globalStore = useGlobalStore();

/** Config Store */
const configStore = useConfigStore();

/** Title */
const title = 'Drop Compress Image';

/** drawer visibility */
const drawer: Ref<boolean> = ref(false);

/** loading overlay visibility */
const loading: WritableComputedRef<boolean> = computed({
  get: () => globalStore.loading,
  set: v => globalStore.setLoading(v)
});

/** Appbar progressbar value */
const progress: ComputedRef<number | null> = computed(() => globalStore.progress);

/** Snackbar visibility */
const snackbarVisibility: Ref<boolean> = ref(false);

/** Snackbar text */
const snackbarText: ComputedRef<string> = computed(() => globalStore.message);

/** Toggle Dark mode */
const isDark: ComputedRef<string> = computed(() => (configStore.theme ? 'dark' : 'light'));

// When snackbar text has been set, show snackbar.
watch(
  () => globalStore.message,
  message => (snackbarVisibility.value = message !== '')
);

/** Clear store when snackbar hide */
const onSnackbarChanged = async () => {
  globalStore.setMessage();
  await nextTick();
};

onMounted(() => {
  document.title = title;
  loading.value = false;
});
</script>

<template>
  <v-app :theme="isDark">
    <v-navigation-drawer v-model="drawer" temporary>
      <drawer-menu />
    </v-navigation-drawer>

    <v-app-bar color="primary">
      <v-app-bar-nav-icon @click="drawer = !drawer" />
      <v-app-bar-title>Drop Compress Image</v-app-bar-title>
      <v-spacer />
      <app-bar-menu />
      <v-progress-linear
        v-show="loading"
        :active="loading"
        :indeterminate="progress === null"
        :model-value="progress !== null ? progress : 0"
        color="blue-accent-3"
      />
    </v-app-bar>

    <v-main>
      <v-container>
        <slot />
      </v-container>
    </v-main>

    <v-overlay v-model="loading" app class="justify-center align-center" persistent>
      <v-progress-circular indeterminate size="64" />
    </v-overlay>

    <v-snackbar v-model="snackbarVisibility" @update:model-value="onSnackbarChanged">
      {{ snackbarText }}
      <template #actions>
        <v-btn icon="mdi-close" @click="onSnackbarChanged" />
      </template>
    </v-snackbar>

    <v-footer app elevation="3">
      <span class="mr-5">2025 &copy; Logue</span>
    </v-footer>
  </v-app>
  <teleport to="head">
    <meta
      name="theme-color"
      :content="theme.computedThemes.value?.[isDark]?.colors?.primary ?? '#1976D2'"
    />
    <link rel="icon" :href="logo" type="image/x-icon" />
  </teleport>
</template>
