use crate::AppState;
use std::{fs, time::SystemTime};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_pinia::ManagerExt;
use tracing;

/// 获取 WebDAV 配置
fn get_webdav_config(app: &AppHandle) -> Result<(String, String, String, String), String> {
    let url = app
        .pinia()
        .get::<String>("appConfig", "webdavUrl")
        .map_err(|e| format!("获取 WebDAV 地址失败: {}", e))?;

    let username = app
        .pinia()
        .get::<String>("appConfig", "webdavUsername")
        .map_err(|e| format!("获取用户名失败: {}", e))?;

    let password = app
        .pinia()
        .get::<String>("appConfig", "webdavPassword")
        .map_err(|e| format!("获取密码失败: {}", e))?;

    let path = app
        .pinia()
        .get::<String>("appConfig", "webdavPath")
        .map_err(|e| format!("获取路径失败: {}", e))?;

    if url.is_empty() || username.is_empty() || password.is_empty() {
        return Err("请先配置 WebDAV 连接信息".to_string());
    }

    Ok((url, username, password, path))
}

/// 构建 WebDAV 请求 URL
fn build_webdav_url(base_url: &str, remote_path: &str, filename: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let path = remote_path.trim_start_matches('/');
    format!("{}/{}/{}", base, path, filename)
}

/// 测试 WebDAV 连接
#[tracing::instrument(skip(app))]
#[tauri::command]
pub async fn webdav_test_connection(app: AppHandle) -> Result<String, String> {
    let (url, username, password, _path) = get_webdav_config(&app)?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    // 使用 PROPFIND 请求测试连接
    let resp = client
        .request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &url)
        .basic_auth(&username, Some(&password))
        .header("Depth", "0")
        .send()
        .await
        .map_err(|e| format!("连接失败: {}", e))?;

    if resp.status().is_success() || resp.status().as_u16() == 207 {
        Ok("连接成功".to_string())
    } else {
        Err(format!("连接失败: 状态码 {}", resp.status()))
    }
}

/// 云备份
#[tracing::instrument(skip(app))]
#[tauri::command]
pub async fn webdav_backup(app: AppHandle) -> Result<String, String> {
    let (url, username, password, remote_path) = get_webdav_config(&app)?;

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("Date.db");

    if !db_path.exists() {
        return Err("数据库文件不存在".to_string());
    }

    // 读取数据库文件
    let db_bytes = fs::read(&db_path).map_err(|e| format!("读取数据库失败: {}", e))?;

    // 生成备份文件名
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let filename = format!("{}_Date.db", timestamp);

    // 构建上传 URL
    let upload_url = build_webdav_url(&url, &remote_path, &filename);

    tracing::info!(url = upload_url, "开始云备份");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    let resp = client
        .put(&upload_url)
        .basic_auth(&username, Some(&password))
        .header("Content-Type", "application/octet-stream")
        .body(db_bytes)
        .send()
        .await
        .map_err(|e| format!("上传失败: {}", e))?;

    if resp.status().is_success() || resp.status().as_u16() == 201 || resp.status().as_u16() == 204 {
        tracing::info!("云备份成功: {}", filename);
        Ok(format!("备份成功: {}", filename))
    } else {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::error!(status = %status, body, "云备份失败");
        Err(format!("备份失败: 状态码 {} - {}", status, body))
    }
}

/// 云恢复
#[tracing::instrument(skip(app, state))]
#[tauri::command]
pub async fn webdav_restore(
    filename: String,
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let (url, username, password, remote_path) = get_webdav_config(&app)?;

    // 构建下载 URL
    let download_url = build_webdav_url(&url, &remote_path, &filename);

    tracing::info!(url = download_url, "开始云恢复");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    let resp = client
        .get(&download_url)
        .basic_auth(&username, Some(&password))
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::error!(status = %status, body, "云恢复失败");
        return Err(format!("下载失败: 状态码 {} - {}", status, body));
    }

    let db_bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    // 关闭数据库连接
    let conn = {
        let mut db = state.db.lock().unwrap();
        db.take()
    };
    if let Some(conn) = conn {
        conn.close().await.map_err(|e| e.to_string())?;
    }

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let target_db_path = app_data_dir.join("Date.db");

    // 备份当前数据库
    if target_db_path.exists() {
        let old_path = target_db_path.with_extension("db.old");
        if old_path.exists() {
            let _ = fs::remove_file(&old_path);
        }
        fs::rename(&target_db_path, &old_path)
            .map_err(|e| format!("备份现有数据库失败: {}", e))?;
    }

    // 写入新数据库
    fs::write(&target_db_path, &db_bytes).map_err(|e| format!("写入数据库失败: {}", e))?;

    tracing::info!("云恢复成功: {}", filename);

    // 重启应用
    app.restart()
}

/// 列出云端备份
#[tracing::instrument(skip(app))]
#[tauri::command]
pub async fn webdav_list_backups(app: AppHandle) -> Result<Vec<WebDavBackupInfo>, String> {
    let (url, username, password, remote_path) = get_webdav_config(&app)?;

    // 构建目录 URL
    let dir_url = if remote_path.ends_with('/') {
        format!("{}{}", url.trim_end_matches('/'), remote_path)
    } else {
        format!("{}/{}", url.trim_end_matches('/'), remote_path)
    };

    tracing::info!(url = dir_url, "获取云端备份列表");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    let resp = client
        .request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &dir_url)
        .basic_auth(&username, Some(&password))
        .header("Depth", "1")
        .header("Content-Type", "application/xml")
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !resp.status().is_success() && resp.status().as_u16() != 207 {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::error!(status = %status, body, "获取备份列表失败");
        return Err(format!("获取列表失败: 状态码 {} - {}", status, body));
    }

    let xml = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;

    // 简单解析 XML，提取 .db 文件
    let mut backups = Vec::new();
    let mut lines = xml.lines();
    while let Some(line) = lines.next() {
        if line.contains("href") && line.contains("_Date.db") {
            // 提取文件名
            if let Some(start) = line.find(">") {
                if let Some(end) = line.find("</") {
                    let href = &line[start + 1..end];
                    let filename = href
                        .rsplit('/')
                        .next()
                        .unwrap_or("")
                        .to_string();

                    if !filename.is_empty() && filename.ends_with(".db") {
                        // 提取时间戳
                        let timestamp: u64 = filename
                            .split('_')
                            .next()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);

                        backups.push(WebDavBackupInfo {
                            filename,
                            timestamp,
                        });
                    }
                }
            }
        }
    }

    // 按时间戳降序排序
    backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    tracing::info!(count = backups.len(), "获取云端备份列表完成");
    Ok(backups)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WebDavBackupInfo {
    pub filename: String,
    pub timestamp: u64,
}
