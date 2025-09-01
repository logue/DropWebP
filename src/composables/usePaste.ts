import { onMounted, onUnmounted } from 'vue';

// クリップボードからペーストイベントを監視
export function usePaste(handlePaste: (e: ClipboardEvent) => void) {
  onMounted(() => {
    window.addEventListener('paste', handlePaste);
  });

  onUnmounted(() => {
    window.removeEventListener('paste', handlePaste);
  });
}
