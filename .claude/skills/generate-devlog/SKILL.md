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
- 合并重复描述，忽略无意义 commit（格式化、依赖更新等）
- 不重复记录已有内容
- 按照最新修改的日期方到最上面
- 尽量精简描述 修改/添加了什么内容

---

## Format

根据场景选择：

### 格式 A：日常 / 功能开发

```md
## 2026-06-02（工程化 / Lint 重构）

### ESLint 配置迁移

- 从 `@antfu/eslint-config` 迁移到原生 flat config
- 组合使用：
  - `@eslint/js`
  - `typescript-eslint`
  - `eslint-plugin-vue`

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
```
