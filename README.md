# Quick Launcher - Windows 快速启动器

![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?style=flat&logo=tauri&logoColor=white)
![Vue.js](https://img.shields.io/badge/Vue.js-3.5-4FC08D?style=flat&logo=vuedotjs&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178C6?style=flat&logo=typescript&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-2021-000000?style=flat&logo=rust&logoColor=white)

基于 Tauri 2 + Vue 3 + TypeScript 的 Windows 桌面快速启动器，提供应用、文件、文件夹、网址的高效启动与管理。

## 功能特性

### 快速启动
- 支持 **文件、文件夹、网址、命令别名、多任务（组合任务）** 五种启动项类型
- 全局快捷键快速呼出搜索窗口（默认 `Alt+Space`）
- 智能拼音搜索（全拼 + 首字母），支持关键词匹配
- 自动补全建议和历史启动记录快速切换
- 支持以管理员权限运行

### 分类管理
- 多级分类目录，自定义图标与排序（按名称/类型/时间/顺序）
- 网格/列表两种布局模式
- 分类可关联本地目录，自动同步目录内文件为启动项（监听文件增删改）
- 支持拖拽排序、批量启用/禁用、搜索排除

### 智能搜索
- 实时模糊搜索，结果按关联度排序
- 网络搜索集成 — 可自定义搜索引擎（Google、Baidu 等），键入前缀触发
- 翻译功能 — 连按 3 次空格进入翻译模式（Baidu Translate API）
- 搜索窗口支持多显示器、跟随鼠标、全屏免打扰

### 便捷操作
- 系统托盘图标（左键显示/隐藏，右键菜单：设置、重启、退出）
- 开机自启动、静默启动
- 剪贴板轮询 — 复制 URL 或文件夹路径时弹出 Toast 快捷操作
- 文件管理器定位（`explorer /select,`）和快捷方式（`.lnk`）路径保留选择
- 图标自动提取（EXE/DLL 系统图标、`.lnk` 目标图标、网站 favicon）

### 现代化界面
- 基于 Naive UI 的简洁界面
- 暗色/亮色主题切换
- 无边框窗口设计，6 个独立窗口各司其职

## 技术栈

| 层       | 技术                                                              |
| -------- | ----------------------------------------------------------------- |
| 前端框架 | Vue 3 + TypeScript + Vite                                         |
| UI 库    | Naive UI + Tailwind CSS                                           |
| 状态管理 | Pinia + pinia-plugin-persistedstate + @tauri-store/pinia（跨窗口同步） |
| 后端框架 | Tauri 2                                                           |
| 系统语言 | Rust (Edition 2021)                                                |
| 数据库   | SQLite（SeaORM ORM + rusqlite）                                   |

### Tauri 插件

| 插件                                    | 用途           |
| --------------------------------------- | -------------- |
| `plugin-global-shortcut`                | 全局快捷键注册 |
| `plugin-clipboard-manager`              | 剪贴板读取     |
| `plugin-autostart`                      | 开机自启动     |
| `plugin-single-instance`                | 单实例运行     |
| `plugin-fs`                             | 文件系统监听   |
| `plugin-process`                        | 进程管理       |
| `plugin-dialog`                         | 文件选择对话框 |
| `plugin-http`                           | HTTP 请求（代理支持） |
| `plugin-opener`                         | 用默认程序打开 |
| `plugin-log`                            | 日志记录       |

## 怎么获取

### 方式一：下载安装包（推荐）

前往 [Releases](https://github.com/your-repo/quicklauncher/releases) 页面下载最新的 `.msi` 或 `.exe` 安装包，双击安装即可。

### 方式二：从源码构建

**环境要求**

- **Node.js** >= 18
- **Rust** >= 1.70
- **pnpm** >= 8（推荐）
- **Windows 10/11**（主要支持平台）

```bash
# 1. 克隆仓库
git clone https://github.com/your-repo/quicklauncher.git
cd quicklauncher

# 2. 安装前端依赖
pnpm install

# 3. 开发模式运行
pnpm tauri dev

# 4. 发布构建（生成安装包）
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/`。

### 卸载

通过 Windows 设置 → 应用 → 找到 "Quick Launcher" → 卸载。应用数据目录（`%APPDATA%/com.quicklauncher.app/`）不会被自动删除，如需彻底清除请手动删除。

## 使用说明

### 主界面

启动后，Quick Launcher 会在系统托盘显示图标。左键点击托盘图标打开主界面，主界面由**左侧分类栏**和**右侧启动项列表**组成。

- **左侧分类栏**：右键创建/编辑/删除分类，支持拖拽排序
- **右侧启动项列表**：右键对启动项进行编辑、删除、复制路径、以管理员运行等操作

### 添加启动项

四种方式添加启动项：

1. **拖拽添加** — 将文件/文件夹从资源管理器拖入主窗口
2. **按钮添加** — 点击主界面添加按钮，在弹窗中填写信息
3. **剪贴板快捷** — 复制文件夹路径后，右下角 Toast 可直接创建
4. **关联目录** — 分类设置中关联目录，应用自动同步目录内所有文件

### 编辑与管理

- **右键启动项** — 编辑、删除、重命名、复制路径、以管理员运行
- **右键分类** — 创建/编辑/删除分类，设置关联目录、图标、排序方式
- **拖拽** — 启动项和分类均支持拖拽排序

### 搜索窗口

按 `Alt+Space` 呼出搜索栏（快捷键可在设置中修改），支持以下搜索模式：

- **启动项搜索** — 输入名称/拼音/关键词，模糊匹配所有启动项
- **命令直行** — 输入任意路径、网址或系统命令，直接回车执行
- **网络搜索** — 输入搜索引擎前缀 + 关键词（如 `g 天气` 用 Google 搜索）
- **翻译模式** — 连续按 3 次空格切换翻译模式，输入内容自动翻译

搜索时可用 `Tab` 切换补全建议，`→` 接受建议，`↑/↓` 切换历史记录。

### 设置面板

右键托盘图标 → 设置，或主界面右上角齿轮图标打开。包含 8 个设置页：

| 设置页   | 功能                                       |
| -------- | ------------------------------------------ |
| 常规     | 开机自启、静默启动、窗口置顶/居中、快捷键配置 |
| 快速搜索 | 搜索快捷键、聚焦行为、免打扰模式、自动补全   |
| 网络搜索 | 自定义搜索引擎（名称/关键词/搜索地址/图标）    |
| 命令别名 | 系统命令快捷方式（控制面板、任务管理器等）     |
| 翻译     | Baidu 翻译 API 配置（App ID / Key）          |
| 网络     | HTTP 代理设置（主机/用户名/密码）             |
| 数据     | 数据库备份、导入、重置、打开数据目录          |
| 传送门   | （功能占位）                                 |

### 快捷键汇总

| 快捷键       | 功能               |
| ------------ | ------------------ |
| `Alt+Space`  | 呼出/隐藏搜索窗口   |
| `Alt+P`      | 呼出/隐藏主窗口     |
| `Esc`        | 关闭搜索窗口       |
| `↑` / `↓`   | 导航搜索结果/历史   |
| `Tab`        | 切换自动补全建议    |
| `→`         | 接受自动补全       |
| `Enter`      | 执行选中项          |
| `Ctrl+Enter` | 以管理员权限运行    |
| `Home`       | 打开剪贴板 URL/目录 |
| `PageUp`     | 文件管理器定位剪贴板路径 |

### 启动项类型

| 类型       | 说明                                        |
| ---------- | ------------------------------------------- |
| 文件       | 应用程序、脚本等可执行文件                    |
| 文件夹     | 打开指定目录                                 |
| 网址       | 在浏览器中打开 URL                           |
| 命令别名   | 系统命令快捷方式（如"控制面板"、"hosts"等）     |
| 多任务     | 顺序执行多个操作，可配置间隔延迟               |

### 剪贴板 Toast

复制 URL 或文件夹路径到剪贴板时，右下角弹出 Toast 通知：
- `Home` — 打开 URL / 文件夹
- `PageUp` — 在文件管理器中定位（仅文件夹）
- 3 秒倒计时后自动消失

## 项目结构

```
quicklauncher/
├── src/                              # Vue 3 前端
│   ├── api/                          # Tauri invoke API 封装
│   ├── assets/                       # 静态资源
│   ├── common/                       # 公共工具函数
│   ├── components/                   # 通用组件
│   ├── composables/                  # 组合式函数（Vue Composables）
│   ├── constant/                     # 常量、枚举、默认数据
│   ├── router/                       # 路由配置（6 个路由）
│   ├── store/                        # Pinia 状态管理
│   ├── styles/                       # 全局样式
│   ├── types/                        # TypeScript 类型声明
│   ├── utils/                        # 工具函数（事件总线、快捷键解析等）
│   └── views/                        # 页面视图
│       ├── Main/                     # 主界面（分类侧边栏 + 启动项列表）
│       ├── Search/                   # 快速搜索窗口
│       ├── Setting/                  # 设置窗口
│       ├── OperationLaunch/          # 添加/编辑启动项弹窗
│       ├── OperationCategory/        # 添加/编辑分类弹窗
│       └── ClipboardToast/           # 剪贴板 Toast 通知
├── src-tauri/                        # Rust 后端
│   ├── src/
│   │   ├── main.rs                   # 入口
│   │   ├── lib.rs                    # Tauri 应用初始化、窗口管理、命令注册
│   │   ├── tray.rs                   # 系统托盘
│   │   ├── clipboard/                # 剪贴板轮询监听
│   │   ├── commands/                 # Tauri 命令（40+，CRUD/搜索/执行等）
│   │   ├── common/utils.rs           # 拼音生成、管理员运行、URL 验证、全屏检测
│   │   ├── db/connection.rs          # SQLite 数据库初始化
│   │   ├── entity/                   # SeaORM 实体（5 张表）
│   │   └── models/                   # DTO 数据模型
│   └── Cargo.toml                    # Rust 依赖
├── package.json                      # 前端依赖与脚本
├── tsconfig.json                     # TypeScript 配置
└── vite.config.ts                    # Vite 构建配置
```

## 数据

### 数据存储位置

所有数据存储在 SQLite 数据库中：

```
%APPDATA%/com.quicklauncher.app/Date.db
```

数据目录（日志文件同在此目录）：

```
%APPDATA%/com.quicklauncher.app/
```

### 数据库表结构

| 表名                   | 用途                 | 关键字段                                  |
| ---------------------- | -------------------- | ----------------------------------------- |
| `launch_items`         | 启动项               | name, path, type, icon, pinyin, keywords  |
| `categories`           | 分类                 | name, parent_id, layout, sort_by          |
| `configs`              | 应用配置（键值 JSON） | name, data                                |
| `launch_history`       | 启动历史（上限 300）  | launch_item_id, command, type, started_at |
| `autocomplete_history` | 搜索补全记录          | query, usage_count, launch_item_id        |

### 数据备份与迁移

- **备份** — 设置 → 数据 → 备份，选择保存位置，生成带时间戳的 `.db` 文件
- **导入** — 设置 → 数据 → 导入，选择之前的备份文件恢复数据
- **重置** — 设置 → 数据 → 重置，删除数据库并重启应用（不可逆）
- **重建** — 通过分类关联目录功能，可以将目录重新扫描为启动项

## 常见问题

### 快捷键不生效？

1. 确认应用已启动（检查系统托盘图标）
2. 在设置 → 常规中检查快捷键是否正确配置
3. 可能与其他应用快捷键冲突，尝试在设置中更换组合键

### 搜索窗口不显示？

1. 确认"快速搜索"功能已开启（设置 → 快速搜索）
2. 检查是否在**全屏应用**中（免打扰模式会自动屏蔽，可关闭）
3. 检查快捷键是否正确，尝试在设置中重新注册

### 搜索不到刚添加的启动项？

1. 中文名称启动项依赖拼音索引，添加后会自动生成拼音，无需手动操作
2. 在设置 → 快速搜索中确认"显示分类"和"显示子分类"选项
3. 检查启动项是否设在被排除搜索的分类下
4. 尝试用关键词或拼音首字母搜索

### 图标没有自动提取？

1. 文件图标依赖 Windows 系统 API 提取，部分文件类型可能无图标
2. `.lnk` 快捷方式会自动解析目标文件图标
3. 网址图标需联网获取，检查网络设置和代理配置
4. 可手动更换图标（右键启动项 → 编辑 → 图标选择器）

### 剪贴板 Toast 不弹出？

1. 确认剪贴板内容是**有效 URL** 或**文件夹绝对路径**
2. 普通文本、文件路径、图片不会触发 Toast
3. 系统剪贴板被其他软件锁定时可能获取失败

### 关联目录不自动更新？

1. 分类关联目录功能依赖文件系统监听，文件变更后 1-2 秒内同步
2. 子目录中的文件不会被自动添加（仅监听一级目录）
3. 大量文件操作时可能延迟，重启应用可强制全量同步

### 如何彻底卸载？

1. Windows 设置 → 应用 → 卸载 Quick Launcher
2. 手动删除数据目录 `%APPDATA%/com.quicklauncher.app/`
3. 如需保留数据，卸载前先备份数据库

## 性能和稳定性

### 资源占用

- **内存** — 空闲约 20-40 MB（含 WebView），使用中约 60-80 MB
- **CPU** — 空闲 < 0.1%，搜索/剪贴板轮询时有微小波动
- **磁盘** — 安装包约 5-10 MB，数据文件通常 < 10 MB

### 性能设计

- **剪贴板轮询** — 每 300ms 检测一次，仅读取文本类型，CPU 开销极低
- **拼音索引** — 启动项创建时预生成拼音全拼和首字母，搜索时直接 SQL 查询，无需实时转换
- **数据库** — SQLite 嵌入式数据库，启动项和分类查询都走索引，十万级数据无压力
- **启动历史** — 自动去重相邻相同记录，上限 300 条，防止数据膨胀
- **跨窗口同步** — 使用 `@tauri-store/pinia` 插件，通过 Tauri IPC 在主进程同步状态，避免重复读写

### 稳定性保障

- **单实例运行** — `plugin-single-instance` 确保只有一个应用实例，重复启动自动聚焦已有窗口
- **Panic 日志** — Rust 端 panic 自动写入 `tauri-error.log`，方便问题排查
- **静默启动** — 支持无窗口静默启动，仅显示托盘图标，不影响用户工作流
- **窗口保护** — 所有窗口关闭时仅隐藏而非销毁，避免状态丢失和重复初始化开销
- **数据库自动迁移** — SeaORM schema sync 在启动时自动同步表结构，升级版本无需手动迁移

## 开发

```bash
pnpm lint         # ESLint 代码检查
pnpm lint:fix     # ESLint 自动修复
pnpm format       # Prettier 格式化
pnpm build        # 类型检查 + 生产构建
```

## 许可证

MIT License

---

**提示**: 本项目专为 Windows 平台设计，依赖部分 Windows 特定 API（图标提取、`.lnk` 解析、全屏检测等），其他平台功能受限。
