<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { platform } from '@tauri-apps/plugin-os';

interface Props {
  show: boolean;
  x: number;
  y: number;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'paste'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const { t } = useI18n();

const onPaste = () => {
  emit('paste');
  emit('update:show', false);
};

const platformName = ref<string>('');

onMounted(async () => {
  platformName.value = await platform();
});
</script>

<template>
  <v-menu
    :model-value="show"
    :style="{ left: x + 'px', top: y + 'px' }"
    absolute
    density="compact"
    @update:model-value="emit('update:show', $event)"
  >
    <v-list>
      <v-list-item prepend-icon="mdi-content-paste" @click="onPaste">
        <v-list-item-title>
          {{ t('paste') }}
        </v-list-item-title>
        <template #append>
          <template v-if="platformName === 'macos'">⌘V</template>
          <template v-else>Ctrl+V</template>
        </template>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<i18n lang="yaml">
en:
  paste: Paste
fr:
  paste: Coller
ja:
  paste: ペースト
ko:
  paste: 붙여넣기
zhHant:
  paste: 貼上
zhHans:
  paste: 粘贴
</i18n>
