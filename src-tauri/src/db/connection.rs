// use crate::entity;
// use entity::prelude::*;
// use sea_orm::{Database, DatabaseConnection, DbErr};
// use tauri::Manager;

// pub async fn init_db(app: &tauri::App) -> Result<DatabaseConnection, DbErr> {
//     let app_data_dir = app.path().app_data_dir().unwrap();

//     let db_path = app_data_dir.join("Date.db");
//     let db_url = format!("sqlite://{}?mode=rwc", db_path.to_str().unwrap());
//     let db = &Database::connect(db_url).await?;
//     // 同步数据库结构
//     db.get_schema_builder()
//         .register(Categories)
//         .register(AutocompleteHistory)
//         .register(Configs)
//         .register(LaunchHistory)
//         .register(LaunchItems)
//         .sync(db)
//         .await?;

//     Ok(db.clone())
// }

use crate::entity;
use entity::prelude::*;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::fs;
use tauri::Manager;

pub async fn init_db(app: &tauri::App) -> Result<DatabaseConnection, DbErr> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    fs::create_dir_all(&app_data_dir).map_err(|e| DbErr::Custom(e.to_string()))?;

    let db_path = app_data_dir.join("Date.db");

    let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

    let db = Database::connect(db_url).await?;

    db.get_schema_builder()
        .register(Categories)
        .register(AutocompleteHistory)
        .register(Configs)
        .register(LaunchHistory)
        .register(LaunchItems)
        .sync(&db)
        .await?;

    Ok(db)
}
