use crate::{
    dto::categories::{CategoryDto, CreateCategoryDto},
    entity, AppState,
};
use entity::categories::{ActiveModel, Model};
use sea_orm::ActiveModelTrait;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn add_category(
    item: CreateCategoryDto,
    state: tauri::State<'_, AppState>,
) -> Result<CategoryDto, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    tracing::info!(name = &item.name, "添加分类");

    let model: ActiveModel = item.into_active_model();
    let category: Model = model.insert(&db).await.map_err(|e| {
        tracing::error!(%e, "插入分类失败");
        format!("插入分类失败: {}", e)
    })?;

    Ok(category.into())
}
