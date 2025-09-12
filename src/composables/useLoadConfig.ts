import { useAppConfigStore } from '@/store/useAppConfigStore'
import { Pinia } from 'pinia'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart'
import { getAppConfig, setAppConfig } from '@/api'
import { LogicalPosition } from '@tauri-apps/api/window'

export const useLoadConfig = async (store: Pinia) => {
  const appConfigStore = useAppConfigStore(store)

  // 获取数据库中的应用配置数据
  const data = await getAppConfig()
  // 判断是否有配置数据
  const hasConfig = !!Object.keys(data)?.length
  // 如果没有配置数据 则使用 store 中的默认配置
  // 如果有配置数据 则将数据库中的配置和 store 中的对比合并
  const config: AppConfigState = hasConfig ? data : appConfigStore.$state

  // console.log('config', JSON.parse(JSON.stringify(config)))

  // 获取 自启动的最新状态 防止任务管理器中被关闭 导致展示状态错误
  const isEnaAffter = await isEnabled()
  // 当被系统禁用时 关闭自启动
  if (!isEnaAffter) await disable().catch(() => '')
  config.autoStart = isEnaAffter

  appConfigStore.loadConfig(config)
  // rust端加载 配置文件数据

  setAppConfig(config)

  // event.emit('load_app_config', config)

  // 当返回空对象 表示第一次加载 则将初始化的配置保存到数据库中
  if (!hasConfig) appConfigStore.saveConfig()

  const mainWindow = await WebviewWindow.getByLabel('main')
  const x = config.mainWindowPositionX > 0 ? config.mainWindowPositionX : 0
  const y = config.mainWindowPositionY > 0 ? config.mainWindowPositionY : 0
  // 设置窗口位置
  if (x || y) {
    mainWindow?.setPosition(new LogicalPosition(x, y))
  }

  // 设置窗口是否置顶、居中、静默启动、开机自启 等配置
  mainWindow?.setAlwaysOnTop(config.onTop)
  config.center && mainWindow?.center()
  config.silentStart ? mainWindow?.hide() : mainWindow?.show()

  const isEna = await isEnabled()
  if (!isEna && config.autoStart) await enable()
  else if (isEna && !config.autoStart) await disable()
}
