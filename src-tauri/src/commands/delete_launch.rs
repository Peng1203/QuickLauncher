use crate::db;
use rusqlite::{params, Result};

/// 删除启动项
#[tauri::command]
pub fn delete_launch(id: i32) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    conn.execute("DELETE FROM launch_items WHERE id = ?1", params![id])
        .map_err(|e| format!("删除启动项失败：{}", e))?;

    Ok(())
}
