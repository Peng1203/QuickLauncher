use entity::launch_history;
use entity::launch_history::Entity as LaunchHistory;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::entity;

const MAX_HISTORY_COUNT: u64 = 300;

#[tauri::command]
pub async fn add_launch_history(
    command: String,
    r#type: String,
    launch_item_id: Option<i32>,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let command = command.trim();
    // 空命令不记录
    if command.is_empty() {
        return Ok(());
    }

    // 查询最后一条历史
    let last_history = LaunchHistory::find()
        .order_by_desc(launch_history::Column::Id)
        .one(&*db)
        .await
        .map_err(|e| e.to_string())?;

    // 连续相同命令不重复插入
    if let Some(last) = last_history {
        let is_same_command = last.command == command;
        let is_same_type = last.r#type == r#type;
        let is_same_launch_item_id = last.launch_item_id == launch_item_id;

        if is_same_command && is_same_type && is_same_launch_item_id {
            return Ok(());
        }
    }

    // 插入历史
    let history = launch_history::ActiveModel {
        command: Set(command.to_string()),
        r#type: Set(r#type),
        launch_item_id: Set(launch_item_id),
        ..Default::default()
    };

    history.insert(&*db).await.map_err(|e| e.to_string())?;

    // 限制历史数量
    let total = LaunchHistory::find()
        .count(&*db)
        .await
        .map_err(|e| e.to_string())?;

    if total > MAX_HISTORY_COUNT {
        let remove_count = total - MAX_HISTORY_COUNT;

        let old_ids: Vec<i32> = LaunchHistory::find()
            .order_by_asc(launch_history::Column::Id)
            .limit(remove_count)
            .all(&*db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|item| item.id)
            .collect();

        if !old_ids.is_empty() {
            LaunchHistory::delete_many()
                .filter(launch_history::Column::Id.is_in(old_ids))
                .exec(&*db)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
