import { watch } from 'vue'
import { useAppConfigStore } from '@/store/useAppConfigStore'

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
