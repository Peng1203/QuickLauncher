use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub priority: i32,
    pub due_date: Option<i64>,
    pub tags: Option<String>,
    pub note: Option<String>,
    pub reminder_at: Option<i64>,
}

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
