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

## 2026-05-20（项目初始化）

### 技术选型

- Tauri v2 + Vue 3
- SeaORM + SQLite
- Pinia + tauri-store

### 架构搭建

- API 层抽离
- commands 分离
- components/ui 分层
