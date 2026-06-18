## 2026-06-18（网站启动项浏览器参数 / Dto 适配修复）

### 网站启动项浏览器参数

- `run_launch` 命令支持网站类型启动项自定义浏览器启动参数
- `OperationLaunch` 编辑页面新增浏览器参数输入框
- 参数通过 `cmd /C start "" "url" "args"` 模式传递

### Dto 适配修复

- 修复 `created_at`/`updated_at` 字段类型为 `number` 导致的前端显示问题
- 适配 `DefaultSearchMode` 和 `CommandAlias` 组件

---

## 2026-06-14（文件搜索模式 / Everything 集成）

### 文件搜索模式

- 新增 `search_files` 命令，支持 Everything 搜索
- 新增 `FileSearchResult` 模型（name, path, size, icon, type, extension）
- 支持两种调用方式：es.exe 命令行 / HTTP API
- HTTP API 参数：`s=search, c=count, j=json, path_column=1, size_column=1`
- 搜索结果支持路径高亮、键盘导航、回车打开、Ctrl+Enter 管理员运行

### 文件搜索设置

- 新增配置字段：`fileSearchMode`, `fileSearchAutoStart`, `everythingExePath`, `everythingHttpHost`, `everythingHttpPort`
- 设置页面：连接方式选择（es.exe / HTTP）、Everything 路径配置、HTTP 服务器配置
- 自动启动 Everything：启动前检测进程状态，未运行时自动启动并等待 8 秒
- es.exe 重试机制：IPC 未就绪时最多重试 5 次，每次等待递增

### SearchModeTabs 更新

- 新增 `drag` 属性控制拖拽
- 新增 `disabled` 字段：禁用项半透明 + not-allowed 光标 + hover 效果
- 拖拽重排触发 `drag` 事件
- TransitionGroup 横向移动过渡效果
- 快捷键切换模式时跳过 disabled 项

## 2026-06-14（Todo 提醒通知 / UI 优化）

### Todo 提醒通知功能

- 新增 `tauri-plugin-notification` 依赖，支持系统原生通知
- 新增 `src-tauri/src/reminder/mod.rs` 提醒调度器（全局单例，tokio 异步任务）
- 应用启动时自动加载数据库中未过期的提醒并调度
- 创建/更新 todo 时自动调度或取消提醒
- 删除/完成/清除 todo 时取消对应提醒
- 前端监听 `todo-reminder` 事件刷新列表

### Todo order_index 支持

- 新增 `order_index` 字段迁移（`m20260613_151526`）
- 新增 `batch_update_todo_order` 批量更新排序命令
- `UpdateTodoDto` 新增 `into_active_model()` 方法，简化更新逻辑

### Todo 列表 UI 优化

- TodoItem 新增过期样式（淡红背景 + 红色标题）
- 提醒时间展示：精确到分钟（如"15分钟后"、"2小时后"、"今天 14:30"）
- 已过提醒时间显示"已提醒"（灰色）
- 新增 `formatRelativeReminder` 公共方法到 `src/utils/date.ts`

### SearchModeTabs 拖拽与 disabled

- 新增 `drag` 属性控制是否启用 VueDraggable 拖拽
- 新增 `disabled` 字段：禁用项半透明 + not-allowed 光标 + hover 效果
- 拖拽重排时触发 `drag` 事件，附带排序后数据
- 添加 `TransitionGroup` 横向移动过渡效果
- 快捷键切换模式时自动跳过 disabled 项

### update_todo Dto 重构

- `update_todo` 命令改用 `UpdateTodoDto.into_active_model()` 模式
- 返回类型从 `TodoModel` 改为 `()`，前端适配

## 2026-06-13（Dto 重构 / models 清理）

### Dto 模式重构

- `launch_items`：新增 `CreateLaunchItemDto`、`UpdateLaunchItemDto`、`SearchLaunchItemDto`，补充 `FromQueryResult` 和 `into_active_model()` 方法
- `todos`：新增 `CreateTodoDto`、`UpdateTodoDto`，补充 `From<Model>` 和 `From<Dto> for ActiveModel` 实现
- `configs`：新增 `UpdateConfigDto`，适配 `save_app_config` 命令
- `launch_history`：`started_at` 改为 `Option<i64>`，`CreateLaunchHistoryDto` 适配

### 命令层适配

