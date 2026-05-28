use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

#[tracing::instrument]
#[tauri::command]
pub fn open_dir_in_terminal(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);

    if !path.exists() {
        log::error!("路径不存在 {}", path.to_string_lossy());
        return Err("路径不存在".into());
    }

    let path_str = path.to_string_lossy();

    let _ = Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", "start", "cmd", "/K", &format!("cd /d {}", path_str)])
        .spawn();

    Ok(())
}
