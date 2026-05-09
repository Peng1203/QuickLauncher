use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::entity::{self, launch_history, launch_items};
use crate::AppState;
use entity::launch_history::Entity as LaunchHistory;
use entity::launch_items::Entity as LaunchItem;

use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct LaunchHistoryWithIcon {
    pub id: i32,
    pub command: String,
    pub r#type: String,
    pub launch_item_id: Option<i32>,
    pub icon: Option<String>,
}

#[tauri::command]
pub async fn get_recent_launch_history(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<LaunchHistoryWithIcon>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 1. 查询最近 20 条历史记录
    let histories = LaunchHistory::find()
        .order_by_desc(launch_history::Column::Id)
        .limit(20)
        .all(&db)
        .await
        .map_err(|e| e.to_string())?;

    // 2. 提取所有非空 launch_item_id
    let item_ids: Vec<i32> = histories.iter().filter_map(|h| h.launch_item_id).collect();

    // 3. 防止 IN () 空查询报错
    let items = if item_ids.is_empty() {
        vec![]
    } else {
        LaunchItem::find()
            .filter(launch_items::Column::Id.is_in(item_ids))
            .all(&db)
            .await
            .map_err(|e| e.to_string())?
    };

    // 4. 构建 id -> icon map（压平 Option）
    let icon_map: HashMap<i32, String> = items
        .into_iter()
        .filter_map(|i| i.icon.map(|icon| (i.id, icon)))
        .collect();

    // 5. 合并结果（安全处理 Option）
    let result: Vec<LaunchHistoryWithIcon> = histories
        .into_iter()
        .map(|h| {
            let icon = h.launch_item_id.and_then(|id| icon_map.get(&id).cloned());

            LaunchHistoryWithIcon {
                id: h.id,
                command: h.command,
                r#type: h.r#type,
                launch_item_id: h.launch_item_id,
                icon,
            }
        })
        .collect();

    Ok(result)
}
