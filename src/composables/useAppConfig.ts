import store from '@/store';
import { useAppConfigStore } from '@/store/useAppConfigStore';
import { storeToRefs } from 'pinia';

/**
 * 加载 App配置 修改时自动保存
 */
export const useAppConfig = () => {
  const appConfigStore = useAppConfigStore(store);
  const state = storeToRefs(appConfigStore);

  return {
    appConfigStore,
    ...appConfigStore,
    ...state,
  };
};
