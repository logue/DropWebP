import { onMounted } from 'vue';

import { listen } from '@tauri-apps/api/event';

export function useDragAndDrop(callback: (paths: string[]) => Promise<void>) {
  onMounted(async () => {
    listen('tauri://drag-drop', async e => {
      const inputs = (e.payload as { paths: string[] }).paths;
      await callback(inputs);
    });
  });
}
