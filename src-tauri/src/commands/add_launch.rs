use rusqlite::params;

use crate::db;

use crate::common::utils::get_pinyin_variants;
use crate::models::launch_item::NewLaunchItem;

#[tauri::command]
pub fn add_launch(item: NewLaunchItem) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let pinyin_value = get_pinyin_variants(&item.name);
    let pinyin_full = pinyin_value.0;
    let pinyin_abbr = pinyin_value.1;

    conn.execute(
        "INSERT INTO launch_items 
        (name, path, type, icon, pinyin_full, pinyin_abbr, extension, hotkey, hotkey_global, keywords, start_dir, remarks, args, run_as_admin, order_index, enabled, category_id, subcategory_id) VALUES 
        (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)", 
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
        item.subcategory_id
    ]).expect("Failed to insert new launch item");
    Ok(())
}
