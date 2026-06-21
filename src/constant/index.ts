export * from "./data.ts";

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
  OPEN_PATH = "open_path",
  OPEN_REVEAL_MANAGER = "open_reveal_manager",
  OPEN_DIR_IN_TERMINAL = "open_dir_in_terminal",

  GET_APP_CONFIG = "get_app_config",
  SAVE_APP_CONFIG = "save_app_config",

  GET_FILE_INFO = "get_file_info",

  ADD_LAUNCH = "add_launch",
  GET_LAUNCH = "get_launch",
  GET_LAUNCH_BY_ID = "get_launch_by_id",
  RUN_LAUNCH = "run_launch",
  RUN_LAUNCH_AS_ADMIN = "run_launch_as_admin",
  SEARCH_LAUNCH = "search_launch",
  RENAME_LAUNCH = "rename_launch",
  DELETE_LAUNCH = "delete_launch",
  UPDATE_LAUNCH = "update_launch",
  DELETE_LAUNCH_BY_CATEGORY = "delete_launch_by_category",
  UPDATE_LAUNCH_ENABLED_BY_CATEGORY = "update_launch_enabled_by_category",

  GET_CATEGORY = "get_category",
  GET_CATEGORY_BY_ID = "get_category_by_id",
  ADD_CATEGORY = "add_category",
  UPDATE_CATEGORY = "update_category",
  DELETE_CATEGORY = "delete_category",
  UPDATE_CATEGORY_ASS_DIR = "update_category_ass_dir",
  ENSURE_DEFAULT_CATEGORY = "ensure_default_category",

  EXE_COMMAND = "exe_command",

  GET_WEBSITE_INFO = "get_website_info",

  GET_LOCAL_ICON_BASE64 = "get_local_icon_base64",
  GET_ONLINE_IMG_BASE64 = "get_online_img_base64",

  ADD_OR_UPDATE_AUTOCOMPLETE = "add_or_update_autocomplete",
  GET_AUTOCOMPLETE = "get_autocomplete",

  // 根据名称和分类获取启动项
  GET_LAUNCH_BY_NAME_AND_CATEGORY = "get_launch_by_name_and_category",

  // 判断前台窗口是否处于全屏
  IS_FOREGROUND_FULLSCREEN = "is_foreground_fullscreen",

  // 获取 命令别名启动项
  GET_ALIAS_LAUNCH = "get_alias_launch",
  // 获取分类启动项树形数据
  GET_CATEGORY_TREE = "get_category_tree",
  // 添加启动项历史
  ADD_LAUNCH_HISTORY = "add_launch_history",
  // 获取历史记录
  GET_RECENT_LAUNCH_HISTORY = "get_recent_launch_history",

  OPEN_APP_DATA_DIR = "open_app_data_dir",
  BACKUP_DATABASE = "backup_database",
  IMPORT_DATABASE = "import_database",
  RESET_DATA = "reset_data",

  // 设置默认托盘图标
  SET_DEFAULT_TRAY_ICON = "set_default_tray_icon",

  // 获取系统字体
  GET_LOCAL_FONTS = "get_local_fonts",

  // Todo
  GET_TODOS = "get_todos",
  ADD_TODO = "add_todo",
  UPDATE_TODO = "update_todo",
  DELETE_TODO = "delete_todo",
  CLEAR_COMPLETED_TODOS = "clear_completed_todos",
  BATCH_UPDATE_TODO_ORDER = "batch_update_todo_order",

  // 文件搜索
  SEARCH_FILES = "search_files",

  // WebDAV 云备份
  WEBDAV_TEST_CONNECTION = "webdav_test_connection",
  WEBDAV_BACKUP = "webdav_backup",
  WEBDAV_RESTORE = "webdav_restore",
  WEBDAV_LIST_BACKUPS = "webdav_list_backups",
}

export enum AppEvent {
  UPDATE_LAUNCH_LIST = "update_launch_list",
  OPEN_OPERATION_LAUNCH = "open_operation_launch",
  DELETE_LAUNCH = "delete_launch_item",

  DELETE_CATEGORY = "delete_category_item",
  OPEN_OPERATION_CATEGORY = "open_operation_category",

  // 关闭右键菜单
  CLOSE_CONTEXT_MENU = "close_context_menu",
  UPDATE_CATEGORY_LIST = "update_category_list",
  ACTIVE_CATEGORY = "active_category",

  // 通知其他窗口更新pinia中的appConfig数据
  UPDATE_APP_CONFIG_DATA = "update_app_config",

  // 快速搜索快捷键事件
  SEARCH_SHORTCU_KEY = "search_shortcu_key",

  // 分类重命名
  CATEGORY_RENAME = "category_rename",

  // 提升查询项优先级
  INCREASE_PRIORITY = "increase_priority",

  // 更新启动项启动次数
  UPDATE_LAUNCH_ITEM_COUNT = "update_launch_item_count",

  // 分类重命名
  LAUNCH_RENAME = "launch_rename",

  // 启动项定位
  LAUNCH_POSITION = "launch_position",

  // 剪贴板通知
  CLIPBOARD = "clipboard",

  // Todo 提醒通知
  TODO_REMINDER = "todo-reminder",

  OPEN_CLIPBOARD_WINDOW_BY_SET_LOCATION_MODAL = "open_clipboard_window_by_set_location_modal",

  // 打开设置界面
  OPEN_SETTING = "open_setting",

  // 主题切换事件通知
  CHANGE_THEME = "change_theme",
}

export type AppEventName = (typeof AppEvent)[keyof typeof AppEvent];

export enum WebSearchOpenModel {
  KEY_SPACE = 0,
  COLON_KEY_SPACE = 1,
  CLOSE = 2,
}

export enum TranslationOpenModel {
  THREE_HITS_ON_SPACES = 1,
  CLOSE = 0,
}

export enum PortalNotifyMode {
  WINDOW = "window",
  TRAY = "tray",
  SILENT = "silent",
}

export enum AutocompleteMatchMode {
  Prefix = "prefix",
  Contains = "contains",
}

export const ACTIVE_CATEGORY_LOCAL_KEY = "active_category";

export const SEARCH_MODEL = {
  DEFAULT_MODEL: 0,
  WEB_SEARCH_MODEL: 1,
  TRANSLATION_MODEL: 2,
  TODO_MODEL: 3,
  FILE_MODEL: 4,
} as const;

export const MODE_TABS: SearchModeItem[] = [
  {
    label: "searchMode",
    value: SEARCH_MODEL.DEFAULT_MODEL,
    icon: "icon-icon-sousuofenlei",
    disabled: false,
    color: "#007AFF",
  },
  {
    label: "todoMode",
    value: SEARCH_MODEL.TODO_MODEL,
    icon: "icon-TODO_INFO",
    disabled: false,
    // color: "#34C759",
    color: "#155dfc",
  },
  {
    label: "translateMode",
    value: SEARCH_MODEL.TRANSLATION_MODEL,
    icon: "icon-fanyi",
    disabled: false,
    color: "#8B5CF6",
  },
  {
    label: "webSearchMode",
    value: SEARCH_MODEL.WEB_SEARCH_MODEL,
    icon: "icon-wangluosousuo",
    disabled: false,
    color: "#06B6D4",
  },
  {
    label: "fileSearchMode",
    value: SEARCH_MODEL.FILE_MODEL,
    icon: "icon-everything",
    disabled: false,
    color: "#F59E0B",
  },
] as const;
