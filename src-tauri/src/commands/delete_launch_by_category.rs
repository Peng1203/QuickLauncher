use crate::{entity, AppState};
use entity::launch_items::{Column, Entity as LaunchItems};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[tauri::command]
pub async fn delete_launch_by_category(
    category_id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let mut query = LaunchItems::delete_many();

    query = if category_id == -1 {
        // category_id IS NULL
        query.filter(Column::CategoryId.is_null())
    } else {
        // category_id = ?
        query.filter(Column::CategoryId.eq(category_id))
    };

    query
        .exec(&db)
        .await
        .map_err(|e| format!("批量删除启动项失败：{}", e))?;

    Ok(())
}
