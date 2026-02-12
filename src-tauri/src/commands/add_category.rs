use crate::db;
use crate::models::category_item::{CategoryItem, NewCategoryItem};
use rusqlite::params;

#[tauri::command]
pub fn add_category(item: NewCategoryItem) -> Result<CategoryItem, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    conn.execute(
        "INSERT INTO categories 
        (name, parent_id, association_directory, icon, exclude, layout) VALUES 
        (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            item.name,
            item.parent_id,
            item.association_directory,
            item.icon,
            item.exclude,
            item.layout
        ],
    )
    .map_err(|e| format!("插入分类失败: {}", e))?; // 出错时返回字符串给前端

    // 2️⃣ 获取刚刚插入的自增 ID
    let new_id = conn.last_insert_rowid();

    // 3️⃣ 查询并返回完整记录
    let mut stmt = conn
        .prepare("SELECT * FROM categories WHERE id = ?1")
        .map_err(|e| format!("查询新分类失败: {}", e))?;

    let category = stmt
        .query_row(params![new_id], CategoryItem::from_row)
        .unwrap();

    Ok(category)
}
