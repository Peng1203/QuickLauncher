import { AppEvent } from '@/constant'
import { useAppConfigStore } from '@/store/useAppConfigStore'
import { emit, listen } from '@tauri-apps/api/event'
// import store from '@/store'

/**
 * 监听 appConfig 中数据变化 用于通知其他窗口更新数据 一用在窗口对应的页面组件中引入一次即可 子组件中无需引入 避免重复执行
 * @author Peng
 */
export const useUpdateAppConfig = () => {
  const appConfigStore = useAppConfigStore()

  watch(
    appConfigStore.$state,
    val => {
      emit(AppEvent.UPDATE_APP_CONFIG_DATA, val)
    },
    { deep: true }
  )

  listen<AppConfigState>(AppEvent.UPDATE_APP_CONFIG_DATA, val => {
    for (const key in val.payload) {
      // @ts-ignore
      appConfigStore[key] = val.payload[key]
    }
  })
}
