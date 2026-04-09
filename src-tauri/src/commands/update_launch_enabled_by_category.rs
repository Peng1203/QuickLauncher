use crate::entity;
use entity::launch_items::{Column, Entity as LaunchItems};
use sea_orm::{prelude::Expr, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[tauri::command]
pub async fn update_launch_enabled_by_category(
    category_id: i32,
    enabled: bool,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let query = LaunchItems::update_many().col_expr(Column::Enabled, Expr::value(Some(enabled)));

    query
        .filter(Column::CategoryId.eq(category_id))
        .exec(db.inner())
        .await
        .map_err(|e| format!("批量更新启用状态失败：{}", e))?;

    Ok(())
}
