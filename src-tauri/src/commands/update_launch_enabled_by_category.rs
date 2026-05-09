use crate::{entity, AppState};
use entity::launch_items::{Column, Entity as LaunchItems};
use sea_orm::{prelude::Expr, ColumnTrait, EntityTrait, QueryFilter};

#[tauri::command]
pub async fn update_launch_enabled_by_category(
    category_id: i32,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let query = LaunchItems::update_many().col_expr(Column::Enabled, Expr::value(Some(enabled)));

    query
        .filter(Column::CategoryId.eq(category_id))
        .exec(&db)
        .await
        .map_err(|e| format!("批量更新启用状态失败：{}", e))?;

    Ok(())
}
