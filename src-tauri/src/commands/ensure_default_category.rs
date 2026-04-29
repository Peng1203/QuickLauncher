use crate::entity;
use entity::categories::{ActiveModel, Entity};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, PaginatorTrait};

// 确保默认分类存在
#[tauri::command]
pub async fn ensure_default_category(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    // 查询是否存在任意分类数据
    let count = Entity::find()
        .count(&*db)
        .await
        .map_err(|e| format!("查询分类数量失败: {}", e))?;

    // 如果已有分类，则跳过
    if count > 0 {
        return Ok(());
    }

    // 创建默认分类
    let model = ActiveModel {
        name: Set("默认".to_string()),
        parent_id: Set(None),
        association_directory: Set(None),
        icon: Set(None),
        exclude: Set(false),
        layout: Set("grid".to_string()),
        order_index: Set(Some(9999)), // 将默认分类order_index设置为9999
        sort_by: Set("time".to_string()),
        ..Default::default()
    };

    Entity::insert(model)
        .exec(&*db)
        .await
        .map_err(|e| format!("创建默认分类失败: {}", e))?;

    Ok(())
}
