#[tauri::command]
pub async fn set_default_tray_icon(app: tauri::AppHandle) {
    let _ = app
        .tray_by_id("tray")
        .unwrap()
        .set_icon(Some(app.default_window_icon().unwrap().clone()));
}
