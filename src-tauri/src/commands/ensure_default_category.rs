use crate::common::utils::get_pinyin_variants;
use crate::{dto::categories::CreateCategoryDto, entity, AppState};
use entity::categories::{ActiveModel, Entity};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn ensure_default_category(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 查询是否存在任意分类数据
    let count = Entity::find()
        .count(&db)
        .await
        .map_err(|e| format!("查询分类数量失败: {}", e))?;

    // 如果已有分类，则跳过
    if count > 0 {
        return Ok(());
    }

    const DEFAULT_CATEGORY_NAME: &str = "默认";

    let pinyin_value = get_pinyin_variants(DEFAULT_CATEGORY_NAME);
    let pinyin_full = pinyin_value.0;
    let pinyin_abbr = pinyin_value.1;

    // 创建默认分类
    let model: ActiveModel = CreateCategoryDto {
        name: DEFAULT_CATEGORY_NAME.to_string(),
        order_index: Some(9999),
        pinyin_full: Some(pinyin_full),
        pinyin_abbr: Some(pinyin_abbr),
        ..Default::default()
    }
    .into();

    model
        .insert(&db)
        .await
        .map_err(|e| format!("创建默认分类失败: {}", e))?;

    Ok(())
}
