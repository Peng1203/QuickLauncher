use entity::categories::{Entity as Categories, Model};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity;

#[tauri::command]
pub async fn get_category(db: tauri::State<'_, DatabaseConnection>) -> Result<Vec<Model>, String> {
    Categories::find()
        .all(&*db)
        .await
        .map_err(|e| format!("查询分类失败：{}", e))
}
