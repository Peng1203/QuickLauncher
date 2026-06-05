use crate::{entity, AppState};
use entity::todos::Entity as Todos;
use entity::todos::Model as TodoModel;

use sea_orm::sea_query::Expr;
use sea_orm::ExprTrait;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TodoSortBy {
    Priority,
    CreatedAt,
    DueDate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoListResult {
    pub todos: Vec<TodoModel>,
    pub total: i64,
    pub active_count: i64,
    pub completed_count: i64,
}

impl TodoSortBy {
    fn apply(&self, query: sea_orm::Select<Todos>) -> sea_orm::Select<Todos> {
        use entity::todos::Column;

        match self {
            Self::Priority => query.order_by_desc(Column::Priority),
            Self::CreatedAt => query.order_by_desc(Column::CreatedAt),
            Self::DueDate => query
                .order_by(Expr::col(Column::DueDate).is_null(), sea_orm::Order::Asc)
                .order_by_asc(Column::DueDate),
        }
    }
}

fn apply_filter(query: sea_orm::Select<Todos>, filter: &TodoFilter) -> sea_orm::Select<Todos> {
    use entity::todos::Column;

    match filter {
        TodoFilter::All => query,
        TodoFilter::Active => query.filter(Column::Completed.eq(false)),
        TodoFilter::Completed => query.filter(Column::Completed.eq(true)),
    }
}

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn get_todos(
    sort_by: Option<TodoSortBy>,
    filter: Option<TodoFilter>,
    state: tauri::State<'_, AppState>,
) -> Result<TodoListResult, String> {
    let db = state.db.lock().unwrap().clone().ok_or("数据库未连接")?;

    let sort_by = sort_by.unwrap_or(TodoSortBy::CreatedAt);
    let filter = filter.unwrap_or(TodoFilter::All);

    use entity::todos::Column;

    // 统计数量
    let total = Todos::find()
        .count(&db)
        .await
        .map_err(|e| format!("查询总数失败: {e}"))? as i64;

    let active_count = Todos::find()
        .filter(Column::Completed.eq(false))
        .count(&db)
        .await
        .map_err(|e| format!("查询进行中数量失败: {e}"))? as i64;

    let completed_count = Todos::find()
        .filter(Column::Completed.eq(true))
        .count(&db)
        .await
        .map_err(|e| format!("查询已完成数量失败: {e}"))? as i64;

    // 查询数据
    let mut query = Todos::find();

    query = apply_filter(query, &filter);

    // 未完成永远排前面
    query = query.order_by_asc(Column::Completed);

    // 用户选择的排序
    query = sort_by.apply(query);

    let todos = query.all(&db).await.map_err(|e| {
        tracing::error!(%e, "查询待办事项失败");
        format!("查询待办事项失败: {e}")
    })?;

    Ok(TodoListResult {
        todos,
        total,
        active_count,
        completed_count,
    })
}
