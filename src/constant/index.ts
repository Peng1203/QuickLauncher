/* 搜索窗口最大高度 */
export const SEARCH_WINDOW_MAX_HEIGHT = 400

/* 搜索窗口固定宽度 */
export const SEARCH_WINDOW_WIDTH = 600

/* 搜索框高度 */
export const SEARCH_INPUT_HEIGHT = 45

/* 搜索结果高度 */
export const SEARCH_RESULT_ITEM_HEIGHT = 48

export const enum SearchWindow {
  WINDOW_WIDTH = SEARCH_WINDOW_WIDTH,
  MAX_HEIGHT = SEARCH_WINDOW_MAX_HEIGHT,
  INPUT_HEIGHT = SEARCH_INPUT_HEIGHT,
  RESULT_ITEM_HEIGHT = SEARCH_RESULT_ITEM_HEIGHT,
}

export const enum InvokeMethod {
  OPEN_PATH = 'open_path',

  GET_APP_CONFIG = 'get_app_config',
  SAVE_APP_CONFIG = 'save_app_config',

  GET_FILE_INFO = 'get_file_info',

  ADD_LAUNCH = 'add_launch',
  GET_LAUNCH = 'get_launch',
  RUN_LAUNCH = 'run_launch',
  RUN_LAUNCH_AS_ADMIN = 'run_launch_as_admin',
  SEARCH_LAUNCH = 'search_launch',
  RENAME_LAUNCH = 'rename_launch',
  DELETE_LAUNCH = 'delete_launch',
  UPDATE_LAUNCH = 'update_launch',

  EXE_COMMAND = 'exe_command',

  GET_WEBSITE_INFO = 'get_website_info',
}

export const enum AppEvent {
  UPDATE_LAUNCH_LIST = 'update_launch_list',
  EDIT_LAUNCH = 'edit_launch',
}

export type AppEventName = (typeof AppEvent)[keyof typeof AppEvent]
