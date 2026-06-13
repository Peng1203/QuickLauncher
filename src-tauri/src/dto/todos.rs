use crate::entity::todos;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub priority: i32,
    pub due_date: Option<i64>,
    pub tags: Option<String>,
    pub note: Option<String>,
    pub reminder_at: Option<i64>,
    pub order_index: Option<i32>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<todos::Model> for TodoDto {
    fn from(model: todos::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            completed: model.completed,
            priority: model.priority,
            due_date: model.due_date,
            tags: model.tags,
            note: model.note,
            order_index: model.order_index,
            reminder_at: model.reminder_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    pub priority: i32,
    pub due_date: Option<i64>,
    pub tags: Option<String>,
    pub note: Option<String>,
    pub order_index: Option<i32>,
    pub reminder_at: Option<i64>,
}

impl From<CreateTodoDto> for todos::ActiveModel {
    fn from(dto: CreateTodoDto) -> Self {
        Self {
            title: Set(dto.title),
            completed: Set(false),
            priority: Set(dto.priority),
            due_date: Set(dto.due_date),
            tags: Set(dto.tags),
            note: Set(dto.note),
            order_index: Set(dto.order_index),
            reminder_at: Set(dto.reminder_at),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoDto {
    pub id: i32,
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<i32>,
    pub due_date: Option<i64>,
    pub tags: Option<String>,
    pub note: Option<String>,
    pub order_index: Option<i32>,
    pub reminder_at: Option<i64>,
}

impl UpdateTodoDto {
    pub fn into_active_model(self, existing: todos::Model) -> todos::ActiveModel {
        let mut active: todos::ActiveModel = existing.into();
        if let Some(v) = self.title {
            active.title = Set(v);
        }
        if let Some(v) = self.completed {
            active.completed = Set(v);
        }
        if let Some(v) = self.priority {
            active.priority = Set(v);
        }
        if let Some(v) = self.due_date {
            active.due_date = Set(Some(v));
        }
        if let Some(v) = self.tags {
            active.tags = Set(Some(v));
        }
        if let Some(v) = self.note {
            active.note = Set(Some(v));
        }
        if let Some(v) = self.reminder_at {
            active.reminder_at = Set(Some(v));
        }
        if let Some(v) = self.order_index {
            active.order_index = Set(Some(v));
        }
        active
    }
}
