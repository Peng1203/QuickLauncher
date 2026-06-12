use crate::{entity, AppState};
use entity::categories::{Column, Entity as Categories};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn delete_category(id: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let result = Categories::delete_many()
        .filter(Column::Id.eq(id))
        .exec(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "删除分类失败");
            format!("删除分类失败: {}", e)
        })?;

    if result.rows_affected == 0 {
        tracing::error!(id, "分类不存在");
        return Err(format!("分类 ID {} 不存在", id));
    }

    tracing::info!(id, "删除分类");

    Ok(())
}
