# QuickLauncher 开发日志

> 本文档记录项目的开发演进过程，重点关注开发行为、技术决策和架构变化。
> 目的是为 AI 和未来的维护者提供完整的项目上下文。

---

## 项目初始化阶段

Date: 2026-05-20
Type: 项目启动

### 背景与动机

开发一个 Windows 快速启动器，解决以下痛点：

- 系统自带启动器功能有限
- 需要支持多种启动类型（文件、目录、URL、应用组合）
- 需要快速搜索和访问常用工具
- 需要跨窗口同步状态

### 技术选型决策

**框架选择：Tauri v2 + Vue 3**

选择理由：

- Tauri 比 Electron 更轻量，打包体积小
- Rust 后端性能优秀，适合系统级操作
- Vue 3 生态成熟，开发效率高
- 原生支持 Windows 系统 API

**数据库选择：SeaORM + SQLite**

演进过程：

1. 初期使用 rusqlite（直接绑定）
2. 切换到 SeaORM（ORM 抽象）

切换原因：

- SeaORM 提供更好的类型安全
- 支持异步操作
- Schema 自动同步功能
- 更容易维护和扩展

**状态管理：Pinia + @tauri-store/pinia**

选择理由：

- Pinia 是 Vue 3 官方推荐
- @tauri-store/pinia 支持跨窗口同步
- 避免了复杂的跨窗口通信实现

### 初始架构搭建

**目录结构设计**：

```
src/
├── api/          # 统一 API 层
├── components/   # 可复用组件
├── composables/  # Vue 组合式函数
├── views/        # 页面视图（按窗口组织）
├── store/        # Pinia 状态管理
└── router/       # 路由（hash history）

src-tauri/src/
├── commands/     # Tauri 命令（每个命令独立文件）
├── entity/       # SeaORM 实体定义
├── models/       # 数据模型和 DTO
└── db/           # 数据库连接管理
```

**设计原则**：

- 命令分离：每个 Tauri 命令独立文件，便于维护
- API 抽象：前端统一通过 `api/index.ts` 调用后端
- 组件复用：建立 `components/ui/` 存放基础组件

---

## 核心功能开发阶段

Date: 2026-05-20 → 2026-05-25
Type: 功能开发

### 启动项管理系统

**功能演进**：

1. 基础 CRUD（创建、读取、更新、删除）
2. 支持多种类型（文件、目录、URL、多任务）
3. 支持自定义图标和参数
4. 支持管理员权限启动

**技术实现**：

- 使用 SeaORM 管理数据
- 支持拖拽导入文件
- 自动识别程序参数（通过 `.lnk` 文件解析）
- 支持批量操作

**遇到的问题**：

- 中文路径编码问题（GBK vs UTF-8）
- `.lnk` 文件解析需要 Windows COM 接口
- 多参数启动时参数传递问题

**解决方案**：

- 使用 `encoding_rs` 处理 GBK 编码
- 使用 `lnk_parser` crate 解析快捷方式
- 重构启动逻辑，统一参数处理

### 搜索系统

**功能设计**：

- 独立搜索窗口（600x45，透明，置顶）
- 支持拼音搜索（全拼和首字母）
- 支持网络搜索集成
- 自动补全建议

**技术实现**：

- 使用 `pinyin` crate 实现拼音转换
- 存储 `pinyin_full` 和 `pinyin_abbr` 字段
- 搜索时同时匹配名称、拼音、关键词

**设计决策**：

- 选择独立窗口而非覆盖层：避免与主窗口交互冲突
- 使用拼音索引：中文用户友好，提升搜索效率

### 分类系统

**功能演进**：

1. 基础分类管理
2. 支持树形结构（自引用外键）
3. 支持目录关联（自动同步文件变化）
4. 支持布局和排序配置

**技术挑战**：

- 删除分类时的级联删除问题
- 目录监控的性能和实时性

**解决方案**：

- 使用 SeaORM 的级联删除关系
- 使用 `notify` crate 监控文件系统变化

### 多窗口架构

**窗口设计**：

- 主窗口（800x600）：启动器界面
- 搜索窗口（600x45）：快速搜索
- 设置窗口（600x500）：配置面板
- 操作窗口（600x400）：启动项编辑
- 剪贴板通知（320x130）：剪贴板监控提示

**技术实现**：

- 所有窗口加载同一个 Vue SPA，通过路由区分
- 使用 `@tauri-store/pinia` 同步状态
- 使用 Tauri 事件系统进行跨窗口通信

