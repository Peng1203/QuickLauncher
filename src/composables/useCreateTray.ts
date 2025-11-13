import { Menu } from '@tauri-apps/api/menu';
import { TrayIcon } from '@tauri-apps/api/tray';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { exit } from '@tauri-apps/plugin-process';

async function createMenu() {
  return await Menu.new({
    // items 的显示顺序是倒过来的
    items: [
      {
        // 菜单 id
        id: 'quit',
        // 菜单文本
        text: '退 出',
        // icon: 'icons/32x32.png',
        // 菜单事件处理程序
        action: () => exit(0),
      },
    ],
  });
}

export async function useCreateTray() {
  const handleShowMainWindow = async () => {
    const mainWindow = await WebviewWindow.getByLabel('main');
    mainWindow?.show();
    mainWindow?.setFocus();
  };

  await TrayIcon.new({
    title: 'Tauri App',
    tooltip: 'Quick Launcher',
    // icon: 'icons/tray.ico',
    icon: 'icons/32x32.png',
    menu: await createMenu(),
    action: e => {
      switch (e.type) {
        case 'Click':
          e.button === 'Left' &&
          e.buttonState === 'Up' &&
          handleShowMainWindow();
          break;
        case 'DoubleClick':
          break;
      }
    },
  });
}
