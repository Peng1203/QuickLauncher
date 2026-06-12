use crate::entity::autocomplete_history;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteHistoryDto {
    pub id: i32,
    pub query: String,
    pub usage_count: Option<i32>,
    pub launch_item_id: Option<i32>,
    pub last_used_at: i64,
}

impl From<autocomplete_history::Model> for AutocompleteHistoryDto {
    fn from(model: autocomplete_history::Model) -> Self {
        Self {
            id: model.id,
            query: model.query,
            usage_count: model.usage_count,
            launch_item_id: model.launch_item_id,
            last_used_at: model.last_used_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutocompleteHistoryDto {
    pub query: String,
    pub usage_count: Option<i32>,
    pub launch_item_id: Option<i32>,
    pub last_used_at: i64,
}

impl From<CreateAutocompleteHistoryDto> for autocomplete_history::ActiveModel {
    fn from(dto: CreateAutocompleteHistoryDto) -> Self {
        Self {
            query: Set(dto.query),
            usage_count: Set(dto.usage_count),
            launch_item_id: Set(dto.launch_item_id),
            last_used_at: Set(dto.last_used_at),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutocompleteHistoryDto {
    pub id: i32,
    pub query: Option<String>,
    pub usage_count: Option<i32>,
    pub launch_item_id: Option<i32>,
    pub last_used_at: Option<i64>,
}

impl From<UpdateAutocompleteHistoryDto> for autocomplete_history::ActiveModel {
    fn from(dto: UpdateAutocompleteHistoryDto) -> Self {
        Self {
            id: Set(dto.id),
            query: dto.query.map(Set).unwrap_or(NotSet),
            usage_count: dto.usage_count.map(Set).unwrap_or(NotSet).into(),
            launch_item_id: dto.launch_item_id.map(Set).unwrap_or(NotSet).into(),
            last_used_at: dto.last_used_at.map(Set).unwrap_or(NotSet),
        }
    }
}
