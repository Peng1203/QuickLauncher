use crate::{entity, AppState};
use entity::todos::Entity as Todos;
use entity::todos::Model as TodoModel;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub async fn update_todo(
    id: i32,
    title: Option<String>,
    completed: Option<bool>,
    priority: Option<String>,
    due_date: Option<Option<String>>,
    tags: Option<Option<String>>,
    note: Option<Option<String>>,
    reminder_at: Option<Option<String>>,
    state: tauri::State<'_, AppState>,
) -> Result<TodoModel, String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    let todo = Todos::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("待办事项不存在")?;

    let now = chrono_now();
    let mut active: entity::todos::ActiveModel = todo.into();

    if let Some(v) = title {
        active.title = Set(v);
    }
    if let Some(v) = completed {
        active.completed = Set(v);
    }
    if let Some(v) = priority {
        active.priority = Set(v);
    }
    if let Some(v) = due_date {
        active.due_date = Set(v);
    }
    if let Some(v) = tags {
        active.tags = Set(v);
    }
    if let Some(v) = note {
        active.note = Set(v);
    }
    if let Some(v) = reminder_at {
        active.reminder_at = Set(v);
    }
    active.updated_at = Set(now);

    let result = active.update(&db).await.map_err(|e| {
        tracing::error!(%e, "更新待办事项失败");
        format!("更新待办事项失败：{}", e)
    })?;

    tracing::info!(id = result.id, "更新待办事项");
    Ok(result)
}

fn chrono_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let secs = now.as_secs();
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let (y, m, d) = days_to_ymd(days);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        y, m, d, hours, minutes, seconds
    )
}

fn days_to_ymd(mut days: u64) -> (i32, u32, u32) {
    let mut y = 1970;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        y += 1;
    }
    let leap = is_leap(y);
    let month_days: [u64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 1;
    for &md in &month_days {
        if days < md {
            break;
        }
        days -= md;
        m += 1;
    }
    (y, m, days as u32 + 1)
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
