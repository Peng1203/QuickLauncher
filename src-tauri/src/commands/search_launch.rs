use crate::{db, models::launch_item::SearchLaunchItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn search_launch(keyword: &str) -> Result<Vec<SearchLaunchItem>, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let like_pattern = format!("%{}%", keyword);
    let mut stmt = conn
        .prepare(
            "SELECT id, name, icon FROM launch_items WHERE name LIKE ?1
            OR pinyin_full LIKE ?1
            OR pinyin_abbr LIKE ?1
            OR keywords LIKE ?1
         ORDER BY order_index ASC",
        )
        .map_err(|e| format!("准备查询语句失败：{}", e))
        .unwrap();

    let rows = stmt.query_map(params![like_pattern], SearchLaunchItem::from_row);

    // 收集所有结果到一个向量中
    let mut items = Vec::new();
    for row in rows.unwrap() {
        let item = row.unwrap();
        items.push(item);
    }

    Ok(items)
}
