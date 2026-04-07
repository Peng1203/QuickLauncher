use crate::{entity, models::category_item::NewCategoryItem};
use entity::categories::{ActiveModel, Entity, Model};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

#[tauri::command]
pub async fn add_category(
    item: NewCategoryItem,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Model, String> {
    let model = ActiveModel {
        name: Set(item.name),
        parent_id: Set(item.parent_id),
        association_directory: Set(item.association_directory),
        icon: Set(item.icon),
        exclude: Set(item.exclude),
        layout: Set(item.layout),
        ..Default::default()
    };

    let res = Entity::insert(model)
        .exec_with_returning(&*db)
        .await
        .map_err(|e| format!("插入分类失败: {}", e))?;

    Ok(res)
}
