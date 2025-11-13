import {
  getCurrentWebviewWindow,
  WebviewWindow,
} from '@tauri-apps/api/webviewWindow';
import { getAppConfig, setAppConfig } from '@/api';
import { useAppConfig } from './useAppConfig';
import { disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { LogicalPosition } from '@tauri-apps/api/dpi';
import { useAppConfigActions } from './useAppConfigActions';

export const useLoadConfig = async () => {
  const currentWindow = getCurrentWebviewWindow();

  const {
    loadConfig,
    saveConfig,
    autoStart,
    silentStart,
    mainWindowPositionX,
    mainWindowPositionY,
  } = useAppConfig();

  // 获取数据库中的应用配置数据
  const data = await getAppConfig().catch(() => ({}));

  // 判断是否有配置数据
  const hasConfig = !!Object.keys(data)?.length;

  // 如果没有配置数据 则使用 store 中的默认配置
  // 如果有配置数据 则将数据库中的配置和 store 中的对比合并
  // currentWindow.label === 'main' &&
  hasConfig && loadConfig(data as AppConfigState);

  // 获取 自启动的最新状态 防止任务管理器中被关闭 导致展示状态错误
  const isEnaAffter = await isEnabled();
  // 当被系统禁用时 关闭自启动
  if (!isEnaAffter) await disable().catch(() => '');
  autoStart.value = isEnaAffter;

  // TODO rust端加载 配置文件数据
  // setAppConfig($state);

  // // 当返回空对象 表示第一次加载 则将初始化的配置保存到数据库中
  if (!hasConfig) saveConfig();

  const mainWindow = await WebviewWindow.getByLabel('main');
  const x = mainWindowPositionX.value > 0 ? mainWindowPositionX.value : 0;
  const y = mainWindowPositionY.value > 0 ? mainWindowPositionY.value : 0;
  // 设置窗口位置
  if (x || y) {
    mainWindow?.setPosition(new LogicalPosition(x, y));
  }

  // 设置窗口是否置顶、居中、静默启动、开机自启 等配置
  // mainWindow?.setAlwaysOnTop(config.onTop)
  const { setAlwaysOnTop, setWindowCenter, setAutoStart } =
    useAppConfigActions();

  setAlwaysOnTop();
  setWindowCenter();
  // config.center && mainWindow?.center()
  silentStart.value ? mainWindow?.hide() : mainWindow?.show();
  setAutoStart();
};
