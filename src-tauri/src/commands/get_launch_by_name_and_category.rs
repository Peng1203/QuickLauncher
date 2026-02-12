use crate::{db, models::launch_item::LaunchItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn get_launch_by_name_and_category(
    category_id: i32,
    name: String,
) -> Result<Option<LaunchItem>, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let sql = r#"
        SELECT * FROM launch_items
        WHERE category_id = ?1
        AND name = ?2
        LIMIT 1
    "#;

    match conn.query_row(sql, params![category_id, name], LaunchItem::from_row) {
        Ok(item) => Ok(Some(item)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("查询失败: {}", e)),
    }
}
