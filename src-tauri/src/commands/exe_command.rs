use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub async fn exe_command(cmd: String) {
    let mut parts = cmd.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();

        Command::new("cmd")
            .creation_flags(0x08000000)
            .current_dir("C:\\Windows\\System32")
            .args(["/C", "start", "", program])
            .args(&args) // 动态拼接剩余参数
            .spawn()
            .expect("failed to spawn process");
    }
}
