use crate::entity;
use entity::launch_items::Entity as LaunchItems;
use sea_orm::{DatabaseConnection, EntityTrait};

#[tauri::command]
pub async fn delete_launch(
    id: i32,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    LaunchItems::delete_by_id(id)
        .exec(db.inner())
        .await
        .map_err(|e| format!("删除启动项失败：{}", e))?;

    Ok(())
}
