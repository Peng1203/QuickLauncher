use rusqlite::params;

use crate::db;
use crate::models::category_item::CategoryItem;

#[tauri::command]
pub fn update_category(item: CategoryItem) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let rows_affected = conn
        .execute(
            "UPDATE categories SET 
        name = ?, 
        parent_id = ?, 
        association_directory = ?,
        exclude = ?,
        layout = ?,
        icon = ?
        WHERE id = ?",
            params![
                item.name,
                item.parent_id,
                item.association_directory.clone().unwrap(),
                item.exclude.clone().unwrap(),
                item.layout.clone().unwrap(),
                item.icon.clone().unwrap(),
                item.id
            ],
        )
        .map_err(|e| format!("Failed to update item: {}", e))?;

    // 检查是否有记录被更新
    if rows_affected == 0 {
        return Err("No item found with the specified ID".to_string());
    }

    Ok(())
}