**设计决策**：

- 选择多窗口而非单窗口：每个功能独立，避免界面复杂
- 所有窗口初始隐藏：通过代码控制显示/隐藏，提升启动速度

---

## 功能完善阶段

Date: 2026-05-25 → 2026-05-27
Type: 功能增强

### 设置系统

**功能模块**：

- 常规设置（启动、窗口行为）
- 主题设置（亮色/暗色/跟随系统）
- 快捷搜索配置
- 网络搜索配置
- 翻译功能配置
- 传送门配置
- 数据管理（备份/导入/重置）

**技术实现**：

- 配置存储在 SQLite（`configs` 表）
- 同时缓存在内存（`AppConfigState`）
- 使用 `@tauri-store/pinia` 跨窗口同步

**设计决策**：

- 双写策略：写入 DB 同时更新内存，确保一致性
- 启动时重试加载：最多重试 20 次，确保配置加载成功

### 主题系统

**功能演进**：

1. 基础亮色/暗色切换
2. 跟随系统主题
3. 自定义主题属性（字体、圆角、模糊、透明度）

**技术实现**：

- 使用 CSS 变量（oklch 色彩空间）
- 使用 ViewTransition API 实现平滑切换
- 使用 `matchMedia` 监听系统主题变化

**UI/UX 优化**：

- 半透明效果和模糊背景
- 统一的设计令牌系统
- 响应式主题切换

### 传送门功能

**功能定义**：
快速启动和访问的入口，支持：

- 剪贴板内容监控和快速打开
- 常用命令快速执行
- 目录快速访问

**技术实现**：

- 使用 `arboard` crate 监控剪贴板
- 300ms 轮询间隔（平衡响应速度和 CPU 占用）
- 自动识别 URL、目录、文件

### 快捷键系统

**功能设计**：

- 全局快捷键呼出搜索窗口
- 应用内快捷键操作（删除、重命名、导航）
- 支持用户自定义快捷键

**技术实现**：

- 使用 `@tauri-apps/plugin-global-shortcut` 注册全局快捷键
- 使用 Vue 监听键盘事件实现应用内快捷键
- 快捷键冲突检测和提示

---

## 质量提升阶段

Date: 2026-05-27 → 2026-05-29
Type: 重构与优化

### 组件体系重构

**重构动机**：

- 基础组件散落在 `components/` 目录
- 缺乏清晰的组件分层
- 组件命名不一致

**重构方案**：

- 新建 `components/ui/` 存放基础 UI 组件
- 迁移 `DescText`、`Kbd` 等基础组件
- 新增 `ShortcutKbd`、`ShortcutKeyInput`、`ToggleGroup` 等组件
- 统一组件命名和接口设计

**效果**：

- 组件职责清晰
- 复用性提升
- 代码可维护性提高

### 设置面板优化

**优化内容**：

- 重构 `SettingItem` 组件，提升复用性
- 统一 `SettingSelectItem`、`SettingSwitchItem` 风格
- 优化 7 个设置面板的代码结构

**技术决策**：

- 使用组合式函数封装设置逻辑
- 统一错误处理和加载状态

### 代码质量提升

**工程化改进**：

- 执行全面代码格式化（Prettier + cargo fmt）
- 建立 ESLint 规范
- 建立 CLAUDE.md 工作记忆文件
- 创建发布自动化脚本

**工具链**：

- `pnpm fmt`：统一格式化前端和后端代码
- `script/updateVersion.js`：自动化版本发布流程
- `.claude/settings.json`：Claude Code hooks 配置

---

## 关键技术决策记录

### 1. 数据库 Schema 同步

**决策**：使用 SeaORM 的 `schema-sync` 功能

**原因**：

- 自动同步实体定义到数据库
- 避免手动编写 migration
- 适合快速迭代开发

**权衡**：

- ✅ 开发效率高
- ❌ 生产环境可能需要 migration 支持

### 2. 多窗口状态同步

**决策**：使用 `@tauri-store/pinia` 插件

**原因**：

- 官方推荐方案
- 自动处理跨窗口同步
- 与 Pinia 无缝集成

**权衡**：

- ✅ 开发简单
- ❌ 依赖第三方插件
- ❌ 同步粒度有限

### 3. 剪贴板监控

**决策**：使用轮询（300ms）而非系统事件

**原因**：

- Windows 剪贴板事件 API 复杂且不稳定
- 轮询实现简单，兼容性好
- 300ms 间隔平衡响应速度和性能

**权衡**：

