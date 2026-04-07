use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity;

#[tauri::command]
pub async fn get_launch_by_name_and_category(
    category_id: i32,
    name: String,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Option<Model>, String> {
    LaunchItems::find()
        .filter(Column::CategoryId.eq(category_id))
        .filter(Column::Name.eq(name))
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败: {}", e))
}
