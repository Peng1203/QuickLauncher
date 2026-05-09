use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entity, AppState};

#[tauri::command]
pub async fn get_launch_by_name_and_category(
    category_id: i32,
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<Option<Model>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    LaunchItems::find()
        .filter(Column::CategoryId.eq(category_id))
        .filter(Column::Name.eq(name))
        .one(&db)
        .await
        .map_err(|e| format!("查询失败: {}", e))
}
