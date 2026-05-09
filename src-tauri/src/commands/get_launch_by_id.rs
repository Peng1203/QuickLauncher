use entity::launch_items::{Entity as LaunchItems, Model};
use sea_orm::EntityTrait;

use crate::{entity, AppState};

#[tauri::command]
pub async fn get_launch_by_id(
    id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<Option<Model>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    LaunchItems::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