- `add_launch` / `update_launch` / `search_launch` 参数改用 Dto 类型
- `add_todo` / `update_todo` 参数改用 Dto 类型
- `save_app_config` 参数改用 `UpdateConfigDto`
- `add_or_update_autocomplete` 修复 `Expr::current_timestamp()` 为 `Expr::value(Utc::now().timestamp_millis())`

### Entity schema 变更

- `launch_items.last_used_at` 改为 `Option<i64>`
- `launch_history.started_at` 改为 `Option<i64>`

### models 目录清理

- 删除全部 models 文件（`launch_item`、`todo_item`、`category_item`、`config_item`、`autocomplete_item`、`app_config_state`）
- 删除 `entity/prelude.rs`
- `lib.rs` 移除 `AppConfigState` 管理初始化

## 2026-06-11（搜索重构 / 分类排序 / UI 优化）

### 快速搜索 - 按分类搜索

- 后端 `search_launch` 新增 `category_id` 参数，支持按分类过滤
- 前端 `searchLaunch` API 签名同步扩展
- 新增配置项：`enableDefaultSearchByCategory`、`enableCategorySearchDefaultData`
- 设置页新增「按分类搜索」和「默认数据」开关
- 新增 4 语种国际化 key（`search.*`、`quickSearch.*`）

### DefaultSearchMode 重构

- 删除旧 `DefaultSearchMode.vue`（603 行），迁移至 `DefaultMode/` 目录
- 搜索窗口显示时调用 `handleBeforeShow` 生命周期

### 侧边栏分类排序

- 使用 `VueDraggable` 实现分类拖拽排序
- 拖拽结束时批量调用 `updateCategory` 更新 `order_index`
- 默认分类（`order_index=9999`）禁止拖拽
- 新增 `fade` 过渡动画
- 分类图标：有 icon 显示图标，无 icon 显示默认分类图标

### 主窗口优化

- `Main.vue` 新增 `initCursor()` 初始化光标状态
- `LaunchList.vue` 添加 `TransitionGroup` 支持拖拽动画
- 暂时注释同一列表内排序的持久化逻辑

### 菜单与国际化

- 关联目录分类右键菜单隐藏「新建启动项」
- 搜索占位符补充「命令别名」说明
- TodoMode 快捷键提示顺序调整

### 代码清理

- 移除 `useCategoryCorrelationDir` 中所有 `console.log` 调试输出
- `OperationLaunch` 分类选项改用 `categoryData` 计算生成
- `exe_command` 注释掉固定 `current_dir` 设置
- 清理 `OperationLaunch` 中未使用的 `baseSelectionHeight`

## 2026-06-09（托盘增强 / 拖拽分类 / 分类图标）

### 托盘菜单增强

- 新增语言切换子菜单（4 语种，CheckMenuItem 标记当前选中）
- 新增主题切换子菜单（浅色/深色/跟随系统）
- 主题切换参照 `Theme.vue` 的 `handleSwitchTheme` 逻辑
- 菜单结构调整：设置、语言、主题置于顶部
- 删除 Rust 端 `tray.rs`，托盘逻辑完全由前端管理

### 启动项拖拽到分类

- 使用 `vue-draggable-plus` 实现启动项可拖拽
- 新增 `useLaunchDrag` composable 管理跨组件拖拽状态
- Sidebar 分类按钮添加 `dragover` / `dragleave` / `drop` 事件
- 拖拽悬停高亮（主题色虚线框 + 半透明背景）
- 关联目录分类禁止拖拽排序和作为放置目标
- 修复 `handleDragEnd` 覆盖 `handleDrop` 的竞态问题
- `getLaunchData` 增加分类 ID 校验防止竞态覆盖

### 侧边栏分类图标

- `AppConfigState` 新增 `showCategoryIcon` 配置
- General 设置页新增「显示分类图标」开关
- Sidebar 根据配置 + item.icon 动态显示分类图标
- 主窗口禁用原生拖放（`dragDropEnabled: false`）

### IconPicker 优化

- `handleGetLocalDirIcon` 支持传入 path 参数
- OperationCategory 关联目录选择时自动调用图标选择器

### 其他

- 关联目录监听函数补充调试日志
- `components.d.ts` 自动更新

---

## 2026-06-06（TodoMode 重构 / SearchModeTabs 适配 / i18n 补全）

### TodoMode 组件重构

- `n-icon` 统一替换为自定义 `Icon` 组件
- 精简模板结构，移除虚拟预览项等冗余代码
- 清理 `index.ts` 中未使用的导出

