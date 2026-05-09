use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{entity, AppState};

#[tauri::command]
pub async fn get_alias_launch(state: tauri::State<'_, AppState>) -> Result<Vec<Model>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    LaunchItems::find()
        .filter(Column::Type.eq("alias"))
        .all(&db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
