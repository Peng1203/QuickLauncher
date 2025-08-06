interface LaunchItem {
  id: number
  name: string
  path: string
  type: 'file' | 'directory' // 网站
  icon?: string
  hotkey?: string
  hotkey_global?: 0 | 1
  keywords?: string
  start_dir?: string
  remarks?: string
  args?: string
  run_as_admin?: 0 | 1
  order_index?: number
  enabled?: 0 | 1
  category_id?: number
  last_used_at?: string // 你可以换成 Date 类型根据需要
  created_at: string
  updated_at: string
}

type NewLaunchItem = Omit<LaunchItem, 'id' | 'last_used_at' | 'created_at' | 'updated_at'>

type SearchLauncItem = Pick<LaunchItem, 'id' | 'name' | 'icon'>

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

interface FileInfo {
  name: string
  path: string
  size: number
  icon: string
  created?: number | null
  modified?: number | null
  is_file: boolean
  is_dir: boolean
  type: 'file' | 'directory' // 更精确的枚举类型
}

// 应用配置状态
interface AppConfigState {
  /** 静默启动 */
  silentStart: boolean
  /** 开机自启 */
  autoStart: boolean
  /** 窗口置顶 */
onTop: boolean
  /** 窗口居中 */
  center: boolean
}
