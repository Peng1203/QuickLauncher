use entity::categories::{Entity as Categories, Model};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity;

#[tauri::command]
pub async fn get_category_by_id(
    id: i32,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Option<Model>, String> {
    Categories::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
