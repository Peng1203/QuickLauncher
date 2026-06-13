use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::entity::todos;
use crate::AppState;
use tracing;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoOrderItem {
    pub id: i32,
    pub order_index: i32,
}

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn batch_update_todo_order(
    items: Vec<TodoOrderItem>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    for item in &items {
        let model = todos::Entity::find_by_id(item.id)
            .one(&db)
            .await
            .map_err(|e| {
                tracing::error!(%e, "查询待办事项失败");
                format!("查询失败: {}", e)
            })?;

        let model = match model {
            Some(m) => m,
            None => continue,
        };

        let mut active: todos::ActiveModel = model.into();
        active.order_index = Set(Some(item.order_index));

        active.update(&db).await.map_err(|e| {
            tracing::error!(%e, "更新待办事项排序失败");
            format!("更新失败: {}", e)
        })?;
    }

    tracing::info!(count = items.len(), "批量更新待办事项排序");
    Ok(())
}
