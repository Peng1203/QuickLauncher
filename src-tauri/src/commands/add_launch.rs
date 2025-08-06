use crate::db;
use crate::models::launch_item::NewLaunchItem;
use pinyin::ToPinyin;

fn get_pinyin_variants(name: &str) -> (String, String) {
    let mut full = String::new();
    let mut abbr = String::new();

    for ch in name.chars() {
        if let Some(p) = ch.to_pinyin() {
            let p_str = p.plain().to_string();
            full.push_str(&p_str);
            abbr.push(p_str.chars().next().unwrap_or_default());
        } else {
            // 处理非中文字符（保留原字符）
            full.push(ch);
            abbr.push(ch);
        }
    }

    (full.to_lowercase(), abbr.to_lowercase())
}

#[tauri::command]
pub fn add_launch(item: NewLaunchItem) -> Result<(), String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let pinyin_value = get_pinyin_variants(&item.name);
    let pinyin_full = pinyin_value.0;
    let pinyin_abbr = pinyin_value.1;

    conn.execute(
        "INSERT INTO launch_items (name, path, type, icon, pinyin_full, pinyin_abbr) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (item.name, item.path, item.r#type, item.icon, pinyin_full, pinyin_abbr),
    )
    .expect("Failed to insert new launch item");
    Ok(())
}
