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
        "INSERT INTO launch_items (name, path, type, icon, pinyin_full, pinyin_abbr, extension) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (item.name, item.path, item.r#type, item.icon, pinyin_full, pinyin_abbr, item.extension),
    )
    .expect("Failed to insert new launch item");
    Ok(())
}
