// use std::{fs, path::Path};

// use tauri::{command, AppHandle, Manager};

// #[command]
// pub fn import_database(import_path: String, app: AppHandle) -> Result<String, String> {
//     // 获取目标数据库路径
//     let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
//     let target_db_path = app_data_dir.join("Date.db");

//     // 验证导入文件是否存在
//     let import_file_path = Path::new(&import_path);
//     if !import_file_path.exists() {
//         return Err(format!("Import file not found: {}", import_path));
//     }

//     // 验证文件扩展名
//     if import_file_path.extension().and_then(|s| s.to_str()) != Some("db") {
//         return Err("Only .db files are supported".to_string());
//     }

//     // 确保目标目录存在
//     if let Some(parent) = target_db_path.parent() {
//         fs::create_dir_all(parent)
//             .map_err(|e| format!("Failed to create app data directory: {}", e))?;
//     }

//     // 如果目标文件存在，先删除
//     if target_db_path.exists() {
//         fs::remove_file(&target_db_path)
//             .map_err(|e| format!("Failed to remove existing database: {}", e))?;
//     }

//     // 复制导入文件到目标位置
//     fs::copy(import_file_path, &target_db_path)
//         .map_err(|e| format!("Failed to import database: {}", e))?;

//     Ok(format!(
//         "✓ Database imported successfully from: {}",
//         import_path
//     ))
// }

use std::{fs, path::Path};
use tauri::{command, AppHandle, Manager};

use crate::AppState;

#[command]
pub async fn import_database(
    import_path: String,
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // 关闭数据库连接
    // 先取出连接并释放锁
    let conn = {
        let mut db = state.db.lock().unwrap();
        db.take()
    };

    // 再异步关闭
    if let Some(conn) = conn {
        conn.close().await.map_err(|e| e.to_string())?;
    }

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let target_db_path = app_data_dir.join("Date.db");

    let import_file_path = Path::new(&import_path);
    if !import_file_path.exists() {
        return Err(format!("Import file not found: {}", import_path));
    }

    if import_file_path.extension().and_then(|s| s.to_str()) != Some("db") {
        return Err("Only .db files are supported".to_string());
    }

    if let Some(parent) = target_db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    // 如果目标文件存在，先重命名为 .old
    if target_db_path.exists() {
        let old_path = target_db_path.with_extension("db.old");

        // 如果 .old 文件存在，先删除
        if old_path.exists() {
            let _ = fs::remove_file(&old_path);
        }

        // 重命名当前文件
        fs::rename(&target_db_path, &old_path)
            .map_err(|e| format!("Failed to backup existing database: {}", e))?;
    }

    // 复制导入文件
    fs::copy(import_file_path, &target_db_path)
        .map_err(|e| format!("Failed to import database: {}", e))?;

    // Ok(format!(
    //     "✓ Database imported successfully\nLocation: {}",
    //     target_db_path.display()
    // ))

    // 重启app
    app.restart();
}
