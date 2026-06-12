use crate::dto::todos::UpdateTodoDto;
use crate::{entity, AppState};
use entity::todos::Entity as Todos;
use entity::todos::Model as TodoModel;
use sea_orm::{ActiveModelTrait, EntityTrait};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_todo(
    item: UpdateTodoDto,
    state: tauri::State<'_, AppState>,
) -> Result<TodoModel, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todo = Todos::find_by_id(item.id)
        .one(&db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("待办事项不存在")?;

    let mut active: entity::todos::ActiveModel = todo.into();
    let dto_active: entity::todos::ActiveModel = item.into();

    active.title = dto_active.title;
    active.completed = dto_active.completed;
    active.priority = dto_active.priority;
    active.due_date = dto_active.due_date;
    active.tags = dto_active.tags;
    active.note = dto_active.note;
    active.reminder_at = dto_active.reminder_at;

    let result = active.update(&db).await.map_err(|e| {
        tracing::error!(%e, "更新待办事项失败");
        format!("更新待办事项失败：{}", e)
    })?;

    tracing::info!(id = result.id, "更新待办事项");
    Ok(result)
}