- ✅ 实现简单
- ❌ 有轻微性能开销
- ❌ 非实时（300ms 延迟）

### 4. 搜索拼音索引

**决策**：存储 `pinyin_full` 和 `pinyin_abbr` 字段

**原因**：

- 中文用户拼音搜索习惯
- 避免运行时拼音转换开销
- 支持首字母快速搜索

**权衡**：

- ✅ 搜索速度快
- ❌ 存储空间增加
- ❌ 需要维护拼音字段一致性

### 5. 进程执行方式

**决策**：始终使用 `CREATE_NO_WINDOW` 标志

**原因**：

- 后台进程不应弹出控制台窗口
- 提升用户体验
- 符合桌面应用规范

**实现**：

```rust
CommandExt::creation_flags(0x08000000) // CREATE_NO_WINDOW
```

---

## 项目演进时间线

```
2026-05-20  项目初始化
    ↓
2026-05-20  核心功能开发（启动项、搜索、分类）
    ↓
2026-05-25  功能完善（设置、主题、传送门）
    ↓
2026-05-27  v0.1.1 发布
    ↓
2026-05-27  质量提升（重构、优化）
    ↓
2026-05-29  v0.1.2 开发中
```

---

## 技术债务追踪

### 已知问题

1. **数据库 Migration**
   - 当前使用 `schema-sync`，生产环境可能需要正式 migration
   - 优先级：中

2. **错误处理**
   - 部分命令的错误处理不够完善
   - 需要统一错误类型和处理方式
   - 优先级：中

3. **测试覆盖**
   - 目前缺少单元测试和集成测试
   - 需要建立测试框架
   - 优先级：高

4. **性能优化**
   - 部分操作可能有性能瓶颈（如大量启动项时的搜索）
   - 需要 profiling 和优化
   - 优先级：低

### 优化机会

1. **组件复用**
   - 继续提取可复用组件
   - 建立组件文档
   - 优先级：中

2. **代码分割**
   - 考虑按窗口分割代码
   - 减少初始加载时间
   - 优先级：低

3. **国际化**
   - 目前硬编码中文
   - 需要支持多语言
   - 优先级：低

---

## 经验总结

### 做得好的

1. **架构设计**：多窗口 + API 抽离 + 状态同步，架构清晰
2. **技术选型**：Tauri + Vue 3 + SeaORM，技术栈合适
3. **代码组织**：命令分离、组件分层，可维护性好
4. **文档记录**：CLAUDE.md 和开发日志，上下文完整

### 需要改进的

1. **测试**：缺少自动化测试
2. **错误处理**：部分错误处理不够完善
3. **性能**：部分功能可能有性能瓶颈
4. **文档**：组件和 API 文档不足

### 关键学习

1. **Tauri 多窗口**：共享 SPA + 路由区分是有效方案
2. **跨窗口状态同步**：`@tauri-store/pinia` 简化了实现
3. **SeaORM**：适合快速迭代，但生产环境需要 migration
4. **拼音搜索**：预计算拼音索引是高效方案

---

## 后续规划

### 短期（v0.1.2）

- [x] 组件体系重构
- [x] 快捷键系统完善
- [ ] 性能优化
- [ ] 错误处理完善

### 中期（v0.2.0）

- [ ] 添加单元测试
- [ ] 完善错误处理
- [ ] 性能优化和 profiling
- [ ] 组件文档

### 长期（v1.0.0）

- [ ] 国际化支持
- [ ] 插件系统
- [ ] 云端同步
- [ ] 更多平台支持

---

## 2026-05-29 v0.1.2 功能开发

Type: 功能开发 / 优化 / 修复

### 变更概述

v0.1.2 版本的核心功能开发，包括传送门功能增强、性能优化和 bug 修复。

### 具体变更

#### 新功能：传送门命令行集成

- **功能**：支持在命令行中打开目录
- **场景**：开发者快速在终端中打开当前目录，提升工作流效率
- **实现**：新增 `open_dir_in_terminal` Tauri 命令

#### 性能优化

- **字体列表优化**：改进 `get_local_fonts` 命令性能，减少枚举开销
  - 优化原因：大量字体时加载缓慢
  - 优化方案：异步加载 + 缓存策略
- **快捷键录制优化**：提升快捷键录制响应速度和准确性
  - 优化原因：用户反馈录制延迟
  - 优化方案：改进事件处理逻辑

#### Bug 修复

- **更新功能修复**：修复应用更新无效的问题
  - 问题描述：用户无法正常更新应用
  - 影响范围：所有用户的自动更新功能
  - 修复方案：修复更新检查和下载逻辑

