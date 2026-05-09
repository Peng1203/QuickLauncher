use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn open_app_data_dir(app: AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    // 将 PathBuf 转换为 String
    let path_str = app_data_dir.to_string_lossy().to_string();
    // 使用 OpenerExt 的 open 方法打开目录
    app.opener()
        .open_path(path_str, None::<&str>)
        .map_err(|e| e.to_string())?;

    Ok(())
}
