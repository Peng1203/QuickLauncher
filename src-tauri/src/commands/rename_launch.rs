use crate::{common::utils::get_pinyin_variants, db};
use rusqlite::{params, Result};

// 重命名启动项
#[tauri::command]
pub fn rename_launch(id: i32, name: String) -> Result<(), String> {
    println!("rename_launch id: {}, name: {}", id, name);
    // 获取新命名的 拼音
    let conn = db::connection::get_conn().lock().unwrap();
    let pinyin_value = get_pinyin_variants(&name);
    let pinyin_full = pinyin_value.0;
    let pinyin_abbr = pinyin_value.1;

    conn.execute(
        "UPDATE launch_items 
        SET name = ?, 
        pinyin_full = ?, 
        pinyin_abbr = ? 
        WHERE id = ?",
        params![name, pinyin_full, pinyin_abbr, id],
    )
    .map_err(|e| format!("重命名启动项失败：{}", e))?;

    Ok(())
}