### 技术决策

**传送门命令行集成设计**

- 选择方案：独立 Tauri 命令而非前端直接调用
- 原因：需要系统级权限执行 `cmd /K cd /d` 命令
- 实现：使用 `CommandExt::creation_flags(CREATE_NO_WINDOW)` 避免弹出控制台

**性能优化策略**

- 字体枚举：从同步改为异步，避免阻塞 UI 线程
- 快捷键：优化事件监听器，减少不必要的事件处理

**更新功能修复**

- 根本原因：更新检查逻辑未正确处理版本比较
- 修复方案：重新实现版本比较算法，确保语义化版本正确解析

### 代码变更统计

- **新增文件**：1 个命令文件
- **修改文件**：5 个文件
- **代码行数**：+120 行 / -30 行
- **测试覆盖**：无（需补充）

### 后续工作

- [ ] 补充单元测试覆盖
- [ ] 添加更新功能的集成测试
- [ ] 监控生产环境更新成功率
- [ ] 收集传送门命令行功能的用户反馈

### Notes

- 本次变更聚焦于用户体验提升和稳定性改进
- 传送门命令行功能是开发者高频使用场景的重要增强
- 性能优化和 bug 修复提升了应用的整体质量

---

## 2026-05-30 00:00

Type: 重构 / 修复

### 变更概述

优化主题切换相关代码，引入 VueUse 替代手动实现，并新增多种 ViewTransition 过渡动画效果。同时修复主窗口位置限制问题和清理本地数据库文件。

### 具体变更

#### 主题切换重构

- **useTheme.ts**：使用 VueUse 的 `usePreferredDark()` 替代手动 `window.matchMedia` 监听，自动管理事件生命周期
- **App.vue**：移除 `onMounted`/`onUnmounted` 中的手动 mediaQuery 监听，用 `watchImmediate` 合并 font/fontSize 的初始化与响应式更新
- **ThemeSwitch.vue**：将 `watch(themeModel)` + `onOsThemeChange` + `syncSwitchState` 三段逻辑合并为单个 `watch(isDark)`
- **新增 CHANGE_THEME 事件**：支持跨窗口主题同步，其他窗口切换主题时自动应用 ViewTransition 动画
- **样式迁移**：将 Setting 相关样式从 `global.css` 迁移至组件 scoped 样式，避免全局污染

#### ViewTransition 过渡效果

在 `useTheme.ts` 中预置 12 种过渡效果，均以注释形式存放，按需启用：

- 圆形扩散（当前生效）、圆形扩散+模糊、菱形扩散
- 左/右/上/下滑入、淡入淡出、缩放
- 水平百叶窗、垂直分割
- 翻页（3D perspective + rotateY + brightness 模拟折叠光影）

#### 其他修复

- **移除主窗口位置非负数限制**：允许窗口定位到负坐标（多显示器场景）
- **删除 Date.db**：清理误提交的本地数据库文件

### 技术决策

**为什么用 usePreferredDark 替代手动 matchMedia**

- VueUse 的 `usePreferredDark()` 自动管理 `MediaQueryList` 的事件监听和清理
- 返回响应式 `Ref<boolean>`，可直接用于 `computed`
- 避免了手动 `addEventListener`/`removeEventListener` 的生命周期管理问题

**为什么 watchImmediate 放在 App.vue 而非 useTheme()**

- `useTheme()` 可能被同一窗口的多个组件调用
- 如果 `watchImmediate` 放在 `useTheme()` 内部，会导致 watcher 重复创建
- 放在 `App.vue`（根组件）确保每个窗口只执行一次

**翻页效果的设计思路**

- 旧页：`perspective(1500px) rotateY` 旋转折叠 + `brightness` 渐暗模拟阴影
- 新页：延迟 200ms 从对侧旋转展开 + `brightness` 渐亮
- 使用 3 段关键帧（0° → 30° → 90°）模拟自然折叠节奏

## 2026-05-30 00:30

Type: 功能开发

### 变更概述

实现应用国际化（i18n）功能，支持简体中文、繁體中文、English、日本語四种语言，并修复了长文本语言下的布局适配问题。

### 具体变更

#### 国际化基础设施

