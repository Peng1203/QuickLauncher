use crate::{db, models::category_item::CategoryItem};
use rusqlite::{params, Result};

#[tauri::command]
pub fn get_category() -> Result<Vec<CategoryItem>, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM categories")
        .map_err(|e| format!("准备查询语句失败：{}", e))
        .unwrap();

    // 执行查询并映射结果到 CategoryItem 结构体
    let rows = stmt.query_map(params![], CategoryItem::from_row);

    // 收集所有结果到一个向量中
    let mut items = Vec::new();
    for row in rows.unwrap() {
        let item = row.unwrap();
        items.push(item);
    }

    Ok(items)
}
