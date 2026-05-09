use crate::AppState;
use crate::{entity, models::app_config_state::AppConfigState};
use entity::configs::ActiveModel;
use entity::configs::{Column, Entity as Configs};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

#[tauri::command]
pub async fn get_app_config(
    state: tauri::State<'_, AppState>,
) -> Result<Option<AppConfigState>, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // 1. 先查
    let config = Configs::find()
        .filter(Column::Name.eq("appConfig"))
        .one(&db)
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
                .insert(&db)
                .await
                .map_err(|e| format!("插入默认配置失败：{}", e))?
        }
    };
    if config.data.trim() == "{}" {
        return Ok(None);
    }

    // 3. 解析 JSON
    let parsed_data: AppConfigState =
        serde_json::from_str(&config.data).map_err(|e| format!("解析 JSON 失败：{}", e))?;

    Ok(Some(parsed_data))
}