### 日期工具多语言

- `dayjs` 新增 zh-hk / en / ja 语言包
- 新增 `setDayjsLang` / `getDayjsLang` 函数
- 语言切换时自动同步 `dayjs.locale`

### SearchModeTabs 尺寸适配

- 新增 `size` prop，支持 `large / default / small / mini`
- `mini` 模式隐藏标签文字，仅显示图标
- `small` 模式缩小图标与间距
- 搜索设置页根据语言映射不同尺寸（en/ja → mini）

### 搜索设置国际化

- `SearchSetting.vue` 硬编码中文改为 `t()` 调用
- 新增 `quickSearch.*` 相关翻译 key

### 全局 i18n 补全（14 个文件）

- 设置页：Setting / CommandAlias / Data / General / Translation / WebSearch
- 搜索页：Search / WebSearchMode
- 右键菜单：CategoryItemContextMenu / ListItemContextMenu
- 新增 39 条翻译 key（`message.ts`）

### 其他

- 图标字体 CSS 更新（`index.html`）
- 禁用 `vueDevTools`（`vite.config.ts`）
- `constant/index.ts` 新增搜索相关常量

---

## 2026-06-02（工程化 / Lint 重构）

### ESLint Prettier 配置迁移至 Vite+

- eslint → oxclint
- prettier → oxcfmt
- 手动补充 `globals`（auto-import 支持）

### 规则调整

- vscode 默认格式化(prettier)修改为 oxc
- 调整 TS/Vue 规则兼容 auto-import

### Lint 修复（36 → 0）

- `no-unused-expressions` → 改 if 结构
- `no-floating-promises` → 加 `void`
- 模板字符串 → `String()`
- 清理 unused disable

### 发布脚本优化

- release notes 改为读取 `CHANGELOG.md`
- 构建产物路径调整到 bundle 目录
- 自动打开构建输出目录
- `latest.json` 不再入 git

## 2026-05-31（搜索系统重构 + 数据库迁移）

### Search 架构拆分

- `Search.vue` 拆为容器 + 多模式组件：
  - DefaultSearchMode
  - WebSearchMode
  - TranslationMode
  - TodoMode

- 每个模式独立处理：
  - 键盘事件
  - 业务逻辑

- 统一接口：`focus / handleKeydown / close / height`

### TodoMode 数据库化

- localStorage → SQLite
- 新增 `todos` 表（SeaORM）
- 新增 Tauri CRUD API：
  - get/add/update/delete/clear_completed

- tags 改为 JSON 字符串
- id 从 UUID → 自增 int

---

## 2026-05-30（i18n + 托盘 + 主题重构）

### 国际化系统

- 引入 `vue-i18n`
- 统一翻译数据结构（数组 → 转对象）
- 支持 4 语言：zh-CN / zh-HK / en / ja
- 覆盖 170+ 文本

### 托盘系统重构

- Rust 托盘 → 前端创建 (`TrayIcon + Menu`)
- 菜单支持 i18n 动态更新
- 避免重复创建托盘实例

### 主题系统重构

- `matchMedia` → VueUse `usePreferredDark`
- 合并 watch 逻辑
- 新增 ViewTransition 动画（多种效果预设）
- 主题切换跨窗口同步（CHANGE_THEME）

---

## 2026-05-29（组件体系重构）

### UI 组件重构

- 新建 `components/ui`
- 统一基础组件结构
- 迁移：
  - SettingItem / Select / Switch
  - Kbd / ShortcutInput 等

### 设置面板优化

- 统一 Setting 组件体系
- composable 化设置逻辑
- 收敛重复 UI 结构

### 工程化

- Prettier + cargo fmt 统一格式化
- ESLint 规范建立
- 发布脚本自动化

---

## 2026-05-27（功能增强 + 体验优化）

### 设置系统

- SQLite 存 configs
- 内存缓存 + DB 双写
- 启动重试加载（最多 20 次）

### 传送门功能

- 剪贴板监听（300ms 轮询）
- 自动识别 URL / 文件 / 目录

### 快捷键系统

- 全局快捷键（Tauri plugin）
- 应用内快捷键体系
- 支持自定义 + 冲突检测

---

## 2026-05-25（核心功能开发）

### 启动项系统

- CRUD + 分类 + 批量操作
- 支持：
  - 文件 / 目录 / URL / 多任务
  - 管理员启动

