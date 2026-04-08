# Quick Launcher - Windows 快速启动器

![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?style=flat&logo=tauri&logoColor=white)
![Vue.js](https://img.shields.io/badge/Vue.js-3.5-4FC08D?style=flat&logo=vuedotjs&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178C6?style=flat&logo=typescript&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-2021-000000?style=flat&logo=rust&logoColor=white)

一款基于 Tauri + Vue 3 + TypeScript 开发的 Windows 快速启动器，提供高效的应用/文件/网址启动和管理功能。

## ✨ 功能特性

### 🚀 快速启动
- 支持应用程序、文件、文件夹、网址等多种类型启动项
- 通过全局快捷键快速呼出搜索界面
- 智能拼音搜索和自动补全
- 支持管理员权限运行应用

### 📁 分类管理
- 自定义启动项分类
- 分类关联目录快速访问
- 拖拽排序和批量操作
- 右键菜单管理

### 🔍 智能搜索
- 实时搜索过滤
- 拼音首字母搜索
- 网络搜索集成（可配置搜索引擎）
- 翻译功能集成

### ⚙️ 便捷操作
- 系统托盘图标控制
- 开机自启动
- 窗口位置记忆
- 剪贴板管理
- 文件管理器快速打开

### 🎨 现代化界面
- 基于 Naive UI 的简洁美观界面
- 暗色/亮色主题
- 响应式布局
- 图标自动提取和显示

## 🛠️ 技术栈

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全的 JavaScript 超集
- **Naive UI** - Vue 3 组件库
- **Pinia** - 状态管理
- **Vite** - 构建工具
- **Tailwind CSS** - 原子化 CSS 框架

### 后端
- **Tauri 2** - 桌面应用框架
- **Rust** - 系统编程语言
- **SQLite** - 嵌入式数据库
- **Rusqlite** - SQLite 的 Rust 绑定

### 核心插件
- `@tauri-apps/plugin-autostart` - 开机自启动
- `@tauri-apps/plugin-global-shortcut` - 全局快捷键
- `@tauri-apps/plugin-clipboard-manager` - 剪贴板管理
- `@tauri-apps/plugin-single-instance` - 单实例运行
- `@tauri-apps/plugin-fs` - 文件系统操作
- `@tauri-apps/plugin-process` - 进程管理

## 🚀 快速开始

### 环境要求
- **Node.js** >= 18.0.0
- **Rust** >= 1.70.0
- **pnpm** >= 8.0.0 (推荐包管理器)
- **Windows 10/11** (主要支持平台)

### 安装依赖

```bash
# 安装前端依赖
pnpm install

# 安装 Rust 依赖 (Tauri 会自动处理)
```

**注意**: 本项目使用 [pnpm](https://pnpm.io/) 作为包管理器。如果使用 npm 或 yarn，请先安装 pnpm：
```bash
npm install -g pnpm
```

### 开发运行

```bash
# 启动开发服务器
pnpm tauri dev
```

### 构建应用

```bash
# 调试构建
pnpm tauri build --debug

# 发布构建
pnpm tauri build
```

构建后的应用位于 `src-tauri/target/release/` 目录。

## 📖 使用指南

### 基本使用
1. **添加启动项**: 点击添加按钮或拖拽文件到窗口
2. **分类管理**: 左侧分类栏创建和管理分类
3. **快速搜索**: 使用全局快捷键呼出搜索界面（默认 `Ctrl+Shift+P`）
4. **启动应用**: 点击启动项或使用键盘选择后回车

### 快捷键
- `Alt+Space`: 呼出/隐藏搜索窗口（默认，可在设置中修改）
- `Esc`: 关闭搜索窗口
- `↑/↓`: 搜索结果导航
- `Enter`: 执行选中项
- `Ctrl+Enter`: 以管理员权限运行

### 配置说明
应用配置存储在 `~/.quicklauncher/config.json`，包含：
- 窗口位置和大小
- 全局快捷键设置
- 网络搜索引擎配置
- 外观主题设置

## 📁 项目结构

```
quicklauncher/
├── src/                          # 前端源代码
│   ├── components/              # 通用组件
│   ├── views/                   # 页面视图
│   │   ├── Main/               # 主界面
│   │   ├── Search/             # 搜索界面
│   │   └── Setting/            # 设置页面
│   ├── store/                  # 状态管理
│   ├── composables/            # 组合式函数
│   └── utils/                  # 工具函数
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── commands/          # Tauri 命令
│   │   ├── models/            # 数据模型
│   │   ├── db/                # 数据库操作
│   │   └── common/            # 公共工具
│   └── Cargo.toml             # Rust 依赖配置
├── public/                     # 静态资源
└── package.json               # 前端依赖配置
```

## 🔧 开发说明

### 代码规范
- **ESLint**: 代码质量检查
- **Prettier**: 代码格式化
- **TypeScript**: 类型检查

```bash
# 代码检查
pnpm lint

# 代码格式化
pnpm format

# 类型检查
pnpm build
```

### 数据库
使用 SQLite 存储启动项和配置数据，数据库文件位于 `~/.quicklauncher/quicklauncher.db`。

### 图标提取
支持从 EXE、LNK、URL 等文件中自动提取图标，使用 Windows API 获取系统图标。

### 网络功能
- 网页图标获取
- 搜索引擎集成
- 翻译服务调用

## 🤝 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 提供优秀的桌面应用开发框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Naive UI](https://www.naiveui.com/) - 高质量的 Vue 3 组件库
- 所有贡献者和用户的支持

---

**提示**: 本项目主要针对 Windows 平台开发，其他平台功能可能有限。
