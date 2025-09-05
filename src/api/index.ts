import { invoke } from '@tauri-apps/api/core'
import { InvokeMethod } from '@/constant'
import { createDiscreteApi } from 'naive-ui'

const { message } = createDiscreteApi(['message'])

/**
 * 打开文件所在文件
 * @author Peng
 *
 * @param {string} path
 * @returns {*}
 */
export const openPath = (path: string) =>
  invoke(InvokeMethod.OPEN_PATH, { path }).catch(e => message.error(e))

/**
 * 获取App配置信息
 * @author Peng
 *
 * @template [T=AppConfigState]
 * @returns {*}
 */
export const getAppConfig = <T = AppConfigState>() => invoke<T>(InvokeMethod.GET_APP_CONFIG)

/**
 * 保存App配置信息
 * @author Peng
 *
 * @template [T=AppConfigState]
 * @param {T} configData
 * @returns {*}
 */
export const saveAppConfig = <T = AppConfigState>(configData: T) =>
  invoke(InvokeMethod.SAVE_APP_CONFIG, {
    config: {
      name: 'appConfig',
      data: JSON.stringify(configData),
    },
  })

/**
 * 运行启动项
 * @author Peng
 *
 * @param {number} id
 * @returns {*}
 */
export const runLaunch = (id: number) => invoke(InvokeMethod.RUN_LAUNCH, { id })

/**
 * 以管理员身份运行启动项
 * @author Peng
 *
 * @param {number} id
 * @returns {*}
 */
export const runLaunchAsAdmin = (id: number) => invoke(InvokeMethod.RUN_LAUNCH_AS_ADMIN, { id })

/**
 * 获取文件信息
 * @author Peng
 *
 * @template [T=FileInfo]
 * @param {string} path
 * @returns {*}
 */
export const getFileInfo = <T = FileInfo>(path: string) =>
  invoke<T>(InvokeMethod.GET_FILE_INFO, { path })

/**
 * 添加启动项
 * @author Peng
 *
 * @template [T=NewLaunchItem]
 * @param {T} item
 * @returns {*}
 */
export const addLaunch = <T = NewLaunchItem>(item: T) => invoke(InvokeMethod.ADD_LAUNCH, { item })

/**
 * 更新启动项
 * @author Peng
 *
 * @template [T=LaunchItem]
 * @param {T} item
 * @returns {*}
 */
export const updateLaunch = <T = LaunchItem>(item: T) =>
  invoke(InvokeMethod.UPDATE_LAUNCH, { item })

/**
 * 获取启动项
 * @author Peng
 *
 * @template [T=NewLaunchItem]
 * @param {T} item
 * @returns {*}
 */
export const getLaunchs = <T = LaunchItem>(categoryId: number = -1) =>
  invoke<T[]>(InvokeMethod.GET_LAUNCH, { categoryId })

/**
 * 执行命令行
 * @author Peng
 *
 * @param {string} cmd
 * @returns {*}
 */
export const exeCommand = (cmd: string) => invoke(InvokeMethod.EXE_COMMAND, { cmd })

/**
 * 根据关键字查询启动项
 * @author Peng
 *
 * @param {string} keyword
 * @returns {*}
 */
export const searchLaunch = <T = SearchLauncItem>(keyword: string) =>
  invoke<T[]>(InvokeMethod.SEARCH_LAUNCH, { keyword })

/**
 * 启动项重命名
 * @author Peng
 *
 * @param {number} id
 * @param {string} name
 * @returns {*}
 */
export const renameLaunch = (id: number, name: string) =>
  invoke(InvokeMethod.RENAME_LAUNCH, { id, name })

/**
 * 删除启动项
 * @author Peng
 *
 * @param {number} id
 * @returns {*}
 */
export const deleteLaunch = (id: number) => invoke(InvokeMethod.DELETE_LAUNCH, { id })

/**
 * 获取网站信息
 * @author Peng
 *
 * @param {string} url
 * @returns {*}
 */
export const getWebsiteInfo = (url: string) => invoke(InvokeMethod.GET_WEBSITE_INFO, { url })

/**
 * 新建分类
 * @author Peng
 *
 * @template [T=NewCategoryItem]
 * @param {T} item
 * @returns {*}
 */
export const addCategory = <T = NewCategoryItem>(item: T) =>
  invoke(InvokeMethod.ADD_CATEGORY, { item })

/**
 * 获取分类
 * @author Peng
 *
 * @returns {*}
 */
export const getCategory = () => invoke<CategoryItem[]>(InvokeMethod.GET_CATEGORY)
