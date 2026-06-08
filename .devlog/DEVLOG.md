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
