use crate::{db, models::launch_item::SearchLaunchItem};
use rusqlite::{params, Result};

// #[tauri::command]
// pub fn search_launch(keyword: &str) -> Result<Vec<SearchLaunchItem>, String> {
//     let conn = db::connection::get_conn().lock().unwrap();

//     let like_pattern = format!("%{}%", keyword);
//     let mut stmt = conn
//         .prepare(
//             "SELECT id, name, icon, category_id, subcategory_id
//             FROM launch_items
//             WHERE enabled = 1
//             AND (
//               name LIKE ?1
//               OR pinyin_full LIKE ?1
//               OR pinyin_abbr LIKE ?1
//               OR keywords LIKE ?1
//             )
//             ORDER BY order_index ASC",
//         )
//         .map_err(|e| format!("准备查询语句失败：{}", e))
//         .unwrap();

//     let rows = stmt.query_map(params![like_pattern], SearchLaunchItem::from_row);

//     // 收集所有结果到一个向量中
//     let mut items = Vec::new();
//     for row in rows.unwrap() {
//         let item = row.unwrap();
//         items.push(item);
//     }

//     Ok(items)
// }

#[tauri::command]
pub fn search_launch(keyword: &str) -> Result<Vec<SearchLaunchItem>, String> {
    let conn = db::connection::get_conn()
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    let like_pattern = format!("%{}%", keyword);

    let mut stmt = conn
        .prepare(
            "SELECT 
                li.id,
                li.name,
                li.icon,
                li.category_id,
                c1.name AS category_name,
                li.subcategory_id,
                c2.name AS subcategory_name
             FROM launch_items li
             LEFT JOIN categories c1 ON li.category_id = c1.id
             LEFT JOIN categories c2 ON li.subcategory_id = c2.id
             WHERE li.enabled = 1
               AND (
                 li.name LIKE ?1
                 OR li.pinyin_full LIKE ?1
                 OR li.pinyin_abbr LIKE ?1
                 OR li.keywords LIKE ?1
               )
             ORDER BY li.order_index DESC",
        )
        .map_err(|e| format!("准备查询语句失败: {}", e))?;

    let rows = stmt
        .query_map(params![like_pattern], SearchLaunchItem::from_row)
        .map_err(|e| format!("执行查询失败: {}", e))?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| format!("读取行失败: {}", e))?);
    }

    Ok(items)
}
