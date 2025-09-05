use crate::{db, models::launch_item::LaunchItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn get_launch(category_id: i32) -> Result<Vec<LaunchItem>, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let sql = if category_id == -1 {
        "SELECT * FROM launch_items"
    } else {
        "SELECT * FROM launch_items WHERE category_id = ?1"
    };
    let params = if category_id == -1 {
        params![]
    } else {
        params![category_id]
    };

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| format!("准备查询语句失败：{}", e))
        .unwrap();

    // 执行查询并映射结果到 LaunchItem 结构体
    let rows = stmt.query_map(params, LaunchItem::from_row);

    // 收集所有结果到一个向量中
    let mut items = Vec::new();
    for row in rows.unwrap() {
        let item = row.unwrap();
        items.push(item);
    }

    Ok(items)
}
