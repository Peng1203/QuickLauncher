use crate::{db, models::config_item::ConfigItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn get_app_config() -> Result<ConfigItem, String> {
    // state: State<'_, Mutex<Connection>>,
    // let conn = state.lock().unwrap();

    let conn = db::connection::get_conn().lock().unwrap();

    // 使用 UPSERT 语句插入或更新配置项
    // 如果配置项已存在，则更新其数据；如果不存在，则插入新记录
    let mut stmt = conn
        .prepare("SELECT * FROM configs WHERE name = ?")
        .map_err(|e| format!("准备查询语句失败：{}", e))?;

    let params = params!["appConfig"];
    let row = stmt.query_row(params, |row| {
        Ok(ConfigItem {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    });

    let config_item = match row {
        Ok(item) => item,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // 没查到，插入默认值
            conn.execute(
                "INSERT INTO configs (name, data) VALUES (?1, ?2)",
                rusqlite::params!["appConfig", "{}"],
            )
            .map_err(|e| format!("插入默认配置失败：{}", e))?;

            // 再查一遍
            let mut stmt = conn
                .prepare("SELECT * FROM configs WHERE name = ?")
                .map_err(|e| format!("准备查询语句失败：{}", e))?;

            stmt.query_row(rusqlite::params!["appConfig"], |row| {
                Ok(ConfigItem {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    data: row.get(2)?,
                })
            })
            .map_err(|e| format!("查询插入后的配置失败：{}", e))?
        }
        Err(e) => return Err(format!("查询失败：{}", e)),
    };

    Ok(config_item)
    // let config_item = row.unwrap();
    // println!("哈哈哈哈3");
    // Ok(config_item)
}
