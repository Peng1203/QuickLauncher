import { defaultWindowIcon } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu } from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exit } from "@tauri-apps/plugin-process";
import { t } from "@/i18n";
import { useAppConfig } from "./useAppConfig";
import { useTheme } from "./useTheme";
import { useToggleWindowVisible } from "./useToggleWindowVisible";

const languageOptions: { label: string; value: LanguageType }[] = [
  { label: "简体中文", value: "zh-CN" },
  { label: "繁體中文", value: "zh-HK" },
  { label: "English", value: "en" },
  { label: "日本語", value: "ja" },
];

const themeOptions: { value: ThemeModel }[] = [
  { value: "light" },
  { value: "dark" },
  { value: "system" },
];

function getThemeLabel(value: ThemeModel) {
  const map: Record<ThemeModel, string> = {
    light: t("tray.themeLight"),
    dark: t("tray.themeDark"),
    system: t("tray.themeSystem"),
  };
  return map[value];
}

export function useTray() {
  const { appConfigStore } = useAppConfig();
  const { prefersDark, setThemeModel, setHTMLThemeClass } = useTheme();
  const { toogleMainWindowVisible, toogleSettingWindowVisible } = useToggleWindowVisible();

  async function handleSwitchTheme(newTheme: ThemeModel) {
    if (appConfigStore.themeModel === newTheme) return;

    let toTheme = newTheme;
    if (newTheme === "system") {
      toTheme = prefersDark.value ? "dark" : "light";
    }
    await setThemeModel(newTheme);
    setHTMLThemeClass(toTheme);
    void createTray();
  }

  async function buildLanguageSubmenu() {
    const items = await Promise.all(
      languageOptions.map((opt) =>
        CheckMenuItem.new({
          id: `lang_${opt.value}`,
          text: opt.label,
          checked: appConfigStore.language === opt.value,
          action: () => {
            appConfigStore.language = opt.value;
            void createTray();
          },
        }),
      ),
    );
    return await Submenu.new({ id: "language", text: t("tray.language"), items });
  }

  async function buildThemeSubmenu() {
    const items = await Promise.all(
      themeOptions.map((opt) =>
        CheckMenuItem.new({
          id: `theme_${opt.value}`,
          text: getThemeLabel(opt.value),
          checked: appConfigStore.themeModel === opt.value,
          action: () => void handleSwitchTheme(opt.value),
        }),
      ),
    );
    return await Submenu.new({ id: "theme", text: t("tray.theme"), items });
  }

  async function createTray() {
    const currentWindow = getCurrentWebviewWindow();
    const isMainWindow = currentWindow?.label === "main";
    // 只在主窗口创建一次托盘菜单
    if (!isMainWindow) return;

    const languageSubmenu = await buildLanguageSubmenu();
    const themeSubmenu = await buildThemeSubmenu();

    const menu = await Menu.new({
      items: [
        await MenuItem.new({
          id: "settings",
          text: t("tray.settings"),
          action: () => toogleSettingWindowVisible(false, true),
        }),
        languageSubmenu,
        themeSubmenu,
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
