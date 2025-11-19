import type { WebSearchOpenModel } from '@/constant';

declare global {
  interface LaunchItem {
    id: number;
    name: string;
    path: string;
    type: 'file' | 'directory' | 'url';
    icon?: string | null;
    hotkey?: string | null;
    hotkey_global?: 0 | 1;
    keywords?: string | null;
    start_dir?: string | null;
    remarks?: string | null;
    args?: string | null;
    run_as_admin?: 0 | 1;
    order_index?: number;
    enabled?: 0 | 1;
    category_id?: number | null;
    subcategory_id?: number | null;
    last_used_at?: string | null;
    created_at: string;
    updated_at: string;

    extension?: string | null;
    launch_count: number;
    failure_count: number;
  }

  type NewLaunchItem = Omit<
    LaunchItem,
    | 'id'
    | 'last_used_at'
    | 'created_at'
    | 'updated_at'
    | 'launch_count'
    | 'failure_count'
  >;

  type SearchLauncItem = Pick<
    LaunchItem,
    'id' | 'name' | 'icon' | 'category_id' | 'subcategory_id'
  > & {
    category_name: string;
    subcategory_name: string;
  };

  // interface NewLaunchItem {
  //   name: string
  //   path: string
  //   type: 'file' | 'directory' // 网站
  //   icon?: string | null
  //   hotkey?: string | null
  //   hotkey_global?: 0 | 1
  //   keywords?: string | null
  //   start_dir?: string | null
  //   remarks?: string | null
  //   args?: string | null
  //   run_as_admin?: 0 | 1
  //   order_index?: number
  //   enabled?: 0 | 1
  //   category_id?: number | null
  // }

  type FileInfo = Pick<
    LaunchItem,
    | 'name'
    | 'path'
    | 'icon'
    | 'type'
    | 'extension'
    | 'args'
    | 'remarks'
    | 'start_dir'
  > & { size: number };

  // 应用配置状态
  interface AppConfigState {
    /** 保存flag */
    saveFlag: boolean;
    /** 静默启动 */
    silentStart: boolean;
    /** 开机自启 */
    autoStart: boolean;
    /** 窗口置顶 */
    onTop: boolean;
    /** 窗口居中 */
    center: boolean;

    /** 搜索窗口最大高度 */
    searchWindowMaxHeight: number;
    /** 搜索窗口固定宽度 */
    searchWindowWidth: number;
    /* 搜索框高度 */
    searchWindowInput: number;
    /* 搜索结果高度 */
    searchResultItemHeight: number;
    /* 全局快捷键 */
    searchGlobalShortcutKey: string;

    proxy: boolean;
    proxyHost: string;
    proxyUsername: string;
    proxyPassword: string;

    // mainWindowPosition: { x: number; y: number }
    mainWindowPositionX: number;
    mainWindowPositionY: number;
    mainWindowGlobalShortcutKey: string;

    settingWindowPositionX: number;
    settingWindowPositionY: number;

    language: LanguageType;

    /** 启动快速搜索 */
    enableSearch: boolean;
    /** 搜索框失去焦点隐藏 */
    searchLostFocusHide: boolean;
    /** 打开后隐藏 */
    searchHideAfterOpen: boolean;

    /** 网络搜索打开呼出方式 0 */
    webSearchOpenModel: WebSearchOpenModel;
    webSearchSourceList: WebSearchSource[];

    showHistory: boolean;
    enableAutocomplete: boolean;
    autocompleteMatchMode: 'prefix' | 'contains';
    // 是否过滤掉使用次数过低的历史记录，仅保留高频记录用于自动补全
    enableAutocompleteFrequencyFilter: boolean;
  }

  type LanguageType = 'zh-CN' | 'zh-HK' | 'en' | 'ja';

  interface OptionItem<T = string | number, K = string> {
    label: K;
    value: T;
    icon?: any;
    type?: any;
  }

  interface CategoryItem {
    id: number;
    name: string;
    parent_id: number;
    association_directory?: string | null;
    created_at: string;
    updated_at: string;
  }

  type NewCategoryItem = Omit<CategoryItem, 'id' | 'created_at' | 'updated_at'>;

  interface WebSearchSource {
    id: number;
    keywords: string;
    name: string;
    searchApi: string;
    icon?: string;
    suggestion?: any;
    suggestionApi?: string;
    desc?: string;
  }

  interface AutocompleteItem {
    id: number;
    query: string;
    usage_count: number;
    last_used_at: string;
    launch_item_id?: number | null;
  }
}

export {};
