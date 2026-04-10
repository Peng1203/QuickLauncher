use crate::{common::utils::run_as_admin, entity};
use entity::launch_items::{Entity as LaunchItems, Model};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub async fn run_launch(id: i32, db: tauri::State<'_, DatabaseConnection>) -> Result<(), String> {
    // ORM 查询
    let launch_item: Model = LaunchItems::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询启动项失败: {}", e))?
        .ok_or("启动项不存在")?;

    log::info!("id:{}, path:{}", id, launch_item.path);

    let mut args;
    dbg!(&launch_item);
    if launch_item.r#type == "file" {
        let start_dir = launch_item
            .start_dir
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("C:\\Windows\\System32");

        if launch_item.run_as_admin == Some(false) {
            args = vec![
                "/C".to_string(),
                "start".to_string(),
                "".to_string(),
                launch_item.path.clone(),
            ];

            if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
                arg.split_whitespace()
                    .for_each(|a| args.push(a.to_string()));
            }

            if let Err(e) = Command::new("cmd")
                .creation_flags(0x08000000)
                .current_dir(start_dir)
                .args(args)
                .spawn()
            {
                log::error!("无法打开文件: {}", e);
                return Err(format!("无法打开文件: {}", e));
            }
        } else {
            run_as_admin(launch_item.clone()).map_err(|e| format!("提权执行失败: {}", e))?;
        }
    } else if launch_item.r#type == "directory" {
        args = vec![
            "/C".to_string(),
            "start".to_string(),
            "".to_string(),
            launch_item.path.clone(),
        ];

        if let Err(e) = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(args)
            .spawn()
        {
            log::error!("无法打开目录: {}", e);
            return Err(format!("无法打开目录: {}", e));
        }
    } else if launch_item.r#type == "url" {
        let mut url_args = vec!["/C".to_string(), "start".to_string(), "".to_string()];

        if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
            url_args.push(arg.clone());
        }

        url_args.push(launch_item.path.clone());

        if let Err(e) = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(url_args)
            .spawn()
        {
            log::error!("无法打开URL: {}", e);
            return Err(format!("无法打开URL: {}", e));
        }
    }

    Ok(())
}
