import { defaultWindowIcon } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { Menu, MenuItem, PredefinedMenuItem } from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exit } from "@tauri-apps/plugin-process";
import { t } from "@/i18n";
import { useAppConfig } from "./useAppConfig";
import { useToggleWindowVisible } from "./useToggleWindowVisible";

export function useTray() {
  const { appConfigStore } = useAppConfig();
  const { toogleMainWindowVisible, toogleSettingWindowVisible } = useToggleWindowVisible();

  async function createTray() {
    const currentWindow = getCurrentWebviewWindow();
    const isMainWindow = currentWindow?.label === "main";
    // 只在主窗口创建一次托盘菜单
    if (!isMainWindow) return;
    const menu = await Menu.new({
      items: [
        await MenuItem.new({
          id: "settings",
          text: t("tray.settings"),
          action: () => toogleSettingWindowVisible(false, true),
        }),
        await PredefinedMenuItem.new({ item: "Separator" }),
        await MenuItem.new({
          id: "restart",
          text: t("tray.restart"),
          action: () => invoke("restart_app"),
        }),
        await MenuItem.new({ id: "quit", text: t("tray.quit"), action: () => exit(0) }),
      ],
    });

    const existing = await TrayIcon.getById("tray");
    if (existing) {
      await existing.setMenu(menu);
      await existing.setTooltip(appConfigStore.title || "Quick Launcher");
      return;
    }
    const icon = await defaultWindowIcon();

    await TrayIcon.new({
      id: "tray",
      icon: icon ?? undefined,
      tooltip: appConfigStore.title || "Quick Launcher",
      menu,
      menuOnLeftClick: false,
      showMenuOnLeftClick: false,
      action: (event) => {
        switch (event.type) {
          case "Click":
            if (event.button === "Left" && event.buttonState === "Up") {
              void toogleMainWindowVisible();
            }
            break;
        }
      },
    });
  }

  return { createTray };
}
