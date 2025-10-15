import { onMounted, ref } from 'vue';

import { listen } from '@tauri-apps/api/event';

export function useDragAndDrop(callback: (paths: string[]) => Promise<void>) {
  const isDragging = ref(false);

  onMounted(async () => {
    // ドラッグホバー開始
    listen('tauri://drag-enter', () => {
      isDragging.value = true;
    });

    // ドラッグホバー終了
    listen('tauri://drag-leave', () => {
      isDragging.value = false;
    });

    // ドロップイベント
    listen('tauri://drag-drop', async e => {
      isDragging.value = false;
      const inputs = (e.payload as { paths: string[] }).paths;
      await callback(inputs);
    });
  });

  return {
    isDragging
  };
}
