import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import { debounce } from 'lodash-es';

export const setupGlobalEvents = () => {
  const isDev = import.meta.env.DEV;

  // 触发关闭contextMenu事件
  window.addEventListener(
    'scroll',
    debounce(() => {
      EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);
    }, 15),
    { capture: true },
  );

  if (!isDev) {
    // 生产环境禁止鼠标右键
    window.addEventListener('contextmenu', e => e.preventDefault());

    window.addEventListener('keydown', e => {
      if (e.key === 'F5' || (e.ctrlKey && e.key.toLowerCase() === 'r')) {
        e.preventDefault();
      }
    });

    window.addEventListener('keydown', e => {
      if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 'i') {
        e.preventDefault();
      }
    });
  }
};
