use entity::categories::{Column, Entity as Categories, Model};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

use crate::entity;

#[tauri::command]
pub async fn get_category(db: tauri::State<'_, DatabaseConnection>) -> Result<Vec<Model>, String> {
    Categories::find()
        .order_by_desc(Column::OrderIndex)
        .all(&*db)
        .await
        .map_err(|e| format!("查询分类失败：{}", e))
}
