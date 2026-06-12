use crate::entity::launch_items;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchItemDto {
    pub id: i32,
    pub name: String,
    pub lnk_name: Option<String>,
    pub path: String,
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
    pub launch_count: Option<i32>,
    pub failure_count: Option<i32>,
    pub last_used_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<launch_items::Model> for LaunchItemDto {
    fn from(model: launch_items::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            lnk_name: model.lnk_name,
            path: model.path,
            r#type: model.r#type,
            icon: model.icon,
            hotkey: model.hotkey,
            hotkey_global: model.hotkey_global,
            pinyin_full: model.pinyin_full,
            pinyin_abbr: model.pinyin_abbr,
            keywords: model.keywords,
            start_dir: model.start_dir,
            remarks: model.remarks,
            args: model.args,
            run_as_admin: model.run_as_admin,
            order_index: model.order_index,
            enabled: model.enabled,
            category_id: model.category_id,
            subcategory_id: model.subcategory_id,
            extension: model.extension,
            launch_count: model.launch_count,
            failure_count: model.failure_count,
            last_used_at: model.last_used_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLaunchItemDto {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLaunchItemDto {
    pub id: i32,
    pub name: Option<String>,
    pub path: Option<String>,
    pub lnk_name: Option<String>,
    pub r#type: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLaunchItemDto {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub icon: Option<String>,
    pub r#type: String,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
    pub subcategory_id: Option<i32>,
    pub subcategory_name: Option<String>,
}
