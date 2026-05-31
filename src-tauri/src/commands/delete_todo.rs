use crate::{entity, AppState};
use entity::todos::Entity as Todos;
use sea_orm::EntityTrait;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn delete_todo(id: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    Todos::delete_by_id(id).exec(&db).await.map_err(|e| {
        tracing::error!(%e, "删除待办事项失败");
        format!("删除待办事项失败：{}", e)
    })?;

    tracing::info!(id, "删除待办事项");
    Ok(())
}
