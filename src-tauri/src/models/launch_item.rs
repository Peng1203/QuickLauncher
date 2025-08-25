use rusqlite::{Result, Row};

#[derive(Debug, Clone, serde::Serialize)]
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
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            r#type: row.get(3)?,
            icon: row.get(4)?,
            hotkey: row.get(5)?,
            hotkey_global: row.get(6)?,
            pinyin_full: row.get(7)?,
            pinyin_abbr: row.get(8)?,
            keywords: row.get(9)?,
            start_dir: row.get(10)?,
            remarks: row.get(11)?,
            args: row.get(12)?,
            run_as_admin: row.get(13)?,
            order_index: row.get(14)?,
            enabled: row.get(15)?,
            category_id: row.get(16)?,
            last_used_at: row.get(17)?,
            created_at: row.get(18)?,
            updated_at: row.get(19)?,
            extension: row.get(20)?,
            launch_count: row.get(21)?,
            failure_count: row.get(22)?,
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
    pub extension: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchLaunchItem {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
}

impl SearchLaunchItem {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
        })
    }
}
