use entity::categories::{Column, Entity as Categories};
use sea_orm::{EntityTrait, QueryOrder};
use tracing;

use crate::{dto::categories::CategoryDto, entity, AppState};
// db: tauri::State<'_, DatabaseConnection>
#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn get_category(state: tauri::State<'_, AppState>) -> Result<Vec<CategoryDto>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;
    let categories = Categories::find()
        .order_by_desc(Column::OrderIndex)
        .all(&db)
        .await
        .map_err(|e| format!("查询分类失败：{}", e))?;
    Ok(categories.into_iter().map(Into::into).collect())
}
