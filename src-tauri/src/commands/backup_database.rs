use std::path::Path;
use std::{fs, time::SystemTime};
use tauri::{command, AppHandle, Manager};

#[command]
pub fn backup_database(backup_path: String, app: AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("Date.db");

    if !db_path.exists() {
        return Err(format!("Database file not found: {:?}", db_path));
    }

    let backup_dir = Path::new(&backup_path);

    // 生成简单的时间戳（使用系统时间）
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    // 拼接备份文件路径
    let backup_file_path = backup_dir.join(format!("{}_{}", timestamp, "Date.db"));
    // let backup_file_path = Path::new(&backup_path);

    // 确保备份目录存在
    if let Some(parent) = &backup_file_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    }

    // 如果备份文件已存在，先删除
    if backup_file_path.exists() {
        fs::remove_file(&backup_file_path)
            .map_err(|e| format!("Failed to remove existing backup file: {}", e))?;
    }

    // 复制文件
    fs::copy(&db_path, backup_file_path)
        .map_err(|e| format!("Failed to backup database: {}", e))?;

    Ok(format!(
        "Database backed up successfully to: {}",
        backup_path
    ))
}
