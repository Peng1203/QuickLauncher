use crate::{dto::launch_items::CreateLaunchItemDto, entity::launch_items::ActiveModel, AppState};
use sea_orm::ActiveModelTrait;

use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn add_launch(
    item: CreateLaunchItemDto,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    dbg!(&item);
    let name = item.name.clone();
    let r#type = item.r#type.clone();

    let model: ActiveModel = item.into_active_model();

    model.insert(&db).await.map_err(|e| {
        tracing::error!(%e, "插入启动项失败");
        format!("插入启动项失败: {}", e)
    })?;

    tracing::info!(name, r#type, "添加启动项");

    Ok(())
}
