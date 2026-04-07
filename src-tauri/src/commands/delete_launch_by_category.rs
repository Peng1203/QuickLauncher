use crate::entity;
use entity::launch_items::{Column, Entity as LaunchItems};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[tauri::command]
pub async fn delete_launch_by_category(
    category_id: i32,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let mut query = LaunchItems::delete_many();

    query = if category_id == -1 {
        // category_id IS NULL
        query.filter(Column::CategoryId.is_null())
    } else {
        // category_id = ?
        query.filter(Column::CategoryId.eq(category_id))
    };

    query
        .exec(db.inner())
        .await
        .map_err(|e| format!("批量删除启动项失败：{}", e))?;

    Ok(())
}
