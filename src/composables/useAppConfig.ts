import { watch } from 'vue'
import { useAppConfigStore } from '@/store/useAppConfigStore'

/**
 * 加载 App配置 修改时自动保存
 */
export const useAppConfig = () => {
  const appConfigStore = useAppConfigStore()

  watch(
    appConfigStore.$state,
    // 每次配置变化都保存到数据库中
    () => appConfigStore.saveConfig(),
    { deep: true }
  )

  return {
    appConfigStore,
  }
}
