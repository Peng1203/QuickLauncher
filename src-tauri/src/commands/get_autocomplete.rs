use entity::autocomplete_history::{Column, Entity as AutocompleteHistory};
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt;

use crate::entity;

#[tauri::command]
pub async fn get_autocomplete(
    keyword: &str,
    app: AppHandle,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<String>, String> {
    let mode = app
        .pinia()
        .get::<String>("appConfig", "autocompleteMatchMode")
        .unwrap();

    let enable_filter = app
        .pinia()
        .get::<bool>("appConfig", "enableAutocompleteFrequencyFilter")
        .unwrap();

    let keyword_pattern = match mode.as_str() {
        "prefix" => format!("{}*", keyword),
        "contains" => format!("*{}*", keyword),
        _ => format!("{}*", keyword),
    };

    let limit_value = 5;
    const MIN_USAGE_COUNT: i32 = 3;

    // 构建查询
    let mut query = AutocompleteHistory::find()
        .select_only()
        .column(Column::Query)
        .order_by_desc(Column::UsageCount)
        .order_by_desc(Column::LastUsedAt)
        .limit(limit_value);

    // 👇 GLOB 条件（核心）
    query = query.filter(Expr::cust_with_values(
        "query GLOB ?",
        [keyword_pattern.clone()],
    ));

    // 👇 频率过滤
    if enable_filter {
        query = query.filter(Column::UsageCount.gte(MIN_USAGE_COUNT));
    }

    let rows = query
        .into_tuple::<String>()
        .all(db.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}
