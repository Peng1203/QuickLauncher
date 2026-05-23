use crate::{common::utils::run_as_admin, entity, AppState};
use entity::launch_items::{Entity as LaunchItems, Model};
use sea_orm::EntityTrait;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn run_launch_as_admin(id: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 查询
    let launch_item: Model = LaunchItems::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("查询失败：{}", e))?
        .ok_or("启动项不存在")?;

    tracing::info!(id, path = launch_item.path, "以管理员身份执行启动项");

    // 执行
    run_as_admin(launch_item).map_err(|e| {
        tracing::error!(%e, "提权执行失败");
        format!("执行失败：{}", e)
    })?;

    Ok(())
}
