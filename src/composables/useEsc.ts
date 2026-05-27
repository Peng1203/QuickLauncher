// useEsc.ts
import { onMounted, onBeforeUnmount } from 'vue';

export function useEsc(callback: () => void) {
  const handler = (e: KeyboardEvent) => {
    if (e.key === 'Escape') callback();
  };

  onMounted(() => window.addEventListener('keydown', handler));
  onBeforeUnmount(() => window.removeEventListener('keydown', handler));
}
