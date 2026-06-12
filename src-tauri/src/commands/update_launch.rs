use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{
    common::utils::get_pinyin_variants, dto::launch_items::UpdateLaunchItemDto, entity::launch_items,
    AppState,
};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_launch(
    item: UpdateLaunchItemDto,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let model = launch_items::Entity::find_by_id(item.id)
        .one(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "查询启动项失败");
            format!("查询失败: {}", e)
        })?;

    let model = match model {
        Some(m) => m,
        None => return Err("No launch item found with the specified ID".to_string()),
    };

    let name = item.name.clone().unwrap_or_default();
    let (pinyin_full, pinyin_abbr) = get_pinyin_variants(&name);

    tracing::info!(name, "更新启动项");

    let active = item.into_active_model(model, pinyin_full, pinyin_abbr);

    active.update(&db).await.map_err(|e| {
        tracing::error!(%e, "更新启动项失败");
        format!("更新失败: {}", e)
    })?;

    Ok(())
}
