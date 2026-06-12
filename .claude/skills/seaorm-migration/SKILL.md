---
name: seaorm-migration
description: 根据 SeaORM entity 文件的变更生成对应的迁移文件。当用户修改了 entity 文件需要生成迁移时触发，或用户说"新增字段"、"add column"、"修改 entity"、"生成迁移"、"迁移文件"、"更新 schema"等。自动创建 migration .rs 文件并注册到 mod.rs 中。
---

# SeaORM 迁移文件生成器

## 固定路径

- entity 目录：`src-tauri/src/entity/`
- 迁移文件目录：`src-tauri/src/db/migration/`
- 迁移注册文件：`src-tauri/src/db/migration/mod.rs`

## 执行步骤

### 1. 读取变更内容

读取用户修改的 entity 文件，以及 `mod.rs` 了解现有迁移列表 可使用 git diff 查看变更内容。

### 2. 生成时间戳文件名

```bash
date -u +"%Y%m%d_%H%M%S"
```

文件名格式：`m{YYYYMMDD}_{HHMMSS}_{描述}.rs`，描述用英文小写下划线。

### 3. 写入迁移文件

**新增字段模板：**

```rust
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m{YYYYMMDD}_{HHMMSS}_{描述}"  // 与文件名一致，不含 .rs
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table({表名}::Table)
                    .add_column(
                        ColumnDef::new({表名}::{字段名})
                            .{类型}()
                            .not_null()
                            .default({默认值}),  // not_null 时必须有默认值
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(()) // SQLite 不支持 DROP COLUMN
    }
}

#[derive(Iden)]
enum {表名} {
    Table,
    {字段名},
}
```

**Rust 类型 → ColumnDef 对照：**

| Entity 类型                       | ColumnDef 方法            |
| --------------------------------- | ------------------------- |
| `i32`                             | `.integer()`              |
| `i64`                             | `.big_integer()`          |
| `String` / `column_type = "Text"` | `.text()`                 |
| `bool`                            | `.boolean()`              |
| `f64`                             | `.double()`               |
| `Option<T>`                       | `.null()`，无需 default   |
| 非 Option                         | `.not_null().default(值)` |

### 4. 注册到 mod.rs

在 `mod.rs` 末尾的 mod 列表追加：

```rust
mod m{YYYYMMDD}_{HHMMSS}_{描述};
```

在 `vec!` 末尾追加：

```rust
Box::new(m{YYYYMMDD}_{HHMMSS}_{描述}::Migration),
```

**严禁调整已有条目顺序。**

### 5. 输出 git commit 消息

根据本次变更内容，生成一条 commit 消息，格式：

```
feat(db): 简短描述本次 schema 变更
```
