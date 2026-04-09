use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    Set,
};

use crate::entity::categories;
use crate::models::category_item::CategoryItem;

#[tauri::command]
pub async fn update_category(
    item: CategoryItem,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    // 先查是否存在
    let model = categories::Entity::find_by_id(item.id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败: {}", e))?;

    let model = match model {
        Some(m) => m,
        None => return Err("No item found with the specified ID".to_string()),
    };

    // 转 ActiveModel
    let mut active: categories::ActiveModel = model.into();

    // 更新字段
    active.name = Set(item.name);
    active.parent_id = Set(item.parent_id);
    active.association_directory = Set(item.association_directory);
    active.exclude = Set(item.exclude);
    active.layout = Set(item.layout);
    active.sort_by = Set(item.sort_by);
    active.sort_order = Set(item.sort_order);
    active.icon = Set(item.icon);

    // 执行更新
    active
        .update(db.inner())
        .await
        .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_category_ass_dir(
    id: i32,
    ass_dir: String,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    // 先查
    let model = categories::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败: {}", e))?;

    // 判空
    let mut model: categories::ActiveModel = match model {
        Some(m) => m.into(),
        None => return Err("No item found with the specified ID".to_string()),
    };

    // 更新字段
    model.association_directory = Set(Some(ass_dir));

    // 执行更新
    model
        .update(db.inner())
        .await
        .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}
