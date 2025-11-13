import { invoke } from '@tauri-apps/api/core';
import { createDiscreteApi } from 'naive-ui';
import { InvokeMethod } from '@/constant';

const { message } = createDiscreteApi(['message']);

/**
 * @description 打开文件所在文件
 * @author Peng
 *
 * @param {string} path
 * @returns {*}
 */
export function openPath(path: string) {
  return invoke(InvokeMethod.OPEN_PATH, { path }).catch(e => message.error(e));
}

/**
 * @description 获取App配置信息
 * @author Peng
 *
 * @template [T=AppConfigState]
 * @returns {*}
 */
export function getAppConfig<T = AppConfigState>() {
  return invoke<T>(InvokeMethod.GET_APP_CONFIG);
}

/**
 * @description 保存App配置信息
 * @author Peng
 *
 * @template [T=AppConfigState]
 * @param {T} configData
 * @returns {*}
 */
export function saveAppConfig<T = AppConfigState>(configData: T) {
  return invoke(InvokeMethod.SAVE_APP_CONFIG, {
    config: {
      name: 'appConfig',
      data: JSON.stringify(configData),
    },
  });
}

/**
 * @description 运行启动项
 * @author Peng
 *
 * @param {number} id
 * @returns {*}
 */
export function runLaunch(id: number) {
  return invoke(InvokeMethod.RUN_LAUNCH, { id });
}

/**
 * @description 以管理员身份运行启动项
 * @author Peng
 *
 * @param {number} id
 * @returns {*}
 */
export function runLaunchAsAdmin(id: number) {
  return invoke(InvokeMethod.RUN_LAUNCH_AS_ADMIN, { id });
}

/**
 * @description 获取文件信息
 * @author Peng
 *
 * @template [T=FileInfo]
 * @param {string} path
 * @returns {*}
 */
export function getFileInfo<T = FileInfo>(path: string) {
  return invoke<T>(InvokeMethod.GET_FILE_INFO, { path });
}

/**
 * @description 添加启动项
 * @author Peng
 *
 * @template [T=NewLaunchItem]
 * @param {T} item
 * @returns {*}
 */
export function addLaunch<T = NewLaunchItem>(item: T) {
  return invoke(InvokeMethod.ADD_LAUNCH, { item });
}

/**
 * @description 更新启动项
 * @author Peng
 *
 * @template [T=LaunchItem]
 * @param {T} item
 * @returns {*}
 */
export function updateLaunch<T = LaunchItem>(item: T) {
  return invoke(InvokeMethod.UPDATE_LAUNCH, { item });
}

/**
 * @description 获取启动项
 * @author Peng
 *
 * @template T=LaunchItem
 * @param {number} categoryId 分类 ID
 * @returns {Promise<T[]>} 启动项数组
 */
export function getLaunchs<T = LaunchItem>(categoryId: number = -1) {
  return invoke<T[]>(InvokeMethod.GET_LAUNCH, { categoryId });
}

/**
 * @description 执行命令行
 * @author Peng
 *
 * @param {string} cmd
 * @returns {*}
 */
export function exeCommand(cmd: string) {
  return invoke(InvokeMethod.EXE_COMMAND, { cmd });
}

/**
 * @description 根据关键字查询启动项
 * @author Peng
 *
 * @param {string} keyword
 * @returns {*}
 */
export function searchLaunch<T = SearchLauncItem>(keyword: string) {
  return invoke<T[]>(InvokeMethod.SEARCH_LAUNCH, { keyword });
}

/**
 * @description 启动项重命名
 * @author Peng
 *
 * @param {number} id
 * @param {string} name
 * @returns {*}
 */
export function renameLaunch(id: number, name: string) {
  return invoke(InvokeMethod.RENAME_LAUNCH, { id, name });
}

/**
 * @description 删除启动项
 * @author Peng
 *
 * @param {number} id
 * @returns {*}
 */
export function deleteLaunch(id: number) {
  return invoke(InvokeMethod.DELETE_LAUNCH, { id });
}

/**
 * @description 获取网站信息
 * @author Peng
 *
 * @param {string} url
 * @returns {*}
 */
export function getWebsiteInfo(url: string) {
  return invoke(InvokeMethod.GET_WEBSITE_INFO, { url });
}

/**
 * @description 新建分类
 * @author Peng
 *
 * @template [T=NewCategoryItem]
 * @param {T} item
 * @returns {*}
 */
export function addCategory<T = NewCategoryItem>(item: T) {
  return invoke(InvokeMethod.ADD_CATEGORY, { item });
}

/**
 * @description 新建分类
 * @author Peng
 *
 * @template [T=CategoryItem]
 * @param {T} item
 * @returns {*}
 */
export function updateCategory<T = CategoryItem>(item: T) {
  return invoke(InvokeMethod.UPDATE_CATEGORY, { item });
}

/**
 * @description 获取分类
 * @author Peng
 *
 * @returns {*}
 */
export function getCategory() {
  return invoke<CategoryItem[]>(InvokeMethod.GET_CATEGORY);
}

export function setAppConfig(config: AppConfigState) {
  return invoke(InvokeMethod.SET_APP_CONFIG, { config });
}

export function getLocalIconBase64(path: string): Promise<string> {
  return invoke<string>(InvokeMethod.GET_LOCAL_ICON_BASE64, { path });
}

export function getOnlineImgBase64(url: string): Promise<string> {
  return invoke<string>(InvokeMethod.GET_ONLINE_IMG_BASE64, { url });
}
