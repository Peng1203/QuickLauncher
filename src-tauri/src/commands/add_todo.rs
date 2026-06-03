use crate::{entity, models::todo_item::NewTodoItem, AppState};
use entity::todos::Model as TodoModel;
use sea_orm::{ActiveModelTrait, Set};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn add_todo(
    item: NewTodoItem,
    state: tauri::State<'_, AppState>,
) -> Result<TodoModel, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todo = entity::todos::ActiveModel {
        title: Set(item.title),
        completed: Set(false),
        priority: Set(item.priority),
        due_date: Set(item.due_date),
        tags: Set(item.tags),
        note: Set(item.note),
        reminder_at: Set(item.reminder_at),
        ..Default::default()
    };

    let result = todo.insert(&db).await.map_err(|e| {
        tracing::error!(%e, "添加待办事项失败");
        format!("添加待办事项失败：{}", e)
    })?;

    tracing::info!(id = result.id, "添加待办事项");
    Ok(result)
}
