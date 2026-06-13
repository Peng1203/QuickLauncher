use crate::entity::todos;
use crate::entity::todos::Entity as Todos;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use tauri::AppHandle;
use tauri::Emitter;
use tauri_plugin_notification::NotificationExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct ReminderScheduler {
    tasks: Arc<Mutex<HashMap<i32, JoinHandle<()>>>>,
}

impl ReminderScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 调度单个 todo 的提醒
    pub async fn schedule(
        &self,
        app: AppHandle,
        db: sea_orm::DatabaseConnection,
        todo_id: i32,
        reminder_at: i64,
    ) {
        let now = chrono::Utc::now().timestamp_millis();
        let delay = reminder_at - now;

        if delay <= 0 {
            return;
        }

        // 先取消旧的
        self.cancel(todo_id).await;

        let tasks = self.tasks.clone();
        let app_clone = app.clone();

        let handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;

            // 查询 todo 标题
            let title = Todos::find_by_id(todo_id)
                .one(&db)
                .await
                .ok()
                .flatten()
                .map(|t| t.title)
                .unwrap_or_else(|| "未知任务".to_string());

            // 发送系统通知
            let _ = app_clone
                .notification()
                .builder()
                .title("QuickLauncher - 待办提醒")
                .body(format!("\"{}\" 的提醒时间已到", title))
                .show();

            // 发送事件给前端
            let _ = app_clone.emit("todo-reminder", todo_id);

            // 任务完成后从 map 中移除
            tasks.lock().await.remove(&todo_id);
        });

        self.tasks.lock().await.insert(todo_id, handle);
    }

    /// 取消某个 todo 的提醒
    pub async fn cancel(&self, todo_id: i32) {
        if let Some(handle) = self.tasks.lock().await.remove(&todo_id) {
            handle.abort();
        }
    }

    /// 从数据库加载并调度所有未过期的提醒
    pub async fn load_all(&self, app: AppHandle, db: sea_orm::DatabaseConnection) {
        let now = chrono::Utc::now().timestamp_millis();

        let todos = Todos::find()
            .filter(todos::Column::ReminderAt.is_not_null())
            .filter(todos::Column::ReminderAt.gte(now))
            .filter(todos::Column::Completed.eq(false))
            .all(&db)
            .await
            .unwrap_or_default();

        for todo in todos {
            if let Some(reminder_at) = todo.reminder_at {
                self.schedule(app.clone(), db.clone(), todo.id, reminder_at)
                    .await;
            }
        }
    }

    /// 清除所有任务
    pub async fn clear_all(&self) {
        let mut tasks = self.tasks.lock().await;
        for (_, handle) in tasks.drain() {
            handle.abort();
        }
    }
}

// 全局调度器实例
static SCHEDULER: OnceLock<Arc<ReminderScheduler>> = OnceLock::new();

pub fn get_scheduler() -> &'static Arc<ReminderScheduler> {
    SCHEDULER.get_or_init(|| Arc::new(ReminderScheduler::new()))
}

/// 初始化提醒调度器（在 app setup 中调用）
pub async fn init_reminder_scheduler(app: AppHandle, db: sea_orm::DatabaseConnection) {
    let scheduler = get_scheduler();
    scheduler.load_all(app, db).await;
}
