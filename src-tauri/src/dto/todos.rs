use crate::entity::todos;
use sea_orm::ActiveValue::{NotSet, Set};
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
    pub reminder_at: Option<i64>,
}

impl From<UpdateTodoDto> for todos::ActiveModel {
    fn from(dto: UpdateTodoDto) -> Self {
        Self {
            id: Set(dto.id),
            title: dto.title.map(Set).unwrap_or(NotSet),
            completed: dto.completed.map(Set).unwrap_or(NotSet),
            priority: dto.priority.map(Set).unwrap_or(NotSet),
            due_date: dto.due_date.map(Set).unwrap_or(NotSet).into(),
            tags: dto.tags.map(Set).unwrap_or(NotSet).into(),
            note: dto.note.map(Set).unwrap_or(NotSet).into(),
            reminder_at: dto.reminder_at.map(Set).unwrap_or(NotSet).into(),
            ..Default::default()
        }
    }
}
