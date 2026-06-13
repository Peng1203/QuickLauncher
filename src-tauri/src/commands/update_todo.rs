use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::dto::todos::UpdateTodoDto;
use crate::entity::todos;
use crate::{reminder, AppState};
use tauri::AppHandle;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_todo(
    item: UpdateTodoDto,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todo_id = item.id;

    let model = todos::Entity::find_by_id(todo_id)
        .one(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "查询待办事项失败");
            format!("查询失败: {}", e)
        })?;

    let model = match model {
        Some(m) => m,
        None => return Err("待办事项不存在".to_string()),
    };

    // 记录旧的 reminder_at 用于比较
    let old_reminder_at = model.reminder_at;

    tracing::info!(id = todo_id, "更新待办事项");

    item.into_active_model(model)
        .update(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "更新待办事项失败");
            format!("更新失败: {}", e)
        })?;

    // 查询更新后的完整记录
    let updated_model = todos::Entity::find_by_id(todo_id)
        .one(&db)
        .await
        .ok()
        .flatten();

    if let Some(updated) = updated_model {
        let new_reminder_at = updated.reminder_at;
        let new_completed = updated.completed;
        let now = chrono::Utc::now().timestamp_millis();

        if new_completed {
            // 已完成：取消提醒
            reminder::get_scheduler().cancel(todo_id).await;
        } else if new_reminder_at != old_reminder_at {
            // reminder_at 发生变化
            if let Some(reminder_at) = new_reminder_at {
                if reminder_at > now {
                    // 重新调度
                    reminder::get_scheduler()
                        .schedule(app, db, todo_id, reminder_at)
                        .await;
                } else {
                    // 已过期，取消
                    reminder::get_scheduler().cancel(todo_id).await;
                }
            } else {
                // reminder_at 被清空，取消提醒
                reminder::get_scheduler().cancel(todo_id).await;
            }
        }
    }

    Ok(())
}
