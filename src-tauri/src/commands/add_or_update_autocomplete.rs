use crate::entity::{self, autocomplete_history::Column};
use entity::autocomplete_history::ActiveModel;
use entity::prelude::AutocompleteHistory;
use sea_orm::{
    sea_query::Expr, sea_query::OnConflict, ActiveValue::Set, DatabaseConnection, EntityTrait,
    ExprTrait,
};

#[tauri::command]
pub async fn add_or_update_autocomplete(
    query: &str,
    launch_item_id: Option<i32>,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let model = ActiveModel {
        query: Set(query.to_string()),
        usage_count: Set(Some(1)),
        launch_item_id: Set(launch_item_id),
        ..Default::default()
    };

    AutocompleteHistory::insert(model)
        .on_conflict(
            OnConflict::column(Column::Query)
                .update_columns([Column::LaunchItemId]) // 可选更新字段
                .value(Column::UsageCount, Expr::col(Column::UsageCount).add(1))
                .value(Column::LastUsedAt, Expr::current_timestamp())
                .to_owned(),
        )
        .exec(db.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
