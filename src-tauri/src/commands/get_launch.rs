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
    // if let Some(sort_by) = sort_by {
    //     let column = match sort_by.as_str() {
    //         "id" => Column::Id,
    //         "name" => Column::Name,
    //         "type" => Column::Type,
    //         "time" => Column::CreatedAt,
    //         // "updated_at" => Column::UpdatedAt,
    //         "order" => Column::OrderIndex,
    //         _ => Column::Id,
    //     };
    //     let order = match sort_order.as_deref() {
    //         Some("desc") => Order::Desc,
    //         _ => Order::Asc,
    //     };
    //     query = query.order_by(column, order);
    // }
    if let Some(sort_by) = sort_by {
        let order = match sort_order.as_deref() {
            Some("desc") => Order::Desc,
            _ => Order::Asc,
        };

        query = match sort_by.as_str() {
            "name" => query.order_by(Column::Name, order),
            "type" => {
                query
                    .order_by(Column::Type, order.clone())
                    .order_by(Column::Extension, order) // ✅ 次排序
            }
            "time" => query.order_by(Column::CreatedAt, order),
            "order" => query.order_by(Column::OrderIndex, order),
            _ => query.order_by(Column::Id, order),
        };
    }

    // ------------------------
    // 执行查询
    // ------------------------
    query
        .all(&*db)
        .await
        .map_err(|e| format!("查询失败：{}", e))
}
