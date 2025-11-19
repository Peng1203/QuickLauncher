import { disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { useAppConfig } from './useAppConfig';
import { useAppConfigActions } from './useAppConfigActions';

export async function useLoadConfig() {
  const { autoStart, saveFlag } = useAppConfig();
  // 初始化加载时立即保存一遍数据 防止后端获取时 拿不到新添加的属性
  // await $tauri.saveAllNow();
  setTimeout(() => (saveFlag.value = !saveFlag.value), 100);

  const {
    setAlwaysOnTop,
    setMainWindowCenter,
    setAutoStart,
    setSilentStart,
    setMainWindowPosition,
  } = useAppConfigActions();

  // 获取 自启动的最新状态 防止任务管理器中被关闭 导致展示状态错误
  const isEnaAffter = await isEnabled();
  // 当被系统禁用时 关闭自启动
  if (!isEnaAffter) await disable().catch(() => '');
  autoStart.value = isEnaAffter;

  setMainWindowPosition();

  // 设置窗口是否置顶、居中、静默启动、开机自启 等配置
  setAlwaysOnTop();
  setMainWindowCenter();
  setSilentStart();
  setAutoStart();
}
