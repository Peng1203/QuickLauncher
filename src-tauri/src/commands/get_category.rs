use entity::categories::{Column, Entity as Categories, Model};
use sea_orm::{EntityTrait, QueryOrder};

use crate::{entity, AppState};
// db: tauri::State<'_, DatabaseConnection>
#[tauri::command]
pub async fn get_category(state: tauri::State<'_, AppState>) -> Result<Vec<Model>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;
    Categories::find()
        .order_by_desc(Column::OrderIndex)
        .all(&db)
        .await
        .map_err(|e| format!("查询分类失败：{}", e))
}
