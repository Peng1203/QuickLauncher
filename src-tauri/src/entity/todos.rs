use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    pub completed: bool,
    #[sea_orm(column_type = "Text")]
    pub priority: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub due_date: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub tags: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub note: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub reminder_at: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub created_at: String,
    #[sea_orm(column_type = "Text")]
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
