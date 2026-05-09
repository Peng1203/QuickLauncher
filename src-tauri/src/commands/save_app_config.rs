use entity::configs::{ActiveModel, Column, Entity as Configs};
use sea_orm::{sea_query::OnConflict, EntityTrait, Set};

use crate::{entity, AppState};

#[tauri::command]
pub async fn save_app_config(
    config: crate::models::config_item::OperConfigItem,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

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
        .exec(&db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
