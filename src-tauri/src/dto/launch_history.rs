use crate::entity::launch_history;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchHistoryDto {
    pub id: i32,
    pub launch_item_id: Option<i32>,
    pub command: String,
    pub r#type: String,
    pub started_at: Option<i64>,
}

impl From<launch_history::Model> for LaunchHistoryDto {
    fn from(model: launch_history::Model) -> Self {
        Self {
            id: model.id,
            launch_item_id: model.launch_item_id,
            command: model.command,
            r#type: model.r#type,
            started_at: model.started_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLaunchHistoryDto {
    pub launch_item_id: Option<i32>,
    pub command: String,
    pub r#type: String,
    pub started_at: Option<i64>,
}

impl From<CreateLaunchHistoryDto> for launch_history::ActiveModel {
    fn from(dto: CreateLaunchHistoryDto) -> Self {
        Self {
            launch_item_id: Set(dto.launch_item_id),
            command: Set(dto.command),
            r#type: Set(dto.r#type),
            started_at: dto.started_at.map(Set).unwrap_or(NotSet).into(),
            ..Default::default()
        }
    }
}
