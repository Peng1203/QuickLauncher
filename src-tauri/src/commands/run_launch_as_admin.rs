use crate::common::utils::run_as_admin;
use crate::db;
use crate::models::launch_item::LaunchItem;
use rusqlite::{params, Result};

#[tauri::command]
pub fn run_launch_as_admin(id: i32) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM launch_items WHERE id = ?")
        .map_err(|e| format!("准备查询语句失败：{}", e))
        .unwrap();

    let row = stmt.query_row(params![id], LaunchItem::from_row);

    let launch_item = row.unwrap();
    // println!("名称：{}", launch_item);
    log::info!("id:{}, path:{}", id, launch_item.path);

    run_as_admin(launch_item)?;

    Ok(())
}
