import { watch } from 'vue'
import { useAppConfigStore } from '@/store/useAppConfigStore'
import { storeToRefs } from 'pinia'

let unWatch: any

/**
 * 加载 App配置 修改时自动保存
 */
export const useAppConfig = () => {
  const appConfigStore = useAppConfigStore()
  const storeRef = storeToRefs(appConfigStore)

  // 保证只生成一次 watch
  if (!unWatch) {
    unWatch = watch(
      appConfigStore.$state,
      // 每次配置变化都保存到数据库中
      _val => appConfigStore.saveConfig(),
      { deep: true }
    )
  }

  return {
    appConfigStore,
    storeRef,
  }
}
