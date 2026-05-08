import { LogicalPosition } from '@tauri-apps/api/window';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { isRegistered, register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import { unRegisterShortcutKey } from '@/utils/shortcutKey';
import { useAppConfig } from './useAppConfig';
import { useToggleWindowVisible } from './useToggleWindowVisible';

export function useAppConfigActions() {
  const { appConfigStore, silentStart, mainWindowPositionX, mainWindowPositionY } = useAppConfig();
  const { getMainWindow, toogleMainWindowVisible } = useToggleWindowVisible();

  // const getMainWindow = async () => {
  //   return mainWindow || (await WebviewWindow.getByLabel('main'));
  // };

  const setAlwaysOnTop = () => {
    const { appConfigStore } = useAppConfig();

    nextTick(async () => {
      const mainWindow = await getMainWindow();
      mainWindow?.setAlwaysOnTop(appConfigStore.onTop);
    });
  };

  const setMainWindowPosition = async () => {
    const mainWindow = await getMainWindow();
    const x = mainWindowPositionX.value > 0 ? mainWindowPositionX.value : 0;
    const y = mainWindowPositionY.value > 0 ? mainWindowPositionY.value : 0;
    // 设置窗口位置
    if (x || y) {
      mainWindow?.setPosition(new LogicalPosition(x, y));
    }
  };

  const setMainWindowCenter = async () => {
    const mainWindow = await getMainWindow();
    appConfigStore.center && mainWindow?.center();
  };

  const setAutoStart = async () => {
    const isEna = await isEnabled();
    if (!isEna && appConfigStore.autoStart) await enable();
    else if (isEna && !appConfigStore.autoStart) await disable();
  };

  const setSilentStart = async () => {
    const mainWindow = await getMainWindow();
    silentStart.value ? mainWindow?.hide() : mainWindow?.show();
  };

  const registerMainWindowShortcutKey = async (shortcutKey: string) => {
    if (!shortcutKey.trim()) return;
    await register(shortcutKey, async e => {
      if (e.state === 'Released') {
        toogleMainWindowVisible();
      }
    });
  };

  const initMainWindowShortcutKey = async () => {
    // console.log(
    //   `%c appConfigStore.mainWindowGlobalShortcutKey ----`,
    //   'color: #fff;background-color: #000;font-size: 18px',
    //   appConfigStore.autoStart,
    //   appConfigStore.mainWindowGlobalShortcutKey
    // );

    if (appConfigStore.mainWindowGlobalShortcutKey) {
      // 注册之前先取消之前注册的快捷键
      const isReg = await isRegistered(appConfigStore.mainWindowGlobalShortcutKey);
      if (isReg) {
        await unRegisterShortcutKey(appConfigStore.mainWindowGlobalShortcutKey).catch(() => '');
      }
      registerMainWindowShortcutKey(appConfigStore.mainWindowGlobalShortcutKey);
    }
  };

  const registerSearchShortcutKey = async (key: string = appConfigStore.searchGlobalShortcutKey) => {
    // || 'Alt+Space'
    if (!key) return;
    const isReg = await isRegistered(key);
    isReg && (await unregister(key));

    register(key, async e => {
      if (e.state === 'Released') {
        EventBus.emit(AppEvent.SEARCH_SHORTCU_KEY);
      }
    });
  };

  return {
    setAlwaysOnTop,
    setMainWindowCenter,
    setMainWindowPosition,
    setAutoStart,
    setSilentStart,
    registerMainWindowShortcutKey,
    initMainWindowShortcutKey,
    registerSearchShortcutKey,
  };
}
