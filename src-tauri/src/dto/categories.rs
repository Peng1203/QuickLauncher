use crate::{common::utils::get_pinyin_variants, entity::categories};
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub association_directory: Option<String>,
    pub exclude: bool,
    pub layout: String,
    pub sort_by: String,
    pub sort_order: String,
    pub order_index: Option<i32>,
    pub icon: Option<String>,
    pub pinyin_full: Option<String>,
    pub pinyin_abbr: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<categories::Model> for CategoryDto {
    fn from(model: categories::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            parent_id: model.parent_id,
            association_directory: model.association_directory,
            exclude: model.exclude,
            layout: model.layout,
            sort_by: model.sort_by,
            sort_order: model.sort_order,
            order_index: model.order_index,
            icon: model.icon,
            pinyin_full: model.pinyin_full,
            pinyin_abbr: model.pinyin_abbr,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub parent_id: Option<i32>,
    pub association_directory: Option<String>,
    pub exclude: bool,
    pub layout: String,
    pub sort_by: String,
    pub sort_order: String,
    pub order_index: Option<i32>,
    pub icon: Option<String>,
    pub pinyin_full: Option<String>,
    pub pinyin_abbr: Option<String>,
}

impl From<CreateCategoryDto> for categories::ActiveModel {
    fn from(dto: CreateCategoryDto) -> Self {
        Self {
            name: Set(dto.name),
            parent_id: Set(dto.parent_id),
            association_directory: Set(dto.association_directory),
            exclude: Set(dto.exclude),
            layout: Set(dto.layout),
            sort_by: Set(dto.sort_by),
            sort_order: Set(dto.sort_order),
            order_index: Set(dto.order_index),
            icon: Set(dto.icon),
            pinyin_full: Set(dto.pinyin_full),
            pinyin_abbr: Set(dto.pinyin_abbr),
            ..Default::default() // id, created_at, updated_at 由 before_save 处理
        }
    }
}

impl Default for CreateCategoryDto {
    fn default() -> Self {
        Self {
            name: String::new(),
            parent_id: None,
            association_directory: None,
            exclude: false,
            layout: "grid".to_string(),
            sort_by: "default".to_string(),
            sort_order: "default".to_string(),
            order_index: None,
            icon: None,
            pinyin_full: None,
            pinyin_abbr: None,
        }
    }
}

impl CreateCategoryDto {
    pub fn into_active_model(self) -> categories::ActiveModel {
        let (pinyin_full, pinyin_abbr) = get_pinyin_variants(&self.name);

        categories::ActiveModel {
            name: Set(self.name),
            parent_id: Set(self.parent_id),
            association_directory: Set(self.association_directory),
            exclude: Set(self.exclude),
            layout: Set(self.layout),
            sort_by: Set(self.sort_by),
            sort_order: Set(self.sort_order),
            order_index: Set(self.order_index),
            icon: Set(self.icon),
            pinyin_full: Set(Some(pinyin_full)),
            pinyin_abbr: Set(Some(pinyin_abbr)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategoryDto {
    pub id: i32,
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub association_directory: Option<String>,
    pub exclude: Option<bool>,
    pub layout: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub order_index: Option<i32>,
    pub icon: Option<String>,
    pub pinyin_full: Option<String>,
    pub pinyin_abbr: Option<String>,
}

impl From<UpdateCategoryDto> for categories::ActiveModel {
    fn from(dto: UpdateCategoryDto) -> Self {
        Self {
            id: Set(dto.id),
            name: dto.name.map(Set).unwrap_or(NotSet),
            parent_id: dto.parent_id.map(|v| Set(Some(v))).unwrap_or(NotSet),
            association_directory: dto
                .association_directory
                .map(|v| Set(Some(v)))
                .unwrap_or(NotSet),
            exclude: dto.exclude.map(Set).unwrap_or(NotSet),
            layout: dto.layout.map(Set).unwrap_or(NotSet),
            sort_by: dto.sort_by.map(Set).unwrap_or(NotSet),
            sort_order: dto.sort_order.map(Set).unwrap_or(NotSet),
            order_index: dto.order_index.map(|v| Set(Some(v))).unwrap_or(NotSet),
            icon: dto.icon.map(|v| Set(Some(v))).unwrap_or(NotSet),
            pinyin_full: dto.pinyin_full.map(|v| Set(Some(v))).unwrap_or(NotSet),
            pinyin_abbr: dto.pinyin_abbr.map(|v| Set(Some(v))).unwrap_or(NotSet),
            ..Default::default()
        }
    }
}

impl UpdateCategoryDto {
    pub fn into_active_model(self) -> categories::ActiveModel {
        let (pinyin_full, pinyin_abbr) = self
            .name
            .as_ref()
            .map(|name| {
                let (full, abbr) = get_pinyin_variants(name);
                (Set(Some(full)), Set(Some(abbr)))
            })
            .unwrap_or((NotSet, NotSet));

        categories::ActiveModel {
            id: Set(self.id),
            name: self.name.map(Set).unwrap_or(NotSet),
            parent_id: self.parent_id.map(|v| Set(Some(v))).unwrap_or(NotSet),
            association_directory: self
                .association_directory
                .map(|v| Set(Some(v)))
                .unwrap_or(NotSet),
            exclude: self.exclude.map(Set).unwrap_or(NotSet),
            layout: self.layout.map(Set).unwrap_or(NotSet),
            sort_by: self.sort_by.map(Set).unwrap_or(NotSet),
            sort_order: self.sort_order.map(Set).unwrap_or(NotSet),
            order_index: self.order_index.map(|v| Set(Some(v))).unwrap_or(NotSet),
            icon: self.icon.map(|v| Set(Some(v))).unwrap_or(NotSet),
            pinyin_full,
            pinyin_abbr,
            ..Default::default()
        }
    }
}
