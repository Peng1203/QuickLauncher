use rusqlite::{Result, Row};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LaunchItem {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub r#type: String, // "type" 是关键字，需加 r#  file, directory, url 等
    pub icon: Option<String>,
    pub hotkey: Option<String>,
    pub hotkey_global: i32, // 0 or 1
    pub pinyin_full: Option<String>,
    pub pinyin_abbr: Option<String>,
    pub keywords: Option<String>,
    pub start_dir: Option<String>,
    pub remarks: Option<String>,
    pub args: Option<String>,
    pub run_as_admin: i32, // 0 or 1
    pub order_index: i32,
    pub enabled: i32, // 0 or 1
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>,  // 新增字段
    pub last_used_at: Option<String>, // 可用 chrono::NaiveDateTime 代替
    pub created_at: String,
    pub updated_at: String,
    pub extension: Option<String>, // TEXT，可能为 NULL
    pub launch_count: i32,         // INTEGER，默认 0
    pub failure_count: i32,        // INTEGER，默认 0
}

impl LaunchItem {
    /// 从 rusqlite::Row 映射出 LaunchItem
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            path: row.get("path")?,
            r#type: row.get("type")?,
            icon: row.get("icon")?,
            hotkey: row.get("hotkey")?,
            hotkey_global: row.get("hotkey_global")?,
            pinyin_full: row.get("pinyin_full")?,
            pinyin_abbr: row.get("pinyin_abbr")?,
            keywords: row.get("keywords")?,
            start_dir: row.get("start_dir")?,
            remarks: row.get("remarks")?,
            args: row.get("args")?,
            run_as_admin: row.get("run_as_admin")?,
            order_index: row.get("order_index")?,
            enabled: row.get("enabled")?,
            category_id: row.get("category_id")?,
            subcategory_id: row.get("subcategory_id")?,
            last_used_at: row.get("last_used_at")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            extension: row.get("extension")?,
            launch_count: row.get("launch_count")?,
            failure_count: row.get("failure_count")?,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewLaunchItem {
    pub name: String,
    pub path: String,
    pub r#type: String,
    pub icon: String,
    pub hotkey: Option<String>,
    pub hotkey_global: Option<i32>,
    pub pinyin_full: Option<String>,
    pub pinyin_abbr: Option<String>,
    pub keywords: Option<String>,
    pub start_dir: Option<String>,
    pub remarks: Option<String>,
    pub args: Option<String>,
    pub run_as_admin: Option<i32>,
    pub order_index: Option<i32>,
    pub enabled: Option<i32>,
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>,
    pub extension: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchLaunchItem {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
    pub subcategory_id: Option<i32>,
    pub subcategory_name: Option<String>,
}

impl SearchLaunchItem {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
            category_id: row.get(3)?,
            category_name: row.get(4)?,
            subcategory_id: row.get(5)?,
            subcategory_name: row.get(6)?,
        })
    }
}
