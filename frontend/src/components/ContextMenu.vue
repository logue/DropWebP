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

/**
 * Emit the paste event to the parent and dismiss the context menu.
 */
const onPaste = () => {
  emit('paste');
  emit('update:show', false);
};

const shortcut = ref<string>('');
onMounted(() => {
  shortcut.value = platform() === 'macos' ? '⌘' : 'Ctrl+';
});
</script>

<template>
  <v-menu
    :model-value="show"
    :style="{ left: x + 'px', top: y + 'px' }"
    density="compact"
    absolute
    @update:model-value="emit('update:show', $event)"
  >
    <v-list>
      <v-list-item :title="t('paste')" prepend-icon="mdi-content-paste" @click="onPaste">
        <!-- eslint-disable-next-line @intlify/vue-i18n/no-raw-text -- 'V' is the keyboard shortcut letter, not localizable text -->
        <template #append>{{ shortcut }}V</template>
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
