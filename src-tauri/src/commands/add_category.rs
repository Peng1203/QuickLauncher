use crate::{entity, models::category_item::NewCategoryItem, AppState};
use entity::categories::{ActiveModel, Entity, Model};
use sea_orm::{ActiveValue::Set, EntityTrait};

#[tauri::command]
pub async fn add_category(
    item: NewCategoryItem,
    state: tauri::State<'_, AppState>,
) -> Result<Model, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let model = ActiveModel {
        name: Set(item.name),
        parent_id: Set(item.parent_id),
        association_directory: Set(item.association_directory),
        icon: Set(item.icon),
        exclude: Set(item.exclude),
        layout: Set(item.layout),
        order_index: Set(item.order_index),
        ..Default::default()
    };

    let res = Entity::insert(model)
        .exec_with_returning(&db)
        .await
        .map_err(|e| format!("插入分类失败: {}", e))?;

    Ok(res)
}
