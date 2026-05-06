use crate::common::utils::is_valid_url;
use std::{os::windows::process::CommandExt, path::Path, process::Command};

#[tauri::command]
pub async fn exe_command(command: String) -> Result<(), String> {
    exec_command_internal(&command)
}
pub fn exec_command_internal(command: &str) -> Result<(), String> {
    let cmd = command.trim();

    // 本地路径
    if Path::new(cmd).exists() {
        Command::new("cmd")
            .creation_flags(0x08000000)
            .args(["/C", "start", "", cmd])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    // URL
    if is_valid_url(cmd) {
        let url = if cmd.starts_with("http://") || cmd.starts_with("https://") {
            cmd.to_string()
        } else {
            format!("https://{}", cmd)
        };

        Command::new("cmd")
            .creation_flags(0x08000000)
            .args(["/C", "start", "", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    // 普通命令
    let mut parts = cmd.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();

        Command::new("cmd")
            .creation_flags(0x08000000)
            .current_dir("C:\\Windows\\System32")
            .args(["/C", "start", "", program])
            .args(&args)
            .spawn()
            .map_err(|e| e.to_string())?;

        return Ok(());
    }

    Err("Invalid command".into())
}
