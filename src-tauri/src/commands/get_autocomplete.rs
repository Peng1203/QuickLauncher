use crate::db;
use rusqlite::{params, Result};
use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt;

#[tauri::command]
pub fn get_autocomplete(keyword: &str, app: AppHandle) -> Result<Vec<String>, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let mode = app
        .pinia()
        .get::<String>("appConfig", "autocompleteMatchMode")
        .unwrap();

    let enable_filter = app
        .pinia()
        .get::<bool>("appConfig", "enableAutocompleteFrequencyFilter")
        .unwrap();

    let keyword_pattern = match mode.as_str() {
        "prefix" => format!("{}*", keyword),
        "contains" => format!("*{}*", keyword),
        _ => format!("{}*", keyword), // 默认 prefix
    };
    let limit_value = 5;

    const MIN_USAGE_COUNT: i32 = 3;
    // 根据是否开启频率过滤选择不同 SQL
    let sql = if enable_filter {
        r#"
        SELECT query
        FROM autocomplete_history
        WHERE query GLOB ?
          AND usage_count >= ?
        ORDER BY usage_count DESC, last_used_at DESC
        LIMIT ?
        "#
    } else {
        r#"
        SELECT query
        FROM autocomplete_history
        WHERE query GLOB ?
        ORDER BY usage_count DESC, last_used_at DESC
        LIMIT ?
        "#
    };

    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

    let mapper = |row: &rusqlite::Row| {
        let query: String = row.get(0)?;
        Ok(query)
    };

    let rows = if enable_filter {
        stmt.query_map(
            params![keyword_pattern, MIN_USAGE_COUNT, limit_value],
            mapper,
        )
    } else {
        stmt.query_map(params![keyword_pattern, limit_value], mapper)
    }
    .map_err(|e| e.to_string())?;

    // let mut stmt = conn
    //     .prepare(
    //         "SELECT query
    //          FROM autocomplete_history
    //          WHERE query GLOB ?
    //          ORDER BY usage_count DESC, last_used_at DESC
    //          LIMIT ?",
    //     )
    //     .map_err(|e| e.to_string())?;

    // let rows = stmt
    //     .query_map(params![keyword_pattern, limit_value], |row| {
    //         let query: String = row.get(0)?;
    //         Ok(query)
    //     })
    //     .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for r in rows {
        results.push(r.map_err(|e| e.to_string())?);
    }

    // models::autocomplete_item::AutocompleteItem
    // let mut stmt = conn
    //     .prepare(
    //         "SELECT id, query, usage_count, last_used_at, launch_item_id
    //          FROM autocomplete_history
    //          WHERE query GLOB ?
    //          ORDER BY usage_count DESC, last_used_at DESC
    //          LIMIT ?",
    //     )
    //     .map_err(|e| e.to_string())?;
    // let rows = stmt
    //     .query_map(params![keyword_pattern, limit_value], |row| {
    //         Ok(AutocompleteItem {
    //             id: row.get(0)?,
    //             query: row.get(1)?,
    //             usage_count: row.get(2)?,
    //             last_used_at: row.get(3)?,
    //             launch_item_id: row.get(4)?,
    //         })
    //     })
    //     .map_err(|e| e.to_string())?;
    // let mut results = Vec::new();
    // for r in rows {
    //     results.push(r.map_err(|e| e.to_string())?);
    // }

    Ok(results)
}
