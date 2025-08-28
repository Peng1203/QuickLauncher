use rusqlite::params;

use crate::common::utils::get_pinyin_variants;
use crate::db;
use crate::models::launch_item::LaunchItem;

#[tauri::command]
pub fn update_launch(item: LaunchItem) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let pinyin_value = get_pinyin_variants(&item.name);
    let pinyin_full = pinyin_value.0;
    let pinyin_abbr = pinyin_value.1;

    let rows_affected = conn
        .execute(
            "UPDATE launch_items SET 
        name = ?1, 
        path = ?2, 
        type = ?3, 
        icon = ?4, 
        pinyin_full = ?5, 
        pinyin_abbr = ?6, 
        extension = ?7, 
        hotkey = ?8, 
        hotkey_global = ?9, 
        keywords = ?10, 
        start_dir = ?11, 
        remarks = ?12, 
        args = ?13, 
        run_as_admin = ?14, 
        order_index = ?15, 
        enabled = ?16, 
        category_id = ?17 
        WHERE id = ?18",
            params![
                item.name,
                item.path,
                item.r#type,
                item.icon,
                pinyin_full,
                pinyin_abbr,
                item.extension,
                item.hotkey,
                item.hotkey_global,
                item.keywords,
                item.start_dir,
                item.remarks,
                item.args,
                item.run_as_admin,
                item.order_index,
                item.enabled,
                item.category_id,
                item.id // WHERE 条件中的 id
            ],
        )
        .map_err(|e| format!("Failed to update launch item: {}", e))?;

    // 检查是否有记录被更新
    if rows_affected == 0 {
        return Err("No launch item found with the specified ID".to_string());
    }

    Ok(())
}
