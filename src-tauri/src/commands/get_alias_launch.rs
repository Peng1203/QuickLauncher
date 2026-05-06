use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tauri::State;

use crate::entity;

#[tauri::command]
pub async fn get_alias_launch(db: State<'_, DatabaseConnection>) -> Result<Vec<Model>, String> {
    LaunchItems::find()
        .filter(Column::Type.eq("alias"))
        .all(&*db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
