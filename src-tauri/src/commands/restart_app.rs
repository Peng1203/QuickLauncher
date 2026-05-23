use tracing;

#[tracing::instrument]
#[tauri::command]
pub fn restart_app(app: tauri::AppHandle) {
    tracing::info!("重启应用");
    app.restart();
}
