use crate::entity::configs;
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
