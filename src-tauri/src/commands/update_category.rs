use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::dto::categories::UpdateCategoryDto;
use crate::entity::categories;
use crate::AppState;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_category(
    item: UpdateCategoryDto,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 先查是否存在
    categories::Entity::find_by_id(item.id)
        .one(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "查询分类失败");
            format!("查询失败: {}", e)
        })?;

    item.into_active_model()
        .update(&db)
        .await
        .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_category_ass_dir(
    id: i32,
    ass_dir: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 先查
    let model = categories::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "查询分类失败");
            format!("查询失败: {}", e)
        })?;

    // 判空
    let mut model: categories::ActiveModel = match model {
        Some(m) => m.into(),
        None => return Err("No item found with the specified ID".to_string()),
    };

    // 更新字段
    model.association_directory = Set(Some(ass_dir));

    // 执行更新
    model.update(&db).await.map_err(|e| {
        tracing::error!(%e, "更新分类失败");
        format!("更新失败: {}", e)
    })?;

    tracing::info!(id, "更新分类");

    Ok(())
}
