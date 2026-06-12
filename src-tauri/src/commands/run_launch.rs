use crate::{
    commands::exe_command::exec_command_internal, common::utils::run_as_admin, entity, AppState,
};
use chrono::Utc;
use entity::launch_items::{Column, Entity as LaunchItems, Model};
use sea_orm::{
    prelude::Expr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, QueryFilter,
};
use std::{os::windows::process::CommandExt, process::Command};
use tracing;

#[tracing::instrument(skip(state), fields(id))]
#[tauri::command]
pub async fn run_launch(id: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    // ORM 查询
    let launch_item: Model = LaunchItems::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("查询启动项失败: {}", e))?
        .ok_or("启动项不存在")?;

    if launch_item.r#type == "file" {
        tracing::info!(name = launch_item.name, r#type = "file", "执行启动项");
        let _ = run_file(&launch_item).await;
    } else if launch_item.r#type == "directory" {
        tracing::info!(name = launch_item.name, r#type = "directory", "打开目录");
        let _ = run_directory(&launch_item).await;
    } else if launch_item.r#type == "url" {
        tracing::info!(name = launch_item.name, url = launch_item.path, "打开URL");
        let _ = run_url(&launch_item).await;
    } else if launch_item.r#type == "alias" {
        tracing::info!(
            name = launch_item.name,
            path = launch_item.path,
            "执行命令别名"
        );
        let _ = run_alias(&launch_item).await;
    } else if launch_item.r#type == "apps" {
        // let _ = run_apps(&launch_item, &db).await;

        let launch_item = launch_item.clone();

        let db_clone = db.clone();
        tokio::spawn(async move {
            if let Err(e) = run_apps(&launch_item, &db_clone).await {
                tracing::error!(%e, "执行 apps 失败");
            }
            drop(db_clone);
        });
    }

    // 更新启动项目的启动次数和最后使用时间
    incr_launch_count(&db, id)
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
    dbg!(launch_item);

    let mut args = vec!["/C".to_string(), "start".to_string(), "".to_string()];

    // 解析 msedge+--new-window
    let mut launch_parts: Vec<String> = Vec::new();

    if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
        launch_parts = arg
            .split('+')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    if let Some(exe) = launch_parts.get(0) {
        args.push(exe.clone());

        for flag in launch_parts.iter().skip(1) {
            args.push(flag.clone());
        }
    }

    args.push(launch_item.path.clone());

    Command::new("cmd")
        .creation_flags(0x08000000)
        .args(args)
        .spawn()
        .map_err(|e| format!("无法打开URL: {}", e))?;

    Ok(())
}
// async fn run_url(launch_item: &Model) -> Result<(), String> {
//     let mut args = vec!["/C".to_string(), "start".to_string(), "".to_string()];

//     if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
//         args.push(arg.clone());
//     }

//     args.push(launch_item.path.clone());

//     Command::new("cmd")
//         .creation_flags(0x08000000)
//         .args(args)
//         .spawn()
//         .map_err(|e| format!("无法打开URL: {}", e))?;

//     Ok(())
// }

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
        .col_expr(
            Column::LastUsedAt,
            Expr::value(Utc::now().timestamp_millis()),
        )
        .filter(Column::Id.eq(id))
        .exec(db)
        .await?;

    Ok(())
}

async fn _incr_failure_count(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
    LaunchItems::update_many()
        .col_expr(Column::FailureCount, Expr::col(Column::FailureCount).add(1))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await?;

    Ok(())
}
