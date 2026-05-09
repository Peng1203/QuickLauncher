use entity::categories::{Entity as Categories, Model};
use sea_orm::EntityTrait;

use crate::{entity, AppState};

#[tauri::command]
pub async fn get_category_by_id(
    id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<Option<Model>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    Categories::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
