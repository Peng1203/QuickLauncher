import { AppEventName } from '@/constant'
import { emit, listen, UnlistenFn } from '@tauri-apps/api/event'

export const EventBus = {
  // 触发事件
  emit: async <T = unknown>(event: AppEventName, payload?: T) => {
    await emit(event, payload)
  },

  // 监听事件
  listen: async <T = unknown>(
    event: AppEventName,
    handler: (payload: T) => void
  ): Promise<UnlistenFn> => {
    return listen<T>(event, e => handler(e.payload))
  },
}
