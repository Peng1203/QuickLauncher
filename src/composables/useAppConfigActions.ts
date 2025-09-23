import { useAppConfigStore } from '@/store/useAppConfigStore'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'

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
      appConfigStore.autoStart
      const isEna = await isEnabled()
      if (!isEna && appConfigStore.autoStart) await enable()
      else if (isEna && !appConfigStore.autoStart) await disable()
    })

  return {
    setAlwaysOnTop,
    setWindowCenter,
    setAutoStart,
  }
}
