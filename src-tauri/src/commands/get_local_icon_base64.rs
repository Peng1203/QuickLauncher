use windows_icons::get_icon_base64_by_path;

#[tauri::command]
pub fn get_local_icon_base64(path: String) -> Result<String, String> {
    // 使用 map_err 将错误转换为 String 类型
    let base64 =
        get_icon_base64_by_path(&path).map_err(|e| format!("Failed to get icon: {}", e))?;

    let icon = format!("data:image/png;base64,{}", base64);

    Ok(icon)
}
