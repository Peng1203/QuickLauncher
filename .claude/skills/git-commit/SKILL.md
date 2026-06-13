---
name: git-commit
description: 智能 Git 提交。当用户想要提交代码、记录变更时使用。自动分析已修改文件，将相关变更归组，生成规范的 commit message 并执行多次提交。
---

# Git Commit

## Goal

分析当前工作区的所有变更，将相关文件智能归组，依次执行多个语义明确的 commit。

---

## Workflow

### Step 0 - 询问生成开发日志 (!!!重要!!!)

可以先询问用户是否需要先执行 生成开发日志的skill `generate-devlog` 当用户选择了 是则先执行 `generate-devlog` 再进行后续的操作

### Step 1 - 获取变更文件

```bash
git status --short          # 查看所有变更文件
git diff --stat             # 查看具体改动量
```

### Step 2 - 分析并归组

按以下维度将文件归入同一 commit：

- **功能相关**：同一功能的实现、组件、样式、测试
- **修复相关**：同一 bug 的修复文件
- **配置相关**：同类配置文件变更
- **重构相关**：同一模块的重构文件

**归组原则：**

- 一个 commit 只做一件事
- 同一功能的前后端文件归为一组
- 不相关的文件拆分到不同 commit
- 新增文件与其依赖的修改文件归为一组

### Step 3 - 展示归组方案，等待确认

输出归组计划，格式如下：
准备执行 N 个 commit：
[1/N] feat: 实现剪贴板监控功能

src/clipboard/listener.rs
src/components/ClipboardToast.tsx
M src/main.rs

[2/N] feat: 新增快捷键自定义设置

src/components/ShortcutKeyInput.tsx
M src/store/settings.ts

[3/N] style: 设置界面布局调整
M src/pages/Settings.tsx
M src/styles/settings.css
是否确认执行？(yes/修改方案)

### Step 4 - 执行提交

用户确认后，依次执行：

```bash
git add <files>
git commit -m "<message>"
```

每次提交后输出结果，全部完成后汇总。

---

## Commit Message 规范

遵循 Conventional Commits 格式：
<type>(<scope>): <subject>
**type 对照：**

| type       | 场景               |
| ---------- | ------------------ |
| `feat`     | 新功能             |
| `fix`      | Bug 修复           |
| `refactor` | 重构（不影响功能） |
| `style`    | UI / 样式调整      |
| `chore`    | 配置、依赖、工程化 |
| `docs`     | 文档变更           |
| `perf`     | 性能优化           |
| `test`     | 测试相关           |

**规则：**

- subject 使用中文，简洁描述做了什么
- scope 可选，填模块名（如 `clipboard`、`settings`）
- 不超过 72 字符
- 不加句号

**示例：**
feat(clipboard): 实现剪贴板监控与变化提示
fix(window): 修复 Windows 11 下窗口位置偏移
refactor(components): 迁移基础组件至 components/ui
chore: 更新依赖版本

---

## Output Rules

- 执行前必须展示归组方案并等待确认
- 每个 commit 执行后输出简要结果
- 若用户对归组有异议，重新调整后再执行
- 遇到冲突或错误立即停止并提示

---

## Example

**变更文件：**
M src/clipboard/listener.rs
M src/components/ClipboardToast.tsx
M src/pages/Settings.tsx
M src/store/settings.ts
M src/components/ShortcutKeyInput.tsx
M src/styles/settings.css
M README.md

**归组输出：**
准备执行 3 个 commit：
[1/3] feat(clipboard): 实现剪贴板监控与变化提示
M src/clipboard/listener.rs
M src/components/ClipboardToast.tsx
[2/3] feat(settings): 新增快捷键自定义设置
M src/components/ShortcutKeyInput.tsx
M src/store/settings.ts
[3/3] style(settings): 设置界面布局与样式调整
M src/pages/Settings.tsx
M src/styles/settings.css
（README.md 已归入 [1/3]，因包含对应功能说明）
是否确认执行？(yes/修改方案)

**执行输出：**

✓ [1/3] feat(clipboard): 实现剪贴板监控与变化提示
✓ [2/3] feat(settings): 新增快捷键自定义设置
✓ [3/3] style(settings): 设置界面布局与样式调整
完成，共 3 个 commit。
