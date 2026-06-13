use crate::{entity, reminder, AppState};
use entity::todos::Entity as Todos;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn clear_completed_todos(state: tauri::State<'_, AppState>) -> Result<u64, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 先查询已完成的 todo ID，取消它们的提醒
    let completed_ids: Vec<i32> = Todos::find()
        .filter(entity::todos::Column::Completed.eq(true))
        .all(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "查询已完成待办事项失败");
            format!("查询失败: {}", e)
        })?
        .into_iter()
        .map(|t| t.id)
        .collect();

    // 取消所有已完成 todo 的提醒
    for id in &completed_ids {
        reminder::get_scheduler().cancel(*id).await;
    }

    let result = Todos::delete_many()
        .filter(entity::todos::Column::Completed.eq(true))
        .exec(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "清除已完成待办事项失败");
            format!("清除已完成待办事项失败：{}", e)
        })?;

    tracing::info!(count = result.rows_affected, "清除已完成待办事项");
    Ok(result.rows_affected)
}
