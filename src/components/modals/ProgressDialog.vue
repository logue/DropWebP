<script setup lang="ts">
import type { PropType } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps({
  dialog: { type: Boolean, default: false },
  title: { type: String, required: true },
  currentFile: { type: String, default: '' },
  progress: {
    type: Number as PropType<number>,
    required: false,
    default: 0
  },
  inProgress: { type: Boolean, default: false }
});

const emit = defineEmits<{
  (e: 'update:dialog', value: boolean): void;
  (e: 'update:inProgress', value: boolean): void;
}>();
</script>

<template>
  <v-dialog
    :model-value="dialog"
    persistent
    style="cursor: wait"
    width="auto"
    @update:model-value="emit('update:dialog', $event)"
  >
    <v-card width="512" prepend-icon="mdi-arrow-collapse-vertical" :title="title">
      <template #actions>
        <v-btn class="ms-auto" :text="t('cancel')" @click="emit('update:inProgress', false)" />
      </template>
      <v-card-text>
        {{ currentFile }}
        <v-progress-linear
          :indeterminate="progress === 0"
          :model-value="progress"
          color="primary"
          height="25"
        >
          <template #default="{ value }">
            <strong v-if="progress">{{ Math.ceil(value) }}%</strong>
          </template>
        </v-progress-linear>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<i18n lang="yaml">
en:
  cancel: Cancel
ja:
  cancel: キャンセル
</i18n>
