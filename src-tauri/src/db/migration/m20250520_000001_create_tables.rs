use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =========================
        // categories
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(pk_auto(Categories::Id))
                    .col(string(Categories::Name).unique_key())
                    .col(integer(Categories::ParentId).null())
                    .col(string(Categories::AssociationDirectory).null())
                    .col(boolean(Categories::Exclude).default(false))
                    .col(string(Categories::Layout).default("grid"))
                    .col(string(Categories::SortBy).default("default"))
                    .col(string(Categories::SortOrder).default("default"))
                    .col(string(Categories::Icon).null())
                    .col(string(Categories::PinyinFull).null())
                    .col(string(Categories::PinyinAbbr).null())
                    .col(integer(Categories::OrderIndex).null().default(0))
                    .col(big_integer(Categories::CreatedAt))
                    .col(big_integer(Categories::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_categories_self")
                            .from(Categories::Table, Categories::ParentId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================
        // launch_items
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(LaunchItems::Table)
                    .if_not_exists()
                    .col(pk_auto(LaunchItems::Id))
                    .col(string(LaunchItems::Name))
                    .col(string(LaunchItems::LnkName).null())
                    .col(string(LaunchItems::Path))
                    .col(string(LaunchItems::Type))
                    .col(string(LaunchItems::Icon).null())
                    .col(string(LaunchItems::Hotkey).null())
                    .col(boolean(LaunchItems::HotkeyGlobal).null())
                    .col(string(LaunchItems::PinyinFull).null())
                    .col(string(LaunchItems::PinyinAbbr).null())
                    .col(string(LaunchItems::Keywords).null())
                    .col(string(LaunchItems::StartDir).null())
                    .col(string(LaunchItems::Remarks).null())
                    .col(string(LaunchItems::Args).null())
                    .col(boolean(LaunchItems::RunAsAdmin).null())
                    .col(integer(LaunchItems::OrderIndex).null())
                    .col(boolean(LaunchItems::Enabled).null())
                    .col(integer(LaunchItems::CategoryId).null())
                    .col(integer(LaunchItems::SubcategoryId).null())
                    .col(string(LaunchItems::Extension).null())
                    .col(integer(LaunchItems::LaunchCount).null())
                    .col(integer(LaunchItems::FailureCount).null())
                    .col(big_integer(LaunchItems::LastUsedAt).null())
                    .col(big_integer(LaunchItems::CreatedAt))
                    .col(big_integer(LaunchItems::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_launch_items_category")
                            .from(LaunchItems::Table, LaunchItems::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_launch_items_subcategory")
                            .from(LaunchItems::Table, LaunchItems::SubcategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================
        // launch_history
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(LaunchHistory::Table)
                    .if_not_exists()
                    .col(pk_auto(LaunchHistory::Id))
                    .col(integer(LaunchHistory::LaunchItemId).null())
                    .col(string(LaunchHistory::Command))
                    .col(string(LaunchHistory::Type))
                    .col(big_integer(LaunchHistory::StartedAt).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_launch_history_launch_item")
                            .from(LaunchHistory::Table, LaunchHistory::LaunchItemId)
                            .to(LaunchItems::Table, LaunchItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================
        // autocomplete_history
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(AutocompleteHistory::Table)
                    .if_not_exists()
                    .col(pk_auto(AutocompleteHistory::Id))
                    .col(string(AutocompleteHistory::Query).unique_key())
                    .col(integer(AutocompleteHistory::UsageCount).null())
                    .col(big_integer(AutocompleteHistory::LastUsedAt))
                    .col(integer(AutocompleteHistory::LaunchItemId).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_autocomplete_history_launch_item")
                            .from(
                                AutocompleteHistory::Table,
                                AutocompleteHistory::LaunchItemId,
                            )
                            .to(LaunchItems::Table, LaunchItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================
        // configs
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(Configs::Table)
                    .if_not_exists()
                    .col(pk_auto(Configs::Id))
                    .col(string(Configs::Name).unique_key())
                    .col(string(Configs::Data))
                    .to_owned(),
            )
            .await?;

        // =========================
        // todos
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(pk_auto(Todos::Id))
                    .col(string(Todos::Title))
                    .col(boolean(Todos::Completed).default(false))
                    .col(integer(Todos::Priority).default(1))
                    .col(big_integer(Todos::DueDate).null())
                    .col(string(Todos::Tags).null())
                    .col(string(Todos::Note).null())
                    .col(big_integer(Todos::ReminderAt).null())
                    .col(big_integer(Todos::CreatedAt))
                    .col(big_integer(Todos::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Configs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AutocompleteHistory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LaunchHistory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LaunchItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;

        Ok(())
    }
}

// =========================
// Iden
// =========================

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    Name,
    ParentId,
    AssociationDirectory,
    Exclude,
    Layout,
    SortBy,
    SortOrder,
    Icon,
    PinyinFull,
    PinyinAbbr,
    OrderIndex,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum LaunchItems {
    Table,
    Id,
    Name,
    LnkName,
    Path,
    Type,
    Icon,
    Hotkey,
    HotkeyGlobal,
    PinyinFull,
    PinyinAbbr,
    Keywords,
    StartDir,
    Remarks,
    Args,
    RunAsAdmin,
    OrderIndex,
    Enabled,
    CategoryId,
    SubcategoryId,
    Extension,
    LaunchCount,
    FailureCount,
    LastUsedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum LaunchHistory {
    Table,
    Id,
    LaunchItemId,
    Command,
    Type,
    StartedAt,
}

#[derive(Iden)]
enum AutocompleteHistory {
    Table,
    Id,
    Query,
    UsageCount,
    LastUsedAt,
    LaunchItemId,
}

#[derive(Iden)]
enum Configs {
    Table,
    Id,
    Name,
    Data,
}

#[derive(Iden)]
enum Todos {
    Table,
    Id,
    Title,
    Completed,
    Priority,
    DueDate,
    Tags,
    Note,
    ReminderAt,
    CreatedAt,
    UpdatedAt,
}
