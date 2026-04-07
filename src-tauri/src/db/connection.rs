use sea_orm::{Database, DatabaseConnection, DbErr};
use tauri::Manager;

pub async fn init_db(app: &tauri::App) -> Result<DatabaseConnection, DbErr> {
    let app_data_dir = app.path().app_data_dir().unwrap();

    let db_path = app_data_dir.join("Date.db");
    let db_url = format!("sqlite://{}?mode=rwc", db_path.to_str().unwrap());

    let db = Database::connect(db_url).await?;
    db.ping().await?;

    Ok(db)
}
