use std::collections::HashMap;

use crate::{
    entity::{
        categories,
        launch_items::{self, Column as LaunchColumn},
    },
    AppState,
};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

#[derive(Debug, serde::Serialize)]
pub struct CategoryWithItems {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub children: Vec<launch_items::Model>,
}

#[tauri::command]
pub async fn get_category_tree(
    // enabled: Option<bool>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<CategoryWithItems>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // ------------------------
    // 查询所有分类
    // ------------------------
    let category_data = categories::Entity::find()
        .order_by_desc(categories::Column::OrderIndex)
        .all(&db)
        .await
        .map_err(|e| format!("查询分类失败: {}", e))?;

    // ------------------------
    // 查询所有启动项
    // ------------------------
    let launch_query = launch_items::Entity::find().filter(LaunchColumn::Type.ne("alias"));

    // if let Some(enabled) = enabled {
    //     launch_query = launch_query.filter(LaunchColumn::Enabled.eq(enabled));
    // }

    let launch_items = launch_query
        .order_by_asc(LaunchColumn::OrderIndex)
        .all(&db)
        .await
        .map_err(|e| format!("查询启动项失败: {}", e))?;

    // ------------------------
    // 按 category_id 分组
    // ------------------------
    let mut launch_map: HashMap<i32, Vec<launch_items::Model>> = HashMap::new();

    for item in launch_items {
        if let Some(category_id) = item.category_id {
            launch_map.entry(category_id).or_default().push(item);
        }
    }

    // ------------------------
    // 组装树结构
    // ------------------------
    let tree = category_data
        .into_iter()
        .map(|category| {
            let children = launch_map.remove(&category.id).unwrap_or_default();

            CategoryWithItems {
                id: category.id,
                name: category.name,
                icon: category.icon,
                children,
            }
        })
        .collect();

    Ok(tree)
}
