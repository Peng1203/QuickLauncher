use crate::db;
use crate::models::category_item::NewCategoryItem;
use rusqlite::params;

#[tauri::command]
pub fn add_category(item: NewCategoryItem) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    conn.execute(
        "INSERT INTO categories 
        (name, parent_id, association_directory) VALUES 
        (?1, ?2, ?3)",
        params![item.name, item.parent_id, item.association_directory],
    )
    .map_err(|e| format!("插入分类失败: {}", e))?; // 出错时返回字符串给前端

    Ok(())
}
