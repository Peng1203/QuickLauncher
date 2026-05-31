use crate::{entity, AppState};
use entity::todos::Entity as Todos;
use entity::todos::Model as TodoModel;
use sea_orm::{EntityTrait, QueryOrder};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn get_todos(state: tauri::State<'_, AppState>) -> Result<Vec<TodoModel>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todos = Todos::find()
        .order_by_desc(entity::todos::Column::Id)
        .all(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "查询待办事项失败");
            format!("查询待办事项失败：{}", e)
        })?;

    Ok(todos)
}
