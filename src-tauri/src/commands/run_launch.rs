use crate::db;
use crate::models::launch_item::LaunchItem;
use rusqlite::{params, Result};
use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub fn run_launch(id: i32) -> Result<LaunchItem, String> {
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
    let mut args = vec![
        "/C".to_string(),
        "start".to_string(),
        "".to_string(),
        launch_item.path.to_string(),
    ];
    // 动态拼接执行参数
    if let Some(ref arg) = launch_item.args {
        args.push(arg.to_string());
    }

    if launch_item.r#type == "file" || launch_item.r#type == "directory" {
        // 如果是文件，直接打开
        // .creation_flags(0x00000008) 不会显示启动命令行窗口执行 但是无法使新运行的程序窗口聚焦

        if let Err(e) = Command::new("cmd")
            .creation_flags(0x08000000)
            .current_dir("C:\\Windows\\System32")
            .args(args)
            .spawn()
        // .arg(&launch_item.args)
        // .spawn()
        {
            return Err(format!("无法打开文件: {}", e));
        }
    } else if launch_item.r#type == "url" {
        return Err("未知的类型".to_string());
    }

    Ok(launch_item)
}
