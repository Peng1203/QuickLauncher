---
name: generate-devlog
description: 生成开发日志。当用户想要记录开发修改过程、总结开发成果、或在任意时间点记录开发状态时使用。
---

# Generate Devlog

## Workflow

1. 检查 `.devlog/DEVLOG.md` 是否存在，了解已记录内容
2. 根据用户意图确定 commit 范围：

```bash
   git log --oneline -10 --no-merges          # 默认：最近 10 条
   git log <start>..<end> --oneline --no-merges  # 指定范围
   git log --after="YYYY-MM-DD" --oneline --no-merges  # 指定日期
```

3. 分析 commit + 文件变化，按逻辑归类，输出结构化日志
4. 追加到 `.devlog/DEVLOG.md`，**绝不覆盖已有内容**

---

## Output Rules

- 使用中文 + Markdown
- 按逻辑分类，不直接复制 commit 列表
- 解释"为什么"，不只写"做了什么"
- 合并重复描述，忽略无意义 commit（格式化、依赖更新等）
- 不重复记录已有内容

---

## Format

根据场景选择：

### 格式 A：日常 / 功能开发

```md
## YYYY-MM-DD HH:MM
Type: 日常开发 / 功能开发 / 重构 / 修复

### 变更概述
（1-2 句话）

### 具体变更
- ...

### 技术决策
（重要决策和原因，没有则省略）

### 后续工作
- [ ] ...（没有则省略）
```

### 格式 B：版本发布

```md
## vX.Y.Z
Date: YYYY-MM-DD
Commits: N

### 新功能
### UI / UX
### 架构调整
### Bug 修复
### 工程化
### Notes
（按实际内容保留分类，无内容的分类省略）
```

### 格式 C：里程碑

```md
## 里程碑：[名称]
Date: YYYY-MM-DD

### 完成内容
### 技术亮点
### 经验总结
### 后续规划
```

---

## 其他补充
禁止自动执行 git commit 相关操作
可参考 git-commit skill 中的规则进行git相关操作

## Example Output

### 日常开发记录

```md
## 2026-05-29 15:30
Type: 功能开发

### 变更概述
实现剪贴板监控功能，支持自动检测变化并弹窗提示。

### 具体变更
- 新增 `clipboard/listener.rs`：轮询监控剪贴板（300ms 间隔）
- 新增 `ClipboardToast` 组件：变化通知弹窗
- 引入 `arboard` crate 实现跨平台剪贴板访问

### 技术决策
- 轮询而非系统事件：Windows 剪贴板事件 API 不稳定
- 300ms 间隔：平衡响应速度与 CPU 占用

### 后续工作
- [ ] 支持配置监控间隔
- [ ] 支持剪贴板历史记录

```