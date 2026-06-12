use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::fs;
use tauri::Manager;

use crate::db::migration::Migrator;

pub async fn init_db(app: &tauri::App) -> Result<DatabaseConnection, DbErr> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    fs::create_dir_all(&app_data_dir).map_err(|e| DbErr::Custom(e.to_string()))?;

    let db_path = app_data_dir.join("Date.db");

    let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

    let db = Database::connect(db_url).await?;

    // 执行数据库迁移
    Migrator::up(&db, None).await?;

    Ok(db)
}
