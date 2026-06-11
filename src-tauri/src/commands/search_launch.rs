use crate::{entity, models::launch_item::SearchLaunchItem, AppState};
use entity::{
    categories::{Column as CColumn, Entity as Categories},
    launch_items::{Column as LIColumn, Entity as LaunchItems},
};
use sea_orm::{
    sea_query::{Alias, Expr},
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn search_launch(
    keyword: &str,
    category_id: Option<i32>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SearchLaunchItem>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let like_pattern = format!("%{}%", keyword);

    let mut query = LaunchItems::find()
        .join(
            JoinType::LeftJoin,
            LaunchItems::belongs_to(Categories)
                .from(LIColumn::CategoryId)
                .to(CColumn::Id)
                .into(),
        )
        .join_as(
            JoinType::LeftJoin,
            LaunchItems::belongs_to(Categories)
                .from(LIColumn::SubcategoryId)
                .to(CColumn::Id)
                .into(),
            "c2",
        )
        .select_only()
        .column(LIColumn::Id)
        .column(LIColumn::Name)
        .column(LIColumn::Path)
        .column(LIColumn::Icon)
        .column(LIColumn::Type)
        .column(LIColumn::CategoryId)
        .column(LIColumn::SubcategoryId)
        .column_as(CColumn::Name, "category_name")
        .column_as(
            Expr::col((Alias::new("c2"), CColumn::Name)),
            "subcategory_name",
        )
        .filter(LIColumn::Enabled.eq(1))
        .filter(
            Condition::any()
                .add(LIColumn::Name.like(&like_pattern))
                .add(LIColumn::PinyinFull.like(&like_pattern))
                .add(LIColumn::PinyinAbbr.like(&like_pattern))
                .add(LIColumn::Keywords.like(&like_pattern)),
        );

    // 分类过滤
    if let Some(category_id) = category_id {
        query = query.filter(LIColumn::CategoryId.eq(category_id));
    }

    let results = query
        .order_by_desc(LIColumn::OrderIndex)
        .into_model::<SearchLaunchItem>()
        .all(&db)
        .await
        .map_err(|e| {
            tracing::error!(%e, "搜索启动项失败");
            format!("查询失败: {}", e)
        })?;

    tracing::info!(keyword, ?category_id, "搜索启动项");

    Ok(results)
}
