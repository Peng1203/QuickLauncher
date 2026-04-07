use entity::configs::{ActiveModel, Column, Entity as Configs};
use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait, Set};

use crate::entity;

#[tauri::command]
pub async fn save_app_config(
    config: crate::models::config_item::OperConfigItem,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let active = ActiveModel {
        name: Set(config.name.clone()),
        data: Set(config.data.clone()),
        ..Default::default()
    };

    Configs::insert(active)
        .on_conflict(
            OnConflict::column(Column::Name)
                .update_columns([Column::Data])
                .to_owned(),
        )
        .exec(db.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
