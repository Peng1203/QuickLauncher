use crate::db;
use crate::models::launch_item::LaunchItem;
use rusqlite::{params, Result};
use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub fn run_launch(id: i32) -> Result<(), String> {
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
    let mut args = vec![];

    if launch_item.r#type == "file" {
        println!("run_as_admin:{}", &launch_item.run_as_admin);
        let start_dir = launch_item
            .start_dir
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("C:\\Windows\\System32");
        if launch_item.run_as_admin == 0 {
            // 动态拼接执行参数
            if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
                args.push(arg.clone());
            }

            if let Err(e) = Command::new(launch_item.path.clone())
                .creation_flags(0x08000000)
                .current_dir(start_dir)
                .args(args)
                .spawn()
            {
                log::error!("无法打开文件: {}", e);
                return Err(format!("无法打开文件: {}", e));
            }
        } else {
            // Start-Process -FilePath C:\Application\QQMusic\QQMusic.exe -Verb RunAs -ArgumentList --12
            // 以管理员权限运行
            let mut powershell_cmd = vec![
                "Start-Process".to_string(),
                "-FilePath".to_string(),
                format!("'{}'", launch_item.path),
                // launch_item.path.to_string(),
                "-Verb".to_string(),
                "RunAs".to_string(),
            ];

            // 如果有参数，则添加到 `-ArgumentList`
            if let Some(ref arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
                powershell_cmd.push("-ArgumentList".to_string());
                powershell_cmd.push(format!("'{}'", arg));
            }

            if let Err(e) = Command::new("powershell")
                .creation_flags(0x08000000)
                .current_dir(start_dir)
                .args(&powershell_cmd)
                .spawn()
            {
                log::error!("❌ 无法以管理员权限运行启动项: {}", e);
                return Err(format!("❌ 无法以管理员权限运行启动项: {}", e));
            }
        }
    } else if launch_item.r#type == "directory" {
        args = vec![
            "/C".to_string(),
            "start".to_string(),
            "".to_string(),
            launch_item.path.to_string(),
        ];
        if let Err(e) = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(args)
            .spawn()
        // .arg(&launch_item.args)
        // .spawn()
        {
            log::error!("无法打开目录: {}", e);
            return Err(format!("无法打开目录: {}", e));
        }
    } else if launch_item.r#type == "url" {
        // 处理URL类型，使用默认浏览器打开
        // TODO 可通过参数指定浏览器
        let mut url_args = vec!["/C".to_string(), "start".to_string(), "".to_string()];
        // "".to_string(),

        // 类型为URL时，args存放浏览器参数
        if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
            url_args.push(arg.clone());
        }

        url_args.push(launch_item.path.to_string());

        dbg!(&url_args);

        // .current_dir("C:\\Windows\\System32")
        if let Err(e) = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(url_args)
            .spawn()
        {
            log::error!("无法打开URL: {}", e);
            return Err(format!("无法打开URL: {}", e));
        }
    }

    // TODO 记录启动次数和最后启动时间

    Ok(())
}
