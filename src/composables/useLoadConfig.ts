import { disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { useAppConfig } from './useAppConfig';
import { useAppConfigActions } from './useAppConfigActions';
import { getAppConfig } from '@/api';
import { delay } from '@/utils/delay';

export async function useLoadConfig() {
  const retryCounter = ref(0);
  const { appConfigStore } = useAppConfig();

  async function initData() {
    try {
      if (retryCounter.value >= 20) return;
      await delay(50);
      await getAppConfig().then(dbAppConfig => {
        // 先从数据库中读取一遍配置 并将其加载到 store中
        if (dbAppConfig) appConfigStore.$state = dbAppConfig;
        setAppConfig();
      });
    } catch (e) {
      console.log('initData', e);
      retryCounter.value++;
      await initData();
    }
  }

  async function setAppConfig() {
    const { autoStart, saveFlag } = useAppConfig();
    // 初始化加载时立即保存一遍数据 防止后端获取时 拿不到新添加的属性
    setTimeout(() => (saveFlag.value = !saveFlag.value), 100);

    const {
      setAlwaysOnTop,
      setAutoStart,
      setSilentStart,
      setMainWindowTitle,
      setMainWindowCenter,
      setMainWindowPosition,
    } = useAppConfigActions();

    // 获取 自启动的最新状态 防止任务管理器中被关闭 导致展示状态错误
    const isEnaAffter = await isEnabled();
    // 当被系统禁用时 关闭自启动
    if (!isEnaAffter) await disable().catch(() => '');
    autoStart.value = isEnaAffter;

    setMainWindowTitle();
    setMainWindowPosition();

    // 设置窗口是否置顶、居中、静默启动、开机自启 等配置
    setAlwaysOnTop();
    setMainWindowCenter();
    setSilentStart();
    setAutoStart();
  }

  initData();

  // await $tauri.saveAllNow();
}
