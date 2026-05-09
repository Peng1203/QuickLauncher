use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{ColumnTrait, Condition, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::{entity, AppState};

#[tauri::command]
pub async fn get_launch(
    category_id: i32,
    default_category: Option<bool>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    enabled: Option<bool>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Model>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let mut query = LaunchItems::find();

    // 默认为 false
    let default_category = default_category.unwrap_or(false);

    // ------------------------
    // category 过滤
    // ------------------------
    if default_category {
        // 默认分类：查询 category_id 为 null 或 当前分类 id
        query = query.filter(
            Condition::any()
                .add(Column::CategoryId.is_null())
                .add(Column::CategoryId.eq(category_id)),
        );
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
    // 排除 alias
    // ------------------------
    query = query.filter(Column::Type.ne("alias"));

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
                    .order_by(Column::Extension, order) // 次排序
            }
            "time" => query.order_by(Column::CreatedAt, order),
            "order" => query.order_by(Column::OrderIndex, order),
            _ => query.order_by(Column::Id, order),
        };
    }

    // ------------------------
    // 执行查询
    // ------------------------
    query.all(&db).await.map_err(|e| format!("查询失败：{}", e))
}
