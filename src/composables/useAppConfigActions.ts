import { useAppConfigStore } from '@/store/useAppConfigStore'
import { unRegisterShortcutKey } from '@/utils/shortcutKey'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { isRegistered, register } from '@tauri-apps/plugin-global-shortcut'

let mainWindow: WebviewWindow | null = null

export const useAppConfigActions = () => {
  const getMainWindow = async () => {
    return mainWindow ? mainWindow : await WebviewWindow.getByLabel('main')
  }

  const appConfigStore = useAppConfigStore()

  const setAlwaysOnTop = () =>
    nextTick(async () => {
      const mainWindow = await getMainWindow()
      mainWindow?.setAlwaysOnTop(appConfigStore.onTop)
    })

  const setWindowCenter = () =>
    nextTick(async () => {
      const mainWindow = await getMainWindow()
      appConfigStore.center && mainWindow?.center()
    })

  const setAutoStart = () =>
    nextTick(async () => {
      const isEna = await isEnabled()
      if (!isEna && appConfigStore.autoStart) await enable()
      else if (isEna && !appConfigStore.autoStart) await disable()
    })

  const setMainWindowShortcutKey = (shortcutKey: string) =>
    nextTick(async () => {
      if (!shortcutKey.trim()) return
      await register(shortcutKey, async e => {
        const mainWindow = await getMainWindow()

        const visible = await mainWindow?.isVisible()

        if (e.state === 'Released') {
          visible ? mainWindow?.hide() : mainWindow?.show()
        }
      })
    })

  const initMainWindowShortcutKey = async () => {
    if (appConfigStore.mainWindowGlobalShortcutKey) {
      // 注册之前先取消之前注册的快捷键
      const isReg = await isRegistered(appConfigStore.mainWindowGlobalShortcutKey)
      if (isReg)
        await unRegisterShortcutKey(appConfigStore.mainWindowGlobalShortcutKey).catch(() => '')
    }
    setMainWindowShortcutKey(appConfigStore.mainWindowGlobalShortcutKey)
  }

  return {
    setAlwaysOnTop,
    setWindowCenter,
    setAutoStart,
    setMainWindowShortcutKey,
    initMainWindowShortcutKey,
  }
}
