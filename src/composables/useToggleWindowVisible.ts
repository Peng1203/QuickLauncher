import { LogicalPosition } from '@tauri-apps/api/dpi';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { useAppConfig } from './useAppConfig';

// 控制窗口 展示状态
export function useToggleWindowVisible() {
  const { appConfigStore } = useAppConfig();

  // main 窗口 800*600
  const getMainWindow = async () => await WebviewWindow.getByLabel('main');
  // search 窗口 600*(45~400)
  const getSearchWindow = async () => await WebviewWindow.getByLabel('search');
  // setting 窗口 600*500
  const getSettingWindow = async () => await WebviewWindow.getByLabel('setting');
  // 启动项操作窗口 600*400
  const getOperLaunchWindow = async () => await WebviewWindow.getByLabel('operLaunch');
  // 分类操作窗口 600*400
  const getOperCategoryWindow = async () => await WebviewWindow.getByLabel('operCategory');

  async function toogleMainWindowVisible() {
    const window = await getMainWindow();
    const visible = await window?.isVisible();
    const focus = await window?.isFocused();
    if (visible && !focus) {
      // 当窗口可见时 但不是在最顶层
      window?.setFocus();
    } else if (visible && focus) {
      // 当窗口可见是 且在最顶层时
      // 该情况只会在通过全局快捷键触发控制时出现
      window?.hide();
    } else {
      // 当窗口不可见时
      window?.setFocus();
      window?.show();
    }
  }

  async function toogleSettingWindowVisible(centerInMainWindow: boolean = true) {
    const window = await getSettingWindow();
    const visible = await window?.isVisible();
    const focus = await window?.isFocused();
    if (visible && !focus) {
      // 当窗口可见时 但不是在最顶层
      window?.setFocus();
      // 如果主窗口开启了置顶 则也同时开启置顶
      await window?.setAlwaysOnTop(appConfigStore.onTop);
    } else if (visible && focus) {
      // 当窗口可见是 且在最顶层时
      // 该情况只会在通过全局快捷键触发控制时出现
      window?.hide();
    } else {
      // 当窗口不可见时
      if (centerInMainWindow) {
        const x = appConfigStore.mainWindowPositionX + 100;
        const y = appConfigStore.mainWindowPositionY + 50;
        window?.setPosition(new LogicalPosition(x, y));
      }
      // 如果主窗口开启了置顶 则也同时开启置顶
      await window?.setAlwaysOnTop(appConfigStore.onTop);
      window?.show();
      window?.setFocus();
    }
  }

  async function toogleOperLaunchWindowVisible(centerInMainWindow: boolean = true) {
    const window = await getOperLaunchWindow();
    const visible = await window?.isVisible();
    const focus = await window?.isFocused();
    if (visible && !focus) {
      // 当窗口可见时 但不是在最顶层
      window?.setFocus();
      // 如果主窗口开启了置顶 则也同时开启置顶
      await window?.setAlwaysOnTop(appConfigStore.onTop);
    } else if (visible && focus) {
      // 当窗口可见是 且在最顶层时
      // 该情况只会在通过全局快捷键触发控制时出现
      window?.hide();
    } else {
      // 当窗口不可见时
      if (centerInMainWindow) {
        const x = appConfigStore.mainWindowPositionX + 100;
        const y = appConfigStore.mainWindowPositionY + 100;
        window?.setPosition(new LogicalPosition(x, y));
      }
      // 如果主窗口开启了置顶 则也同时开启置顶
      await window?.setAlwaysOnTop(appConfigStore.onTop);
      window?.show();
      window?.setFocus();
    }
  }

  async function toogleOperCategoryWindowVisible(centerInMainWindow: boolean = true) {
    const window = await getOperCategoryWindow();
    const visible = await window?.isVisible();
    const focus = await window?.isFocused();
    if (visible && !focus) {
      // 当窗口可见时 但不是在最顶层
      window?.setFocus();
      // 如果主窗口开启了置顶 则也同时开启置顶
      await window?.setAlwaysOnTop(appConfigStore.onTop);
    } else if (visible && focus) {
      // 当窗口可见是 且在最顶层时
      // 该情况只会在通过全局快捷键触发控制时出现
      window?.hide();
    } else {
      // 当窗口不可见时
      if (centerInMainWindow) {
        const x = appConfigStore.mainWindowPositionX + 100;
        const y = appConfigStore.mainWindowPositionY + 100;
        window?.setPosition(new LogicalPosition(x, y));
      }
      // 如果主窗口开启了置顶 则也同时开启置顶
      await window?.setAlwaysOnTop(appConfigStore.onTop);
      window?.show();
      window?.setFocus();
    }
  }

  return {
    getMainWindow,
    toogleMainWindowVisible,

    getSettingWindow,
    toogleSettingWindowVisible,

    getOperLaunchWindow,
    toogleOperLaunchWindowVisible,

    getOperCategoryWindow,
    toogleOperCategoryWindowVisible,

    getSearchWindow,
  };
}
