use crate::models::todo_item::TodoItem;
use crate::{entity, AppState};
use entity::todos::Entity as Todos;
use entity::todos::Model as TodoModel;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_todo(
    item: TodoItem,
    state: tauri::State<'_, AppState>,
) -> Result<TodoModel, String> {
    dbg!(&item);

    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todo = Todos::find_by_id(item.id)
        .one(&db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("待办事项不存在")?;

    let mut active: entity::todos::ActiveModel = todo.into();
    active.title = Set(item.title);
    active.completed = Set(item.completed);
    active.priority = Set(item.priority);
    active.due_date = Set(item.due_date);
    active.tags = Set(item.tags);
    active.note = Set(item.note);
    active.reminder_at = Set(item.reminder_at);

    let result = active.update(&db).await.map_err(|e| {
        tracing::error!(%e, "更新待办事项失败");
        format!("更新待办事项失败：{}", e)
    })?;

    tracing::info!(id = result.id, "更新待办事项");
    Ok(result)
}
