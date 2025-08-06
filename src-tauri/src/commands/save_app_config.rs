use crate::{db, models::config_item::OperConfigItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn save_app_config(config: OperConfigItem) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    // 使用 UPSERT 语句插入或更新配置项
    // 如果配置项已存在，则更新其数据；如果不存在，则插入新记录
    conn.execute(
        "INSERT INTO configs (name, data) VALUES (?1, ?2) ON CONFLICT(name) DO UPDATE SET data = excluded.data",
        params![config.name, config.data],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
