use crate::db;
use crate::models::launch_item::LaunchItem;
use rusqlite::{params, Result};
use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub fn run_launch_as_admin(id: i32) -> Result<LaunchItem, String> {
    let conn = db::connection::get_conn().lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM launch_items WHERE id = ?")
        .map_err(|e| format!("准备查询语句失败：{}", e))
        .unwrap();

    let row = stmt.query_row(params![id], LaunchItem::from_row);

    let launch_item = row.unwrap();
    // println!("名称：{}", launch_item);
    log::info!("id:{}, path:{}", id, launch_item.path);

    // dbg!(&launch_item);
    let mut powershell_cmd = vec![
        "Start-Process".to_string(),
        "-FilePath".to_string(),
        format!("'{}'", launch_item.path),
        "-Verb".to_string(),
        "RunAs".to_string(),
    ];

    // 如果有参数，则添加到 `-ArgumentList`
    if let Some(ref arg) = launch_item.args {
        powershell_cmd.push("-ArgumentList".to_string());
        powershell_cmd.push(format!("'{}'", arg));
    }

    if launch_item.r#type == "file" || launch_item.r#type == "directory" {
        if let Err(e) = Command::new("powershell")
            .creation_flags(0x08000000)
            .current_dir("C:\\Windows\\System32")
            .arg("-Command")
            .args(&powershell_cmd)
            .spawn()
        {
            log::error!("❌ 无法以管理员权限运行启动项: {}", e);
            return Err(format!("❌ 无法以管理员权限运行启动项: {}", e));
        }
    } else if launch_item.r#type == "url" {
        log::error!("⚠️ 暂不支持 URL 类型的启动项以管理员方式运行");
        return Err("⚠️ 暂不支持 URL 类型的启动项以管理员方式运行".into());
    }

    Ok(launch_item)
}
