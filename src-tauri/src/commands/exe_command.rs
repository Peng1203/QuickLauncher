use crate::common::utils::is_valid_url;
use std::{os::windows::process::CommandExt, path::Path, process::Command};

#[tauri::command]
pub async fn exe_command(cmd: String) {
    dbg!(&cmd);
    let cmd = cmd.trim();

    // 判断是否是一个本地路径
    if Path::new(cmd).exists() {
        // 直接打开路径（文件 / 目录）
        Command::new("cmd")
            .creation_flags(0x08000000)
            .args(["/C", "start", "", cmd])
            .spawn()
            .expect("failed to open path");
        return;
    }

    // 判断是否是网址
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
            .expect("failed to open url");

        return;
    }

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

// #[tauri::command]
// pub async fn exe_command(cmd: String) {
//     Command::new("cmd")
//         .creation_flags(0x08000000)
//         .args(["/C", "start", "", &cmd])
//         .spawn()
//         .expect("failed to spawn process");
// }

// use std::{os::windows::process::CommandExt, process::Command};

// #[tauri::command]
// pub async fn exe_command(cmd: String) -> Result<String, String> {
//     let mut parts = cmd.split_whitespace();

//     if let Some(program) = parts.next() {
//         let args: Vec<&str> = parts.collect();

//         // 使用 output() 而不是 spawn() 来捕获执行结果
//         match Command::new("cmd")
//             .creation_flags(0x08000000)
//             .current_dir("C:\\Windows\\System32")
//             .args(["/C", "start", "/wait", "", program])  // 添加 /wait 等待程序结束
//             .args(&args)
//             .output()  // 改为 output() 获取执行结果
//         {
//             Ok(output) => {
//                 if output.status.success() {
//                     let stdout = String::from_utf8_lossy(&output.stdout);
//                     let stderr = String::from_utf8_lossy(&output.stderr);

//                     if !stderr.is_empty() {
//                         Ok(format!("程序执行完成(有警告):\n标准输出: {}\n错误输出: {}", stdout, stderr))
//                     } else {
//                         Ok(format!("程序执行成功:\n{}", stdout))
//                     }
//                 } else {
//                     let stderr = String::from_utf8_lossy(&output.stderr);
//                     Err(format!("程序执行失败(退出码: {:?}):\n{}", output.status.code(), stderr))
//                 }
//             },
//             Err(e) => Err(format!("启动程序失败: {} - 错误: {}", program, e)),
//         }
//     } else {
//         Err("命令为空或格式不正确".to_string())
//     }
// }
