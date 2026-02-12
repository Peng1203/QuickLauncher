/* eslint-disable */
export * from './data.ts';

/* 搜索窗口最大高度 */
export const SEARCH_WINDOW_MAX_HEIGHT = 400;

/* 搜索窗口固定宽度 */
export const SEARCH_WINDOW_WIDTH = 600;

/* 搜索框高度 */
export const SEARCH_INPUT_HEIGHT = 45;

/* 搜索结果高度 */
export const SEARCH_RESULT_ITEM_HEIGHT = 48;

/* 右击菜单的宽度 */
export const MENU_WIDTH = 150;

export enum SearchWindow {
  WINDOW_WIDTH = SEARCH_WINDOW_WIDTH,
  MAX_HEIGHT = SEARCH_WINDOW_MAX_HEIGHT,
  INPUT_HEIGHT = SEARCH_INPUT_HEIGHT,
  RESULT_ITEM_HEIGHT = SEARCH_RESULT_ITEM_HEIGHT,
}

export enum InvokeMethod {
  OPEN_PATH = 'open_path',
  REVEAL_IN_FILE_MANAGER = 'reveal_in_file_manager',

  GET_APP_CONFIG = 'get_app_config',
  SAVE_APP_CONFIG = 'save_app_config',

  GET_FILE_INFO = 'get_file_info',

  ADD_LAUNCH = 'add_launch',
  GET_LAUNCH = 'get_launch',
  GET_LAUNCH_BY_ID = 'get_launch_by_id',
  RUN_LAUNCH = 'run_launch',
  RUN_LAUNCH_AS_ADMIN = 'run_launch_as_admin',
  SEARCH_LAUNCH = 'search_launch',
  RENAME_LAUNCH = 'rename_launch',
  DELETE_LAUNCH = 'delete_launch',
  UPDATE_LAUNCH = 'update_launch',

  GET_CATEGORY = 'get_category',
  ADD_CATEGORY = 'add_category',
  UPDATE_CATEGORY = 'update_category',
  DELETE_CATEGORY = 'delete_category',

  EXE_COMMAND = 'exe_command',

  GET_WEBSITE_INFO = 'get_website_info',

  // 将app配置数据存放在后端共享线程中
  SET_APP_CONFIG = 'set_app_config',

  GET_LOCAL_ICON_BASE64 = 'get_local_icon_base64',
  GET_ONLINE_IMG_BASE64 = 'get_online_img_base64',

  ADD_OR_UPDATE_AUTOCOMPLETE = 'add_or_update_autocomplete',
  GET_AUTOCOMPLETE = 'get_autocomplete',

  // 根据名称和分类获取启动项
  GET_LAUNCH_BY_NAME_AND_CATEGORY = 'get_launch_by_name_and_category',
}

export enum AppEvent {
  UPDATE_LAUNCH_LIST = 'update_launch_list',
  OPEN_OPERATION_LAUNCH = 'open_operation_launch',
  OPEN_OPERATION_CATEGORY = 'open_operation_category',

  // 关闭右键菜单
  CLOSE_CONTEXT_MENU = 'close_context_menu',
  UPDATE_CATEGORY_LIST = 'update_category_list',

  // 通知其他窗口更新pinia中的appConfig数据
  UPDATE_APP_CONFIG_DATA = 'update_app_config',

  // 快速搜索快捷键事件
  SEARCH_SHORTCU_KEY = 'search_shortcu_key',
}

export type AppEventName = (typeof AppEvent)[keyof typeof AppEvent];

export enum WebSearchOpenModel {
  KEY_SPACE = 0,
  COLON_KEY_SPACE = 1,
}

export enum AutocompleteMatchMode {
  Prefix = 'prefix',
  Contains = 'contains',
}
