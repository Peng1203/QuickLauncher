use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder};
use tauri::State;

use crate::entity;

#[tauri::command]
pub async fn get_launch(
    category_id: i32,
    sort_by: Option<String>,
    sort_order: Option<String>,
    enabled: Option<bool>,
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<Model>, String> {
    let mut query = LaunchItems::find();

    // ------------------------
    // category 过滤
    // ------------------------
    if category_id == -1 {
        query = query.filter(Column::CategoryId.is_null());
    } else {
        query = query.filter(Column::CategoryId.eq(category_id));
    }

    // ------------------------
    // enabled 过滤
    // ------------------------
    if let Some(enabled) = enabled {
        query = query.filter(Column::Enabled.eq(enabled));
    }

    // ------------------------
    // 排序字段（白名单防止乱传）
    // ------------------------
    if let Some(sort_by) = sort_by {
        let column = match sort_by.as_str() {
            // "id" => Column::Id,
            "name" => Column::Name,
            "type" => Column::Type,
            "time" => Column::CreatedAt,
            // "updated_at" => Column::UpdatedAt,
            "order" => Column::OrderIndex, // 如果你有这个字段
            _ => Column::Id,               // 默认兜底
        };

        let order = match sort_order.as_deref() {
            Some("desc") => Order::Desc,
            _ => Order::Asc,
        };

        query = query.order_by(column, order);
    }

    // ------------------------
    // 执行查询
    // ------------------------
    query
        .all(&*db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
