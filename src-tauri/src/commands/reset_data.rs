use crate::AppState;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn reset_data(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), ()> {
    // 先取出连接并释放锁
    let conn = {
        let mut db = state.db.lock().unwrap();
        db.take()
    };

    // 再异步关闭
    if let Some(conn) = conn {
        conn.close().await.map_err(|e| e.to_string()).unwrap();
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())
        .unwrap();
    let target_db_path = app_data_dir.join("Date.db");

    // 删除 数据库文件
    if target_db_path.exists() {
        std::fs::remove_file(&target_db_path).map_err(|_| ())?;
    }

    // 删除 store 文件
    // app.pinia();

    app.restart();
}
