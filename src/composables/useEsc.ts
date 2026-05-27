// useEsc.ts
import { onMounted, onUnmounted } from 'vue';

export function useEsc(callback: () => void) {
  const handler = (e: KeyboardEvent) => {
    if (e.key === 'Escape') callback();
  };

  onMounted(() => window.addEventListener('keydown', handler));
  onUnmounted(() => window.removeEventListener('keydown', handler));
}
