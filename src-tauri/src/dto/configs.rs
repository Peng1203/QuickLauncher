use crate::entity::configs;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDto {
    pub id: i32,
    pub name: String,
    pub data: String,
}

impl From<configs::Model> for ConfigDto {
    fn from(model: configs::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            data: model.data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigDto {
    pub name: String,
    pub data: String,
}

impl From<UpdateConfigDto> for configs::ActiveModel {
    fn from(dto: UpdateConfigDto) -> Self {
        Self {
            name: Set(dto.name.clone()),
            data: Set(dto.data.clone()),
            ..Default::default()
        }
    }
}
