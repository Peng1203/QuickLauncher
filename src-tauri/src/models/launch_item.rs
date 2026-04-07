#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewLaunchItem {
    pub name: String,
    pub path: String,
    pub lnk_name: Option<String>,
    pub r#type: String,
    pub icon: Option<String>,
    pub hotkey: Option<String>,
    pub hotkey_global: Option<bool>,
    pub pinyin_full: Option<String>,
    pub pinyin_abbr: Option<String>,
    pub keywords: Option<String>,
    pub start_dir: Option<String>,
    pub remarks: Option<String>,
    pub args: Option<String>,
    pub run_as_admin: Option<bool>,
    pub order_index: Option<i32>,
    pub enabled: Option<bool>,
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>,
    pub extension: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, sea_orm::FromQueryResult)]
pub struct SearchLaunchItem {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
    pub subcategory_id: Option<i32>,
    pub subcategory_name: Option<String>,
}

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, serde::Serialize, sea_orm::FromQueryResult)]
pub struct LaunchItemDto {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub r#type: String,
    pub icon: Option<String>,
    pub hotkey: Option<String>,
    pub hotkey_global: Option<bool>,
    pub keywords: Option<String>,
    pub start_dir: Option<String>,
    pub remarks: Option<String>,
    pub args: Option<String>,
    pub run_as_admin: Option<bool>,
    pub order_index: Option<i32>,
    pub enabled: Option<bool>,
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>,
    pub extension: Option<String>,
}