- 引入 `vue-i18n` 11.x 依赖
- 新建 `src/i18n/lang.ts`：所有翻译数据集中在一个文件，使用数组格式 `[zh-CN, zh-HK, en, ja]`
- 新建 `src/i18n/index.ts`：vue-i18n 实例 + 数组→对象转换函数 + 导出 `t` 函数
- `src/main.ts`：注册 i18n 插件
- `src/App.vue`：集成 NaiveUI locale（zhCN/zhTW/enUS/jaJP）+ 语言切换联动

#### 翻译覆盖（170+ 文本）

- 11 个设置页面组件（General、Theme、QuickSearch、WebSearch、Translation、Proxy、Data、Portal、CommandAlias、About、Setting）
- 7 个主窗口/共享组件（Header、Footer、ListItem、4 个右键菜单）
- 4 个操作/搜索窗口组件（OperationLaunch、OperationCategory、Search、LaunchList）
- 辅助组件（BrowserPicker、IconPicker、OpenDemoVideo、ShortcutKeyInput）
- 工具函数 `formatLaunchType.ts`

#### 布局适配

- `SettingItem.vue`：左侧容器添加 `min-w-0`，title 添加 `truncate`，description 改为 `line-clamp-2`，右侧控件添加 `flex-shrink-0`
- `SettingGroup.vue`：移除 `uppercase tracking-wider`，添加 `truncate`
- `SettingSelectItem.vue`：n-select 固定 `width: 160px`

#### 文档更新

- `CLAUDE.md`：补充 i18n 功能说明、vue-i18n 技术栈、i18n 文件引用

### 技术决策

**翻译数据用数组而非对象**

- 所有语言的翻译放在同一个文件中，数组顺序固定对应 `[zh-CN, zh-HK, en, ja]`
- 在 `i18n/index.ts` 中通过 `buildLocaleMessages` 转换为 vue-i18n 需要的嵌套对象格式
- 优点：便于对照翻译、减少文件数量、添加新语言只需扩展数组

**使用 `t` 导出而非 `useI18n()`**

- 从 `@/i18n` 直接导出 `t = i18n.global.t`
- 组件中直接 `import { t } from '@/i18n'`，无需每次调用 `useI18n()`
- 简化了使用方式，减少样板代码

**SettingItem 的 `min-w-0` + `flex-shrink-0` 模式**

- flex 子元素默认 `min-width: auto` 会阻止收缩，添加 `min-w-0` 允许文本区域被压缩
- 右侧控件添加 `flex-shrink-0` 保证不被挤压
- 中文文本短不会触发 truncate，长文本语言下自动省略

### 后续工作

- [ ] 将 ViewTransition 过渡效果改为可配置
- [ ] 研究 `visibility: hidden` 方案解决主题切换时的颜色闪烁

## 2026-05-30 02:00

Type: 功能开发

### 变更概述

将托盘菜单从 Rust 端迁移到前端创建，实现托盘菜单国际化；补充通知类方法的 i18n 翻译。

### 具体变更

#### 托盘菜单前端化

- 新建 `src/composables/useTray.ts`：使用 `TrayIcon.new()` + `Menu.new()` 从前端创建托盘
- 移除 Rust 端 `create_tray(app)` 调用（`lib.rs`）
- 托盘菜单文本直接使用 `t('tray.xxx')` 翻译，语言切换时自动更新
- 通过主窗口判断 + `TrayIcon.getById` 避免重复创建

#### 通知类方法 i18n

- `useAppVersion.ts`：更新检查相关的 5 个通知文案翻译
- `useCategoryCorrelationDir.ts`：分类关联目录的 dialog 和 message 翻译
- `message.ts`：新增 `update` 和 `categoryDir` 翻译模块

#### 其他

- `useToggleWindowVisible.ts`：`toogleMainWindowVisible` 添加 `center` 参数，修复 show/focus 时序

### 技术决策

**为什么从 Rust 迁移到前端创建托盘**

- Rust 端托盘菜单文本硬编码，无法直接使用 vue-i18n
- 前端创建可直接调用 `t()` 函数，语言切换时自动重建菜单
- Tauri v2 的 `@tauri-apps/api/tray` 和 `@tauri-apps/api/menu` 提供完整 API
- 权限已通过 `core:default` 授予，无需额外配置

**如何避免重复创建托盘**

- 仅在主窗口（`label === 'main'`）执行创建
- 语言切换时先检查 `TrayIcon.getById('tray')`，存在则只更新菜单文本

---

## 2026-05-31 10:00

Type: 重构 / 功能开发

### 变更概述

将搜索窗口从单体组件拆分为独立模式组件架构，并将 TodoMode 数据存储从 localStorage 迁移到 SQLite 数据库。

### 具体变更

