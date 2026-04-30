<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

/**
 * Progress dialog props.
 */
interface Props {
  /** Whether the dialog is shown. */
  dialog?: boolean;
  /** Dialog title. */
  title: string;
  /** Currently processed file path. */
  currentFile?: string;
  /** Progress percentage (0-100). */
  progress?: number;
  /** Whether processing is in progress. */
  inProgress?: boolean;
}

withDefaults(defineProps<Props>(), {
  dialog: false,
  currentFile: '',
  progress: 0,
  inProgress: false
});

const emit = defineEmits<{
  (e: 'update:dialog', value: boolean): void;
  (e: 'update:inProgress', value: boolean): void;
}>();
</script>

<template>
  <v-dialog
    :model-value="dialog"
    style="cursor: wait"
    width="auto"
    persistent
    @update:model-value="emit('update:dialog', $event)"
  >
    <v-card :title="title" width="512" prepend-icon="mdi-arrow-collapse-vertical">
      <template #actions>
        <v-btn :text="t('cancel')" class="ms-auto" @click="emit('update:inProgress', false)" />
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
            <!-- eslint-disable-next-line @intlify/vue-i18n/no-raw-text -- '%' is a universal symbol, not localizable text -->
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
fr:
  cancel: Annuler
ja:
  cancel: キャンセル
ko:
  cancel: 취소
zhHant:
  cancel: 取消
zhHans:
  cancel: 取消
</i18n>
