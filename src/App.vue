<script setup lang="ts">
import { useGlobalStore, useConfigStore } from '@/store';
import {
  computed,
  nextTick,
  onMounted,
  ref,
  watch,
  type ComputedRef,
  type Ref,
  type WritableComputedRef
} from 'vue';

// Components
import AppBarMenuComponent from '@/components/AppBarMenuComponent.vue';
import MainContent from '@/components/MainContent.vue';

/** Global Store */
const globalStore = useGlobalStore();

/** Config Store */
const configStore = useConfigStore();

/** Title */
const title = import.meta.env.VITE_APP_TITLE ?? 'DropWebP';

/** loading overlay visibility */
const loading: WritableComputedRef<boolean> = computed({
  get: () => globalStore.loading,
  set: v => globalStore.setLoading(v)
});

/** Appbar progressbar value */
const progress: WritableComputedRef<number | null> = computed({
  get: () => globalStore.progress,
  set: v => globalStore.setProgress(v)
});

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

onMounted(async () => {
  document.title = title;
  loading.value = false;
});
</script>

<template>
  <v-app :theme="isDark" data-tauri-drag-region="true">
    <v-app-bar color="primary">
      <v-app-bar-title tag="h1">{{ title }}</v-app-bar-title>
      <v-spacer />
      <app-bar-menu-component />
      <v-progress-linear
        v-show="loading"
        :active="loading"
        :indeterminate="progress === null"
        :model-value="progress !== null ? progress : 0"
        color="blue-accent-3"
      />
    </v-app-bar>

    <v-main>
      <main-content />
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

    <v-footer app elevation="3" color="primary">
      <span class="mr-5">2025 &copy; Logue</span>
    </v-footer>
  </v-app>
</template>

<style lang="scss">
/* stylelint-disable-next-line scss/load-no-partial-leading-underscore */
@use 'vuetify/_settings';
@use 'sass:map';

html {
  // Fix always scrollbar shown.
  overflow-y: auto;
  // Modern scrollbar style
  scrollbar-width: thin;
  scrollbar-color: map.get(settings.$grey, 'lighten-2') map.get(settings.$grey, 'base');
}

::-webkit-scrollbar {
  width: 0.5rem;
  height: 0.5rem;
}

::-webkit-scrollbar-track {
  box-shadow: inset 0 0 0.5rem rgba(0, 0, 0, 0.1);
  background-color: map.get(settings.$grey, 'lighten-2');
}

::-webkit-scrollbar-thumb {
  border-radius: 0.5rem;
  background-color: map.get(settings.$grey, 'base');
  box-shadow: inset 0 0 0.5rem rgba(0, 0, 0, 0.1);
}

// Fixed a bug that the theme color is interrupted when scrolling
.v-application {
  overflow-y: auto;
}

// Fix app-bar's progress-bar
.v-app-bar .v-progress-linear {
  position: absolute;
  bottom: 0;
}
</style>
