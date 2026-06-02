import type { UnlistenFn } from "@tauri-apps/api/event";
import type { AppEventName } from "@/constant";
import { emit, listen } from "@tauri-apps/api/event";

export const EventBus = {
  // 触发事件
  emit: <T = unknown>(event: AppEventName, payload?: T) => emit(event, payload),

  // 监听事件
  listen: <T = unknown>(
    event: AppEventName,
    handler: (payload: T) => void,
  ): Promise<UnlistenFn> => {
    return listen<T>(event, (e) => handler(e.payload));
  },
};
