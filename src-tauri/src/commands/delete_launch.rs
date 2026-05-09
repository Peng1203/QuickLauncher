use crate::{entity, AppState};
use entity::launch_items::Entity as LaunchItems;
use sea_orm::EntityTrait;

#[tauri::command]
pub async fn delete_launch(id: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    LaunchItems::delete_by_id(id)
        .exec(&db)
        .await
        .map_err(|e| format!("删除启动项失败：{}", e))?;

    Ok(())
}
