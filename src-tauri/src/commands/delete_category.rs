use crate::entity;
use entity::categories::{Column, Entity as Categories};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[tauri::command]
pub async fn delete_category(
    id: i64,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let result = Categories::delete_many()
        .filter(Column::Id.eq(id))
        .exec(db.inner())
        .await
        .map_err(|e| format!("删除分类失败: {}", e))?;

    if result.rows_affected == 0 {
        return Err(format!("分类 ID {} 不存在", id));
    }

    Ok(())
}