- `.lnk` 解析 + 参数统一处理

### 搜索系统

- 独立搜索窗口
- 拼音索引（full + abbr）
- 支持：
  - 中文拼音搜索
  - 自动补全

### 分类系统

- 树形结构（自引用）
- 文件夹同步监听（notify）

### 多窗口架构

- 主 / 搜索 / 设置 / 编辑 / 剪贴板
- SPA + 路由复用
- `@tauri-store/pinia` 跨窗口状态

---

## 2026-06-03（搜索模式重构 / Todo 优化）

### WebSearchMode 改造

- 搜索结果列表改为搜索引擎下拉选择
- 输入框为空时展示引擎列表，输入时隐藏
- 支持从默认模式带 source 进入时自动定位引擎
- 新增 `webSearchDefaultSourceId` 配置，支持设置默认搜索引擎
- 处理搜索源为空时的禁用状态提示

### TodoMode 重构

- 拆分 `detail` 状态为 `detail-create` / `detail-edit`
- 新建待办不再立即调 API，保存时才创建
- 键盘导航：上下切换、Enter 完成、右键进入详情
- 输入框右侧显示快捷键提示
- 详情页新增提醒日期（`n-date-picker`）
- 截止日期使用 `dayjs` + `getDaysUntil` 计算天数
- 默认优先级改为 `low`
- 标签改用 `n-dynamic-tags` 组件
- 排序和筛选改为后端实现（`getTodos` 传入参数）
- Tab 快捷键切换排序方式
- TodoItem 组件抽离

### 搜索设置页

- 新增「搜索设置」Tab（位于快速搜索上方）
- 包含：模式标签、快捷键、窗口行为等配置

### 其他

- 修复主题切换多窗口同步问题
- 补全翻译模式遗漏的 i18n 文本
- 新增 `getDaysUntil` 日期工具方法
- 换行符统一为 LF

---

## 2026-06-08（菜单复用 / 浏览器配置）

### 排序菜单代码复用

- 提取 `useLayoutOrderMenu` composable
- `CategoryItemContextMenu` 和 `ListContextMenu` 共享菜单数据源
- 支持配置项：`showDefault`、`showLaunchCount`
- `handleLayoutOrderSelect` 统一处理菜单选择事件

### 底部栏排序同步

- Footer 排序选项与右键菜单保持一致
- 新增 `default` 和 `launch_count` 排序方式
- 排序方向新增 `default` 选项

### 浏览器配置

- `WebSearchSource` 新增 `browser` 字段
- `BrowserPicker` 改用 `appConfigStore.browserOptions` 全局配置
- 网络搜索源编辑表单新增浏览器选择
- 传送门设置新增浏览器选择配置

---

## 2026-06-06（i18n 补全 / 组件适配）

### 国际化

- TodoMode 全面 i18n（占位符、快捷键提示、筛选/排序标签、详情页字段、按钮）
- SearchModeTabs 尺寸适配与搜索设置国际化
- 补全多语言翻译与配置更新

### 组件重构

- TodoItem 组件抽离
- TodoMode 日期工具多语言支持

---

## 2026-06-05（搜索模式架构）

### SearchModeTabs 组件抽离

- 从 Search.vue 抽离独立组件
- 支持模式切换 Tab 栏

---

## 2026-06-02（工程化 / Lint 重构）

### ESLint 配置迁移

- 从 `@antfu/eslint-config` 迁移到原生 flat config
- 组合使用 `@eslint/js` + `typescript-eslint` + `eslint-plugin-vue`
- 手动补充 `globals`（auto-import 支持）

### 规则调整

- 关闭 `stylistic`，交由 Prettier 处理
- `.ts / .vue` 关闭 `no-undef`
- 调整 TS/Vue 规则兼容 auto-import

### Lint 修复（36 → 0）

- `no-unused-expressions` → 改 if 结构
- `no-floating-promises` → 加 `void`
- 模板字符串 → `String()`
- 清理 unused disable

### 发布脚本优化

- release notes 改为读取 `CHANGELOG.md`
- 构建产物路径调整到 bundle 目录
- 自动打开构建输出目录
- `latest.json` 不再入 git

---

## 2026-05-20（项目初始化）

### 技术选型

- Tauri v2 + Vue 3
- SeaORM + SQLite
- Pinia + tauri-store

### 架构搭建

- API 层抽离
- commands 分离
- components/ui 分层
