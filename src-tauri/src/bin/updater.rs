use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return;
    }

    // argv[1]
    let app_path = PathBuf::from(&args[1]);

    // argv[2]
    let import_db = PathBuf::from(&args[2]);

    // argv[3]
    let target_db = PathBuf::from(&args[3]);

    // 等待文件解锁
    wait_until_unlocked(&target_db);

    // 删除 wal/shm
    remove_sqlite_temp_files(&target_db);

    // 备份旧 db
    backup_old_db(&target_db);

    // 替换 db
    fs::copy(&import_db, &target_db).expect("copy database failed");

    // 启动 app
    Command::new(app_path).spawn().expect("restart app failed");
}

fn wait_until_unlocked(path: &Path) {
    loop {
        let result = fs::OpenOptions::new().read(true).write(true).open(path);

        match result {
            Ok(_) => {
                break;
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(300));
            }
        }
    }
}

fn remove_sqlite_temp_files(db_path: &Path) {
    let wal = db_path.with_extension("db-wal");
    let shm = db_path.with_extension("db-shm");

    let _ = fs::remove_file(wal);
    let _ = fs::remove_file(shm);
}

fn backup_old_db(db_path: &Path) {
    if !db_path.exists() {
        return;
    }

    let backup = db_path.with_extension("db.old");

    let _ = fs::remove_file(&backup);

    fs::rename(db_path, backup).expect("backup old db failed");
}
