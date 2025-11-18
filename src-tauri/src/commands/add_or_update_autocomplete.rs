use crate::db;
use rusqlite::{params, Result};

#[tauri::command]
pub fn add_or_update_autocomplete(query: &str, launch_item_id: Option<i32>) -> Result<(), String> {
    dbg!(&query, &launch_item_id);
    let conn = db::connection::get_conn().lock().unwrap();

    // 先尝试更新已有记录
    let updated = conn
        .execute(
            "UPDATE autocomplete_history
             SET usage_count = usage_count + 1,
             last_used_at = CURRENT_TIMESTAMP,
             launch_item_id = COALESCE(?, launch_item_id)
             WHERE query = ?",
            params![launch_item_id, query],
        )
        .map_err(|e| e.to_string())?;

    if updated == 0 {
        // 没有更新到任何记录，说明 query 不存在 → 插入新记录
        conn.execute(
            "INSERT INTO autocomplete_history (query, usage_count, last_used_at, launch_item_id)
             VALUES (?1, 1, CURRENT_TIMESTAMP, ?2)",
            params![query, launch_item_id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
