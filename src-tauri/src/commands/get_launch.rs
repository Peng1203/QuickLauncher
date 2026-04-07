use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity;

#[tauri::command]
pub async fn get_launch(
    category_id: i32,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<Model>, String> {
    let mut query = LaunchItems::find();

    if category_id == -1 {
        // 对应：category_id IS NULL
        query = query.filter(Column::CategoryId.is_null());
    } else {
        // 对应：category_id = ?
        query = query.filter(Column::CategoryId.eq(category_id));
    }

    query
        .all(&*db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
