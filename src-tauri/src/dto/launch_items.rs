use crate::{common::utils::get_pinyin_variants, entity::launch_items};
use sea_orm::ActiveValue::{NotSet, Set};
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
    pub last_used_at: Option<i64>,
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

impl From<CreateLaunchItemDto> for launch_items::ActiveModel {
    fn from(dto: CreateLaunchItemDto) -> Self {
        Self {
            name: Set(dto.name),
            path: Set(dto.path),
            lnk_name: Set(dto.lnk_name),
            r#type: Set(dto.r#type),
            icon: Set(dto.icon),
            hotkey: Set(dto.hotkey),
            hotkey_global: Set(dto.hotkey_global),
            pinyin_full: Set(dto.pinyin_full),
            pinyin_abbr: Set(dto.pinyin_abbr),
            keywords: Set(dto.keywords),
            start_dir: Set(dto.start_dir),
            remarks: Set(dto.remarks),
            args: Set(dto.args),
            run_as_admin: Set(dto.run_as_admin),
            order_index: Set(dto.order_index),
            enabled: Set(dto.enabled),
            category_id: Set(dto.category_id),
            subcategory_id: Set(dto.subcategory_id),
            extension: Set(dto.extension),
            ..Default::default()
        }
    }
}

impl Default for CreateLaunchItemDto {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            lnk_name: None,
            r#type: String::new(),
            icon: None,
            hotkey: None,
            hotkey_global: None,
            pinyin_full: None,
            pinyin_abbr: None,
            keywords: None,
            start_dir: None,
            remarks: None,
            args: None,
            run_as_admin: None,
            order_index: None,
            enabled: None,
            category_id: None,
            subcategory_id: None,
            extension: None,
        }
    }
}

impl CreateLaunchItemDto {
    pub fn into_active_model(self) -> launch_items::ActiveModel {
        let (pinyin_full, pinyin_abbr) = get_pinyin_variants(&self.name);

        launch_items::ActiveModel {
            name: Set(self.name),
            lnk_name: Set(self.lnk_name),
            path: Set(self.path),
            r#type: Set(self.r#type),
            icon: Set(self.icon),
            pinyin_full: Set(Some(pinyin_full)),
            pinyin_abbr: Set(Some(pinyin_abbr)),
            extension: Set(self.extension),
            hotkey: Set(self.hotkey),
            hotkey_global: Set(self.hotkey_global),
            keywords: Set(self.keywords),
            start_dir: Set(self.start_dir),
            remarks: Set(self.remarks),
            args: Set(self.args),
            run_as_admin: Set(self.run_as_admin),
            order_index: Set(self.order_index),
            enabled: Set(self.enabled),
            category_id: Set(self.category_id),
            subcategory_id: Set(self.subcategory_id),
            ..Default::default()
        }
    }
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

impl From<UpdateLaunchItemDto> for launch_items::ActiveModel {
    fn from(dto: UpdateLaunchItemDto) -> Self {
        Self {
            id: Set(dto.id),
            name: dto.name.map(Set).unwrap_or(NotSet),
            lnk_name: Set(dto.lnk_name),
            r#type: dto.r#type.map(Set).unwrap_or(NotSet),
            icon: Set(dto.icon),
            hotkey: Set(dto.hotkey),
            hotkey_global: Set(dto.hotkey_global),
            pinyin_full: Set(dto.pinyin_full),
            pinyin_abbr: Set(dto.pinyin_abbr),
            keywords: Set(dto.keywords),
            start_dir: Set(dto.start_dir),
            remarks: Set(dto.remarks),
            args: Set(dto.args),
            run_as_admin: Set(dto.run_as_admin),
            order_index: Set(dto.order_index),
            enabled: Set(dto.enabled),
            category_id: Set(dto.category_id),
            subcategory_id: Set(dto.subcategory_id),
            extension: Set(dto.extension),
            ..Default::default()
        }
    }
}

impl UpdateLaunchItemDto {
    pub fn into_active_model(
        self,
        existing: launch_items::Model,
        pinyin_full: String,
        pinyin_abbr: String,
    ) -> launch_items::ActiveModel {
        let mut active: launch_items::ActiveModel = existing.into();
        active.name = Set(self.name.unwrap_or_default());
        active.path = Set(self.path.unwrap_or_default());
        active.r#type = Set(self.r#type.unwrap_or_default());
        active.icon = Set(self.icon);
        active.pinyin_full = Set(Some(pinyin_full));
        active.pinyin_abbr = Set(Some(pinyin_abbr));
        active.extension = Set(self.extension);
        active.hotkey = Set(self.hotkey);
        active.hotkey_global = Set(self.hotkey_global);
        active.keywords = Set(self.keywords);
        active.start_dir = Set(self.start_dir);
        active.remarks = Set(self.remarks);
        active.args = Set(self.args);
        active.run_as_admin = Set(self.run_as_admin);
        active.order_index = Set(self.order_index);
        active.enabled = Set(self.enabled);
        active.category_id = Set(self.category_id);
        active.subcategory_id = self.subcategory_id.map(Set).unwrap_or(NotSet).into();
        active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sea_orm::FromQueryResult)]
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
