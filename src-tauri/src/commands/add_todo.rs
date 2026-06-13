use crate::{dto::todos::CreateTodoDto, entity, reminder, AppState};
use entity::todos::Model as TodoModel;
use sea_orm::ActiveModelTrait;
use tauri::AppHandle;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn add_todo(
    item: CreateTodoDto,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<TodoModel, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todo: entity::todos::ActiveModel = item.into();

    let result = todo.insert(&db).await.map_err(|e| {
        tracing::error!(%e, "添加待办事项失败");
        format!("添加待办事项失败：{}", e)
    })?;

    // 如果设置了提醒时间，调度提醒
    if let Some(reminder_at) = result.reminder_at {
        let now = chrono::Utc::now().timestamp_millis();
        if reminder_at > now {
            reminder::get_scheduler()
                .schedule(app, db, result.id, reminder_at)
                .await;
        }
    }

    tracing::info!(id = result.id, "添加待办事项");
    Ok(result)
}
