use rusqlite::{Connection, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use once_cell::sync::OnceCell;

static DB_CONN: OnceCell<Mutex<Connection>> = OnceCell::new();

fn ensure_dir_exists(path: &str) {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path).expect("创建目录失败");
    }
}

// 创建并返回数据库连接
fn connect_db(date_dir: PathBuf) -> Result<Connection> {
    ensure_dir_exists(date_dir.to_str().unwrap());
    // 数据库路径
    let db_path = PathBuf::from(date_dir).join("Date.db");
    Connection::open(db_path)
}

// 创建数据库表
fn create_tables(conn: &Connection) -> Result<()> {
    // 创建查询项表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS launch_items (
	        id INTEGER PRIMARY KEY AUTOINCREMENT,
	        name TEXT NOT NULL,
	        path TEXT NOT NULL,
	        type TEXT NOT NULL CHECK (type IN ('file', 'directory', 'url')),
	        icon TEXT,
	        hotkey TEXT,
          hotkey_global INTEGER DEFAULT 0 CHECK (hotkey_global IN (0, 1)),
          pinyin_full TEXT,
          pinyin_abbr TEXT,
	        keywords TEXT,
	        start_dir TEXT,
	        remarks TEXT,
          args TEXT,
          run_as_admin INTEGER DEFAULT 0 CHECK (run_as_admin IN (0, 1)),
	        order_index INTEGER DEFAULT 0,
          enabled INTEGER DEFAULT 1 CHECK (enabled IN (0, 1)),
	        category_id INTEGER,
          subcategory_id INTEGER,
          last_used_at DATETIME,
          created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
          updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
          extension TEXT,
          launch_count INTEGER DEFAULT 0,
          failure_count INTEGER DEFAULT 0,
	        FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL,
          FOREIGN KEY (subcategory_id) REFERENCES categories(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // 创建查询索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_name ON launch_items(name)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_pinyin_full ON launch_items(pinyin_full)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_pinyin_abbr ON launch_items(pinyin_abbr)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_keywords ON launch_items(keywords)",
        [],
    )?;

    // 创建分类表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL UNIQUE,
          parent_id INTEGER NOT NULL DEFAULT 0,
          association_directory TEXT,
          created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
          updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    // TODO 创建搜索记录表
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS search_history (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         query TEXT NOT NULL,
    //         timestamp TEXT NOT NULL
    //     )",
    //     [],
    // )?;

    // 创建配置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS configs (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL UNIQUE,
          data TEXT NOT NULL
        );",
        [],
    )?;

    Ok(())
}

// app_data_dir: PathBuf
// 初始化数据库
pub fn init_db() -> Result<()> {
    let identifier = "com.quicklauncher.app";
    let app_data_dir = dirs::data_dir().unwrap().join(identifier);
    // 连接数据库
    let conn = connect_db(app_data_dir)?;

    create_tables(&conn)?;

    let _ = DB_CONN.set(Mutex::new(conn));

    // 创建表
    Ok(())
}

// 外部调用：获取连接（全局唯一）
pub fn get_conn() -> &'static Mutex<Connection> {
    DB_CONN
        .get()
        .expect("数据库尚未初始化，请先调用 init_db_once")
}
