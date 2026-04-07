use crate::{entity, models::launch_item::SearchLaunchItem};
use entity::{
    categories::{Column as CColumn, Entity as Categories},
    launch_items::{Column as LIColumn, Entity as LaunchItems},
};
use sea_orm::{
    sea_query::{Alias, Expr},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QueryOrder,
    QuerySelect,
};

#[tauri::command]
pub async fn search_launch(
    keyword: &str,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<SearchLaunchItem>, String> {
    let like_pattern = format!("%{}%", keyword);

    let results = LaunchItems::find()
        // LEFT JOIN categories c1 (category)
        .join(
            JoinType::LeftJoin,
            LaunchItems::belongs_to(Categories)
                .from(LIColumn::CategoryId)
                .to(CColumn::Id)
                .into(),
        )
        // LEFT JOIN categories c2 (subcategory) —— 手动 alias
        .join_as(
            JoinType::LeftJoin,
            LaunchItems::belongs_to(Categories)
                .from(LIColumn::SubcategoryId)
                .to(CColumn::Id)
                .into(),
            "c2",
        )
        // 只选择需要的字段
        .select_only()
        .column(LIColumn::Id)
        .column(LIColumn::Name)
        .column(LIColumn::Icon)
        .column(LIColumn::CategoryId)
        .column(LIColumn::SubcategoryId)
        // category_name
        .column_as(CColumn::Name, "category_name")
        // subcategory_name（来自 c2）
        .column_as(
            Expr::col((Alias::new("c2"), CColumn::Name)),
            "subcategory_name",
        )
        // WHERE enabled = 1
        .filter(LIColumn::Enabled.eq(1))
        // 模糊搜索（OR）
        .filter(
            Condition::any()
                .add(LIColumn::Name.like(&like_pattern))
                .add(LIColumn::PinyinFull.like(&like_pattern))
                .add(LIColumn::PinyinAbbr.like(&like_pattern))
                .add(LIColumn::Keywords.like(&like_pattern)),
        )
        // ORDER BY
        .order_by_desc(LIColumn::OrderIndex)
        // 映射到自定义结构体
        .into_model::<SearchLaunchItem>()
        .all(db.inner())
        .await
        .map_err(|e| format!("查询失败: {}", e))?;

    Ok(results)
}
