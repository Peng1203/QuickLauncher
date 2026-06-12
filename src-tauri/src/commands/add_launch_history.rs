use entity::launch_history;
use entity::launch_history::Entity as LaunchHistory;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use tracing;

use crate::{dto::launch_history::CreateLaunchHistoryDto, entity, AppState};

const MAX_HISTORY_COUNT: u64 = 300;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn add_launch_history(
    command: String,
    r#type: String,
    launch_item_id: Option<i32>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap().clone().ok_or("数据库未连接")?;

    let command = command.trim();
    if command.is_empty() {
        return Ok(());
    }

    // 查询最后一条历史
    let last = LaunchHistory::find()
        .order_by_desc(launch_history::Column::Id)
        .one(&db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(last) = last {
        if last.command == command && last.r#type == r#type && last.launch_item_id == launch_item_id
        {
            return Ok(());
        }
    }

    let dto = CreateLaunchHistoryDto {
        launch_item_id,
        command: command.to_string(),
        r#type,
        started_at: Some(chrono::Utc::now().timestamp_millis()),
    };

    let history: launch_history::ActiveModel = dto.into();

    history.insert(&db).await.map_err(|e| e.to_string())?;

    // 控制数量
    let total = LaunchHistory::find()
        .count(&db)
        .await
        .map_err(|e| e.to_string())?;

    if total > MAX_HISTORY_COUNT {
        let remove_count = total - MAX_HISTORY_COUNT;

        let old_ids: Vec<i32> = LaunchHistory::find()
            .order_by_asc(launch_history::Column::Id)
            .limit(remove_count)
            .all(&db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|item| item.id)
            .collect();

        if !old_ids.is_empty() {
            LaunchHistory::delete_many()
                .filter(launch_history::Column::Id.is_in(old_ids))
                .exec(&db)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
