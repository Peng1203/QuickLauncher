use crate::{commands::exe_command::exec_command_internal, common::utils::run_as_admin, entity};
use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{
    prelude::Expr, sea_query::prelude::Local, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    ExprTrait, QueryFilter,
};
use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
pub async fn run_launch(id: i32, db: tauri::State<'_, DatabaseConnection>) -> Result<(), String> {
    // ORM 查询
    let launch_item: Model = LaunchItems::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询启动项失败: {}", e))?
        .ok_or("启动项不存在")?;

    if launch_item.r#type == "file" {
        let _ = run_file(&launch_item).await;
    } else if launch_item.r#type == "directory" {
        let _ = run_directory(&launch_item).await;
    } else if launch_item.r#type == "url" {
        let _ = run_url(&launch_item).await;
    } else if launch_item.r#type == "alias" {
        let _ = run_alias(&launch_item).await;
    } else if launch_item.r#type == "apps" {
        // let _ = run_apps(&launch_item, &db).await;

        let db = db.inner().clone();
        let launch_item = launch_item.clone();

        tokio::spawn(async move {
            if let Err(e) = run_apps(&launch_item, &db).await {
                log::error!("执行 apps 失败: {}", e);
            }
        });
    }

    // 更新启动项目的启动次数和最后使用时间
    incr_launch_count(db.inner(), id)
        .await
        .map_err(|e| format!("更新启动次数失败: {}", e))?;

    // 添加历史记录
    Ok(())
}

async fn run_file(launch_item: &Model) -> Result<(), String> {
    let start_dir = launch_item
        .start_dir
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("C:\\Windows\\System32");

    let mut args = vec![
        "/C".to_string(),
        "start".to_string(),
        "".to_string(),
        launch_item.path.clone(),
    ];

    if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
        arg.split_whitespace()
            .for_each(|a| args.push(a.to_string()));
    }

    if launch_item.run_as_admin == Some(false) {
        Command::new("cmd")
            .creation_flags(0x08000000)
            .current_dir(start_dir)
            .args(args)
            .spawn()
            .map_err(|e| format!("无法打开文件: {}", e))?;
    } else {
        run_as_admin(launch_item.clone()).map_err(|e| format!("提权执行失败: {}", e))?;
    }

    Ok(())
}

async fn run_directory(launch_item: &Model) -> Result<(), String> {
    let args = vec![
        "/C".to_string(),
        "start".to_string(),
        "".to_string(),
        launch_item.path.clone(),
    ];

    Command::new("cmd")
        .creation_flags(0x08000000)
        .args(args)
        .spawn()
        .map_err(|e| format!("无法打开目录: {}", e))?;

    Ok(())
}

async fn run_url(launch_item: &Model) -> Result<(), String> {
    let mut args = vec!["/C".to_string(), "start".to_string(), "".to_string()];

    if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
        args.push(arg.clone());
    }

    args.push(launch_item.path.clone());

    Command::new("cmd")
        .creation_flags(0x08000000)
        .args(args)
        .spawn()
        .map_err(|e| format!("无法打开URL: {}", e))?;

    Ok(())
}

async fn run_alias(launch_item: &Model) -> Result<(), String> {
    exec_command_internal(&launch_item.path).map_err(|e| format!("执行命令失败: {}", e))?;

    Ok(())
}

use tokio::time::{sleep, Duration};
async fn run_apps(launch_item: &Model, db: &DatabaseConnection) -> Result<(), String> {
    let delay_ms = launch_item
        .args
        .as_deref()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0);

    let ids: Vec<i32> = launch_item
        .path
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    for (index, id) in ids.iter().enumerate() {
        let sub_item = LaunchItems::find_by_id(*id)
            .one(db)
            .await
            .map_err(|e| format!("查询子任务失败: {}", e))?
            .ok_or(format!("子任务不存在: {}", id))?;

        if sub_item.id == launch_item.id {
            continue;
        }

        if sub_item.r#type == "file" {
            let _ = run_file(&sub_item).await;
        } else if sub_item.r#type == "directory" {
            let _ = run_directory(&sub_item).await;
        } else if sub_item.r#type == "url" {
            let _ = run_url(&sub_item).await;
        }

        if delay_ms > 0 && index < ids.len() - 1 {
            sleep(Duration::from_millis(delay_ms)).await;
        }
    }

    Ok(())
}

async fn incr_launch_count(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
    LaunchItems::update_many()
        .col_expr(Column::LaunchCount, Expr::col(Column::LaunchCount).add(1))
        .col_expr(Column::LastUsedAt, Expr::value(Local::now().naive_local()))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await?;

    Ok(())
}

async fn incr_failure_count(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
    LaunchItems::update_many()
        .col_expr(Column::FailureCount, Expr::col(Column::FailureCount).add(1))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await?;

    Ok(())
}
