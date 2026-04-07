use crate::{entity, models::app_config_state::AppConfigState};
use entity::configs::ActiveModel;
use entity::configs::{Column, Entity as Configs};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[tauri::command]
pub async fn get_app_config(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<AppConfigState, String> {
    // 1. 先查
    let config = Configs::find()
        .filter(Column::Name.eq("appConfig"))
        .one(db.inner())
        .await
        .map_err(|e| format!("查询配置失败：{}", e))?;

    let config = match config {
        Some(model) => model,

        None => {
            // 2. 没有就插入默认值
            let model = ActiveModel {
                name: Set("appConfig".to_string()),
                data: Set("{}".to_string()),
                ..Default::default()
            };

            model
                .insert(db.inner())
                .await
                .map_err(|e| format!("插入默认配置失败：{}", e))?
        }
    };

    // 3. 解析 JSON
    let parsed_data: AppConfigState =
        serde_json::from_str(&config.data).map_err(|e| format!("解析 JSON 失败：{}", e))?;

    Ok(parsed_data)
}
