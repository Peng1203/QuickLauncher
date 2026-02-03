use crate::{db, models::launch_item::LaunchItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn get_launch_by_id(id: i32) -> Result<Option<LaunchItem>, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM launch_items WHERE id = ?1")
        .map_err(|e| format!("准备查询语句失败：{}", e))?;

    let mut rows = stmt
        .query_map(params![id], LaunchItem::from_row)
        .map_err(|e| format!("执行查询失败：{}", e))?;

    if let Some(row) = rows.next() {
        let item = row.map_err(|e| format!("解析数据失败：{}", e))?;
        Ok(Some(item))
    } else {
        Ok(None) // 没查到
    }
}
