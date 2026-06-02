# QuickLauncher - Claude Code 工作记忆

## 项目概述

Windows 快速启动器桌面应用，基于 Tauri v2（Rust 后端）+ Vue 3（TypeScript 前端）构建。包管理器：pnpm。

---

## 1. 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                    前端 (Vue 3 + TypeScript)                 │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │
│  │  主窗口  │ │ 搜索窗口 │ │ 设置窗口 │ │ 操作窗口 │           │
│  │  Main   │ │ Search  │ │ Setting │ │ OperXXX │           │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘           │
│       │           │           │           │                 │
│  ┌────┴───────────┴───────────┴───────────┴────┐            │
│  │              Pinia Store (跨窗口同步)         │            │
│  │         @tauri-store/pinia 插件              │            │
│  └─────────────────────┬───────────────────────┘            │
│                        │ invoke()                           │
├────────────────────────┼────────────────────────────────────┤
│                   Rust 后端 (Tauri v2)                       │
│  ┌─────────────────────┴───────────────────────┐            │
│  │           45 个 Tauri 命令                    │            │
│  │       src-tauri/src/commands/*.rs            │            │
│  └─────────────────────┬───────────────────────┘            │
│                        │                                    │
│  ┌─────────────────────┴───────────────────────┐            │
│  │         SeaORM (SQLite: Date.db)             │            │
│  │       src-tauri/src/entity/*.rs              │            │
│  └─────────────────────────────────────────────┘            │
│                                                             │
│  ┌─────────────────────────────────────────────┐            │
│  │  12 个 Tauri 插件 (剪贴板, 文件系统, HTTP...) │            │
│  │  Win32 API (windows crate) - 托盘, COM, GDI  │            │
│  └─────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

### 前端结构

```
src/
├── main.ts                    # 入口文件
├── App.vue                    # 根组件：NaiveUI providers + router-view
├── api/index.ts               # ~50 个 Tauri invoke() 封装
├── router/index.ts            # 6 个路由（hash history），与 Tauri 窗口 1:1 对应
├── store/
│   ├── useStore.ts            # 主数据存储（启动项、分类）
│   └── useAppConfigStore.ts   # ~80 个配置字段
├── composables/               # 14 个组合式函数（useTheme, useToggleWindowVisible 等）
├── i18n/                      # 国际化
│   ├── index.ts               # vue-i18n 实例 + 数组转对象转换函数
│   └── lang.ts                # 所有翻译数据（单一文件，数组格式 [zh-CN, zh-HK, en, ja]）
├── views/
│   ├── Main/                  # 主启动器窗口
│   ├── Search/                # 快速搜索覆盖层
│   ├── Setting/               # 10 个设置面板
│   ├── OperationLaunch/       # 添加/编辑启动项
│   ├── OperationCategory/     # 添加/编辑分类
│   └── ClipboardToast/        # 剪贴板通知
├── components/                # 共享组件 + UI 基础组件
├── constant/index.ts          # 枚举：InvokeMethod, AppEvent 等
└── styles/global.css          # Tailwind 4 + CSS 设计令牌 (oklch)
```

**核心模式：**

- 所有 API 调用通过 `src/api/index.ts` → `invoke()` → Rust 命令
- 跨窗口同步通过 `@tauri-store/pinia` 插件
- 跨窗口事件通过 `src/utils/eventBus.ts`（Tauri 事件系统）
- UI：Naive UI + Tailwind CSS 4，自动导入（`unplugin-vue-components`）
- 国际化：vue-i18n，翻译数据集中在 `src/i18n/lang.ts`，使用数组格式 `[zh-CN, zh-HK, en, ja]`，通过 `useI18n()` 的 `t()` 函数在组件中使用

### Rust 后端结构

```
src-tauri/src/
├── main.rs                    # 二进制入口 (windows_subsystem = "windows")
├── lib.rs                     # 应用构建器：插件、命令、状态、托盘
├── logging.rs                 # Tracing + 每日轮转文件日志
├── tray.rs                    # 系统托盘设置
├── commands/                  # 45 个 Tauri 命令（每个命令一个文件）
├── clipboard/listener.rs      # 剪贴板轮询（300ms，arboard）
├── common/utils.rs            # Win32 全屏检测
├── db/connection.rs           # SeaORM SQLite 初始化 + schema 同步
├── entity/                    # 5 个 SeaORM 实体
│   ├── launch_items.rs        # 启动项（文件/目录/URL/别名/应用组）
│   ├── categories.rs          # 分类（自引用树结构）
│   ├── configs.rs             # 键值配置存储
│   ├── launch_history.rs      # 启动历史
│   └── autocomplete_history.rs # 自动补全建议
└── models/                    # DTO 和状态结构体
```

**核心模式：**

- 状态：`AppState { db: Mutex<Option<DatabaseConnection>> }` + `Mutex<AppConfigState>`
- 每个命令从 mutex 克隆数据库连接，执行异步操作，返回结果
- Windows API 通过 `windows` crate (v0.58)：COM 对话框、GDI、全屏检测
- 进程执行：始终使用 `CREATE_NO_WINDOW` 标志，`cmd /C start` 模式

### 多窗口架构

| 窗口       | 标签             | 尺寸    | 路由              | 用途           |
| ---------- | ---------------- | ------- | ----------------- | -------------- |
| 主窗口     | `main`           | 800x600 | `/main`           | 主启动器界面   |
| 搜索窗口   | `search`         | 600x45  | `/search`         | 快速搜索覆盖层 |
| 设置窗口   | `setting`        | 600x500 | `/setting`        | 设置面板       |
| 启动项编辑 | `operLaunch`     | 600x400 | `/operLaunch`     | 启动项编辑器   |
| 分类编辑   | `operCategory`   | 600x350 | `/operCategory`   | 分类编辑器     |
| 剪贴板通知 | `clipboardToast` | 320x130 | `/clipboardToast` | 剪贴板通知弹窗 |

所有窗口初始隐藏，无装饰栏，通过代码管理显示/隐藏。

---

## 2. 关键路径（未经理解请勿修改）

### 版本文件（必须保持同步）

以下 3 个文件必须始终保持相同的版本号：

- `package.json` → `"version": "x.y.z"`
- `src-tauri/Cargo.toml` → `version = "x.y.z"`
- `src-tauri/tauri.conf.json` → `"version": "x.y.z"`

**始终使用 `script/updateVersion.js` 修改版本号。**

### 数据库 Schema

- `src-tauri/src/entity/*.rs` — SeaORM 实体，包含级联删除关系
- Schema 在启动时通过 `schema-sync` 功能自动同步
- 修改实体文件会影响运行时数据库结构

### 签名配置

- `src-tauri/tauri.conf.json` → `plugins.updater.pubkey` — 更新验证公钥
- `script/updateVersion.js` → `TAURI_SIGNING_PRIVATE_KEY` + `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — 签名凭据

### Tauri 插件注册

- `src-tauri/src/lib.rs` — 所有 12 个插件在此注册
- 添加/移除插件需要同时更新 `Cargo.toml` 和 `lib.rs`

### 窗口配置

- `src-tauri/tauri.conf.json` → `app.windows[]` — 6 个窗口定义
- `src-tauri/src/capabilities/` — 插件权限授予

---

## 3. 发布流程

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  git commit  │────▶│ script/      │────▶│ pnpm tauri   │
│  (约定式提交) │     │ updateVer-   │     │ build --ci   │
│              │     │ sion.js      │     │              │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                     更新 3 个文件：        使用环境变量签名：
                     - package.json        - TAURI_SIGNING_PRIVATE_KEY
                     - Cargo.toml          - TAURI_SIGNING_PRIVATE_KEY_PASSWORD
                     - tauri.conf.json     ──────────────────────
                            │              │
                     生成：                │
                     - latest.json         ▼
                     - git tag v{x.y.z}  ┌──────────────┐
                            │             │ NSIS .exe    │
                            │             │ + .sig 签名  │
                            ▼             └──────┬───────┘
                     打开浏览器：                 │
                     GitHub releases/new          ▼
                                         上传构建产物
                                         到 GitHub Release
```

### 自动化步骤（通过 script/updateVersion.js）

1. **版本选择** — 交互式提示（重大/功能/次要/自定义/跳过）
2. **版本传播** — 同时更新 3 个文件
3. **构建** — `pnpm tauri build --ci`，注入签名环境变量
4. **签名提取** — 读取 `QuickLauncher_{version}_x64-setup.exe.sig`
5. **发布说明** — 从 `git log` 自动生成，按约定式提交类型分组
6. **生成 latest.json** — 更新清单，包含版本、说明、签名、下载链接
7. **Git 标签** — 创建带注释的标签 `v{version}`
8. **打开浏览器** — 打开 GitHub releases/new 页面

### 手动步骤（开发者必须执行）

1. 推送 git 标签：`git push origin v{x.y.z}`
2. 上传到 GitHub Release：
   - `src-tauri/target/release/bundle/nsis/QuickLauncher_{version}_x64-setup.exe`
   - `latest.json`

### 更新器流程（客户端）

1. 应用检查 `https://github.com/Peng1203/QuickLauncher/releases/latest/download/latest.json`
2. 比较版本，使用 `tauri.conf.json` 中的公钥验证签名
3. 下载 `.exe`，静默安装

---

## 4. Claude 安全规则

### 前端 UI 风格一致性（VERY IMPORTANT）

新增页面、组件、弹窗、设置面板时，必须优先复用现有设计风格与组件结构，避免创建风格割裂的新 UI。

必须遵循：

- 优先复用已有组件（尤其是 `src/components/` 与 `Setting/` 相关组件）
- 优先保持与现有页面一致的：
  - 圆角
  - 间距（padding / gap）
  - 字体层级
  - border 样式
  - hover/active 动效
  - 深色模式表现
- 优先使用：
  - Tailwind CSS utility class
  - Naive UI 组件
  - 现有 CSS 设计令牌（`src/styles/global.css`）
- 新页面应尽量保持：
  - macOS / 桌面工具风格
  - 半透明层次感
  - 柔和边框与背景
  - 高密度但清晰的信息布局

禁止：

- 引入与当前项目风格明显冲突的 UI 风格
- 使用与现有页面差异过大的颜色体系
- 随意创建新的 design token
- 在未参考现有页面结构前完全重新设计 UI

在新增 UI 前：

- 必须先参考现有 views/components 的实现风格
- 优先参考：
  - `src/views/Setting`
  - `src/components`
  - 已存在的展开面板与设置项结构

### 未经明确指示绝不可修改

- `src-tauri/src/lib.rs` — 插件注册、命令处理器、状态设置
- `src-tauri/tauri.conf.json` — 窗口配置、插件配置、打包配置、更新器公钥
- `src-tauri/src/entity/*.rs` — 数据库 schema（变更影响运行时数据库）
- `src-tauri/src/db/connection.rs` — 数据库初始化
- `script/updateVersion.js` — 发布自动化、签名凭据
- `src/api/index.ts` — API 层必须与 Rust 命令签名匹配
- `src/constant/index.ts` — InvokeMethod 枚举必须与 Rust 命令名匹配

### 必须使用脚本/工具

- **版本更新** → `node script/updateVersion.js`（绝不手动编辑版本字段）
- **前端构建** → `pnpm build`（运行类型检查 + vite 构建）
- **完整应用构建** → `pnpm tauri build`（前端 + Rust + 打包）
- **代码检查** → `pnpm lint` 或 `pnpm lint:fix`
- **代码格式化** → `pnpm fmt`（prettier + cargo fmt）

### 执行前必须确认

- 任何 `Cargo.toml` 依赖变更（影响构建时间、二进制大小）
- 任何 `tauri.conf.json` 变更（影响窗口行为、插件、更新器）
- 任何实体文件变更（影响数据库 schema）
- 运行 `pnpm tauri build`（需要几分钟）

### 可自由修改

- `src/views/**/*.vue` — 视图组件
- `src/components/**/*.vue` — 共享组件
- `src/composables/*.ts` — Vue 组合式函数
- `src/store/*.ts` — Pinia 存储（但需理解同步影响）
- `src/styles/*.css` — 样式
- `src/utils/*.ts` — 工具函数
- `src-tauri/src/commands/*.rs` — 单个命令实现（添加新命令需要更新 `lib.rs`）

---

## 5. 开发命令

### 开发

```bash
pnpm dev                    # 启动 Vite 开发服务器（仅前端）
pnpm tauri dev              # 以开发模式启动完整 Tauri 应用
```

### 构建

```bash
pnpm build                  # 类型检查 + Vite 生产构建
pnpm tauri build            # 完整 Tauri 构建（前端 + Rust + 打包）
pnpm tauri build --ci       # 非交互式构建（用于发布脚本）
```

### 代码质量

```bash
pnpm lint                   # ESLint 检查
pnpm lint:fix               # ESLint 自动修复
pnpm format                 # Prettier 格式化前端代码
pnpm fmt                    # 同时格式化前端（prettier）+ 后端（cargo fmt）
```

### 发布

```bash
node script/updateVersion.js    # 交互式发布脚本
```

### Git

```bash
git tag -l                  # 列出所有标签
git describe --tags --abbrev=0  # 获取最新标签
git log v0.1.1..HEAD --oneline  # 查看上次发布以来的提交
```

---

## 6. 技术栈参考

| 层级       | 技术               | 版本     |
| ---------- | ------------------ | -------- |
| 桌面运行时 | Tauri              | 2.x      |
| 前端框架   | Vue 3              | 3.5      |
| 语言       | TypeScript         | 5.6      |
| 构建工具   | Vite               | 8.x      |
| UI 框架    | Naive UI           | 2.44     |
| CSS        | Tailwind CSS       | 4.3      |
| 状态管理   | Pinia              | 3.x      |
| 多窗口同步 | @tauri-store/pinia | 4.x      |
| 路由       | Vue Router         | 4.x      |
| 国际化     | vue-i18n           | 11.x     |
| 后端语言   | Rust               | —        |
| ORM        | SeaORM             | 2.0.0-rc |
| 数据库     | SQLite             | —        |
| 包管理器   | pnpm               | 10.12    |

---

## 7. 关键文件参考

| 文件                        | 用途                                                 |
| --------------------------- | ---------------------------------------------------- |
| `package.json`              | 前端依赖、脚本、版本号                               |
| `vite.config.ts`            | Vite 配置、路径别名、插件                            |
| `tsconfig.json`             | TypeScript 配置                                      |
| `src-tauri/Cargo.toml`      | Rust 依赖、crate 配置、版本号                        |
| `src-tauri/tauri.conf.json` | Tauri 配置：窗口、插件、打包、更新器                 |
| `src-tauri/src/lib.rs`      | 应用构建器、插件注册、命令处理器                     |
| `src-tauri/src/commands/`   | 45 个 Tauri 命令（每个命令一个文件）                 |
| `src-tauri/src/entity/`     | 5 个 SeaORM 实体（数据库 schema）                    |
| `src-tauri/capabilities/`   | Tauri 权限授予                                       |
| `src/api/index.ts`          | 前端 API 层（~50 个 invoke 封装）                    |
| `src/store/`                | Pinia 存储（主存储 + 应用配置）                      |
| `src/constant/index.ts`     | 枚举：InvokeMethod, AppEvent                         |
| `src/i18n/lang.ts`          | 所有翻译数据（数组格式，顺序：zh-CN, zh-HK, en, ja） |
| `src/i18n/index.ts`         | vue-i18n 实例，数组→对象转换函数                     |
| `script/updateVersion.js`   | 发布自动化脚本                                       |
| `latest.json`               | Tauri 更新器清单                                     |

<!--VITE PLUS START-->

# Using Vite+, the Unified Toolchain for the Web

This project is using Vite+, a unified toolchain built on top of Vite, Rolldown, Vitest, tsdown, Oxlint, Oxfmt, and Vite Task. Vite+ wraps runtime management, package management, and frontend tooling in a single global CLI called `vp`. Vite+ is distinct from Vite, and it invokes Vite through `vp dev` and `vp build`. Run `vp help` to print a list of commands and `vp <command> --help` for information about a specific command.

Docs are local at `node_modules/vite-plus/docs` or online at https://viteplus.dev/guide/.

## Review Checklist

- [ ] Run `vp install` after pulling remote changes and before getting started.
- [ ] Run `vp check` and `vp test` to format, lint, type check and test changes.
- [ ] Check if there are `vite.config.ts` tasks or `package.json` scripts necessary for validation, run via `vp run <script>`.
- [ ] If setup, runtime, or package-manager behavior looks wrong, run `vp env doctor` and include its output when asking for help.

<!--VITE PLUS END-->