#### 搜索窗口架构重构

原 `Search.vue` 为 890 行的单体组件，混合了 3 种搜索模式的逻辑。重构后：

- **Search.vue**（~220 行）：纯容器组件，负责窗口管理（显示/隐藏/定位/聚焦）和模式切换
- **DefaultSearchMode.vue**：默认搜索模式，自处理键盘事件（Enter/↑↓/Tab/→/Esc/Ctrl+W/Space×3→翻译/Space→Web搜索）
- **WebSearchMode.vue**：网络搜索模式，自处理 Enter/↑↓/Esc
- **TranslationMode.vue**：翻译模式，自处理 Tab（语言切换）/Esc/Enter/↑↓
- **TodoMode.vue**：待办模式，自处理 Enter/Ctrl+Enter/Esc
- **searchModes.ts**：共享常量（模式枚举、Tab 栏高度）

**架构设计**：

- 每个模式组件完全自包含，自行管理键盘事件和业务逻辑
- 通过 `defineExpose` 暴露统一接口：`focus()`、`handleKeydown()`、`handleClose()`、`getDefaultHeight()`
- Search.vue 使用 `<component :is="activeModeComponent">` 动态渲染
- 子组件通过 emit `closeWindow` 和 `switchMode` 事件与父组件通信
- Shift+Tab 在 Search.vue 层面处理，用于循环切换模式

**过渡效果**：

- TodoMode 内部 4 个视图状态（empty/create/list/detail）使用 `<transition>` 包裹
- 采用 `position: absolute` + `width: 100%` 解决过渡时的布局位移问题
- `v-if` 而非 `v-else-if`，因为多个 `<transition>` 包裹时 v-else 不生效

#### TodoMode 数据库迁移

将 TodoMode 从 localStorage 迁移到 SQLite，新增 `todos` 表：

**后端（Rust）**：

- 新增 `entity/todos.rs`：SeaORM 实体定义（id, title, completed, priority, due_date, tags, note, reminder_at, created_at, updated_at）
- 新增 5 个 Tauri 命令：`get_todos`、`add_todo`、`update_todo`、`delete_todo`、`clear_completed_todos`
- Schema 自动同步：通过 SeaORM 的 `schema-sync` 功能，启动时自动建表

**前端（TypeScript）**：

- `types/index.d.ts`：新增 `TodoItem` 接口和 `TodoPriority` 类型
- `api/index.ts`：新增 5 个 API 封装函数
- `constant/index.ts`：新增 5 个 `InvokeMethod` 枚举
- `TodoMode.vue`：移除 localStorage 逻辑，所有 CRUD 操作通过 API 调用数据库

**数据结构变化**：

| 字段 | localStorage（旧） | SQLite（新） |
|------|-------------------|-------------|
| id | `string` (UUID) | `number` (自增) |
| tags | `string[]` | `string` (JSON) |
| dueDate | `string` | `due_date: string` |
| createdAt | `string` | `created_at: string` |
| reminder_at | 无 | `string` (新增，预留给通知功能) |

### 技术决策

**为什么拆分而非条件渲染**

- 原 Search.vue 中 keydown handler 通过 switch 分发到各模式方法，耦合严重
- 拆分后每个模式独立演进，添加新模式只需新建组件 + 注册到 searchModes
- `<component :is>` 动态组件比 v-if 链更清晰

**为什么用数据库替代 localStorage**

- 多窗口同步：localStorage 需手动监听 storage 事件，数据库通过 API 天然一致
- 数据备份：localStorage 不包含在 `backup_database` 中
- 通知功能：Rust 后端可定时扫描数据库，localStorage 无法后台访问
- 查询能力：支持 SQL 筛选、排序、分页

**tags 字段用 JSON 字符串而非独立表**

- 待办标签数量少（通常 0-5 个），无需关联查询
- 简化 schema，避免额外的关联表
- 前端通过 `parseTags()`/`stringifyTags()` 转换

**chrono_now 自实现而非引入 chrono crate**

- 仅需格式化当前时间，功能简单
- 避免增加编译时间和二进制大小
- 使用 `SystemTime` + 手动计算年月日

### 后续工作

- [ ] 通知功能：添加 `tauri-plugin-notification`，Rust 后端定时扫描 `reminder_at` 到期的待办
- [ ] TodoMode 提醒 UI：在 detail 视图增加 datetime picker 设置提醒时间
- [ ] 旧组件清理：确认功能稳定后删除 `DefaultSearch.vue`、`WebSearch.vue`
