use chrono::Utc;
use sea_orm::prelude::*;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
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

    #[sea_orm(nullable)]
    pub due_date: Option<i64>,

    #[sea_orm(column_type = "Text", nullable)]
    pub tags: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub note: Option<String>,

    #[sea_orm(nullable)]
    pub reminder_at: Option<i64>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now().timestamp_millis();
        if insert {
            self.created_at = Set(now);
        }
        self.updated_at = Set(now);
        Ok(self)
    }
}
