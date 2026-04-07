use entity::launch_items::{Entity as LaunchItems, Model};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity;

#[tauri::command]
pub async fn get_launch_by_id(
    id: i32,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Option<Model>, String> {
    LaunchItems::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
