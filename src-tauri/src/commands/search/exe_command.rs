use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub async fn exe_command(cmd: String) {
    // let result =
    Command::new("cmd")
        .creation_flags(0x00000008)
        .current_dir("C:\\Windows\\System32")
        .args(["/C", "start", "", &cmd])
        .spawn()
        .expect("0");
}
