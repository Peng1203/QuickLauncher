use crate::{dto::categories::CategoryDto, entity, AppState};

use entity::categories::Entity as Categories;
use sea_orm::EntityTrait;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn get_category_by_id(
    id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<Option<CategoryDto>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let category = Categories::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("查询失败：{}", e))?;

    Ok(category.map(Into::into))
}
