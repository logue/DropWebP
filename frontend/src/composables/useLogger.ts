import { useGlobalStore } from '@/store';
import { onMounted } from 'vue';

import { listen } from '@tauri-apps/api/event';

export function useLogger() {
  const globalStore = useGlobalStore();

  onMounted(async () => {
    // バックエンドからのログメッセージを受信
    await listen('log-message', event => {
      const logData = event.payload as { level: string; message: string; timestamp: string };
      console.log(`[${logData.level.toUpperCase()}] ${logData.message} (${logData.timestamp})`);

      // 必要に応じてUIに表示（例：スナックバーで通知）
      if (logData.level === 'info') {
        globalStore.setMessage(logData.message);
      } else if (logData.level === 'error') {
        globalStore.setMessage(logData.message, 'red');
      }
    });
  });
}
