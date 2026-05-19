import { useDebounceFn, useEventListener } from '@vueuse/core';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';

export function setupGlobalEvents() {
  const emitClose = useDebounceFn(() => {
    EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);
  }, 15);

  useEventListener(window, 'scroll', emitClose, { capture: true });
}
