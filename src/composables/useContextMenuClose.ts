import { onMounted, onUnmounted } from 'vue';
import { EventBus } from '@/utils/eventBus';
import { AppEvent } from '@/constant';

export function useContextMenuClose(handleClose: () => void, handleOutsideClick?: (e: MouseEvent) => void) {
  onMounted(() => {
    EventBus.listen(AppEvent.CLOSE_CONTEXT_MENU, handleClose);

    handleOutsideClick && window.addEventListener('click', handleOutsideClick);
    handleOutsideClick && window.addEventListener('contextmenu', handleOutsideClick);
  });

  onUnmounted(() => {
    handleOutsideClick && window.removeEventListener('click', handleOutsideClick);
    handleOutsideClick && window.removeEventListener('contextmenu', handleOutsideClick);
  });
}
