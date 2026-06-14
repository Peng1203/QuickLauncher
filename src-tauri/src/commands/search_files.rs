use crate::models::file_search::FileSearchResult;
use encoding_rs::GBK;
use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::AppHandle;
use tauri_plugin_http::reqwest;
use tauri_plugin_pinia::ManagerExt;
use tracing;
use windows_icons::get_icon_base64_by_path;

/// 检测 Everything 是否正在运行
fn is_everything_running() -> bool {
    let output = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq Everything.exe", "/NH"])
        .creation_flags(0x08000000)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("Everything.exe")
        }
        Err(_) => false,
    }
}

/// 启动 Everything
fn start_everything(path: &str) -> Result<(), String> {
    tracing::info!(path, "启动 Everything");

    // 使用 cmd /C start 方式启动，确保 Everything 正确初始化
    Command::new("cmd")
        .args(["/C", "start", "", path])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()
        .map_err(|e| {
            tracing::error!(%e, "启动 Everything 失败");
            format!("启动 Everything 失败: {}", e)
        })?;

    Ok(())
}

#[tracing::instrument(skip(app))]
#[tauri::command]
pub async fn search_files(
    keyword: String,
    app: AppHandle,
) -> Result<Vec<FileSearchResult>, String> {
    if keyword.trim().is_empty() {
        return Ok(vec![]);
    }

    // 从 Pinia 读取配置
    let search_mode = app
        .pinia()
        .get::<String>("appConfig", "fileSearchMode")
        .unwrap_or_else(|_| "es".to_string());

    let max_results = app
        .pinia()
        .get::<u32>("appConfig", "fileSearchMaxResults")
        .unwrap_or(20);

    let search_path = app
        .pinia()
        .get::<String>("appConfig", "fileSearchPath")
        .unwrap_or_default();

    let filter = app
        .pinia()
        .get::<String>("appConfig", "fileSearchFilter")
        .unwrap_or_default();

    let sort = app
        .pinia()
        .get::<String>("appConfig", "fileSearchSort")
        .unwrap_or_else(|_| "default".to_string());

    let regex = app
        .pinia()
        .get::<bool>("appConfig", "fileSearchRegex")
        .unwrap_or(false);

    // 检查是否需要自动启动 Everything
    let auto_start = app
        .pinia()
        .get::<bool>("appConfig", "fileSearchAutoStart")
        .unwrap_or(false);

    // let everything_is_running = is_everything_running();
    // tracing::info!(auto_start, everything_is_running, search_mode, "文件搜索准备");

    // if auto_start && !everything_is_running {
    //     let everything_path = app
    //         .pinia()
    //         .get::<String>("appConfig", "everythingExePath")
    //         .unwrap_or_default();

    //     tracing::info!(everything_path, "尝试启动 Everything");

    //     if everything_path.is_empty() {
    //         tracing::warn!("Everything.exe 路径未配置，无法自动启动");
    //     } else if !std::path::Path::new(&everything_path).exists() {
    //         tracing::warn!(path = everything_path, "Everything.exe 文件不存在");
    //     } else {
    //         match start_everything(&everything_path) {
    //             Ok(()) => {
    //                 tracing::info!("Everything 启动成功，等待初始化...");
    //                 tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    //                 tracing::info!("Everything 等待完成");
    //             }
    //             Err(e) => {
    //                 tracing::warn!(%e, "自动启动 Everything 失败");
    //             }
    //         }
    //     }
    // } else if auto_start && search_mode == "http" {
    //     // HTTP 模式下，即使 Everything 已运行，也等待一下确保 HTTP 服务器就绪
    //     tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    // }

    let stdout = if search_mode == "http" {
        // HTTP 模式
        let host = app
            .pinia()
            .get::<String>("appConfig", "everythingHttpHost")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = app
            .pinia()
            .get::<u32>("appConfig", "everythingHttpPort")
            .unwrap_or(80);

        search_via_http(
            &keyword,
            &host,
            port,
            max_results,
            &search_path,
            &filter,
            &sort,
            regex,
        )
        .await?
    } else {
        // es.exe 模式
        let es_path = app
            .pinia()
            .get::<String>("appConfig", "esFilePath")
            .unwrap_or_default();

        if es_path.is_empty() {
            return Err("Everything es.exe 路径未配置".to_string());
        }

        if !std::path::Path::new(&es_path).exists() {
            return Err(format!("es.exe 文件不存在: {}", es_path));
        }

        // 重试机制：如果 IPC 窗口未就绪，等待后重试
        let mut stdout = String::new();
        let mut last_error = String::new();

        for attempt in 0..5 {
            tracing::info!(attempt, "尝试执行 es.exe 搜索...");
            match search_via_es(
                &keyword,
                &es_path,
                max_results,
                &search_path,
                &filter,
                &sort,
                regex,
            ) {
                Ok(result) => {
                    stdout = result;
                    tracing::info!(attempt, "es.exe 搜索成功");
                    break;
                }
                Err(e) => {
                    last_error = e.clone();
                    let is_ipc_error = e.contains("IPC") || e.contains("Error 8");
                    tracing::warn!(attempt, is_ipc_error, error = %e, "es.exe 搜索失败");

                    if is_ipc_error && attempt < 4 {
                        let wait_secs = 2 + (attempt as u64);
                        tracing::info!(wait_secs, "等待 IPC 窗口就绪...");
                        tokio::time::sleep(std::time::Duration::from_secs(wait_secs)).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        if stdout.is_empty() && !last_error.is_empty() {
            return Err(last_error);
        }
        stdout
    };

    let results = if search_mode == "http" {
        // HTTP 模式返回 JSON
        parse_json_results(&stdout)
    } else {
        // es.exe 模式返回 CSV
        parse_csv_results(&stdout)
    };

    tracing::info!(count = results.len(), "文件搜索完成");

    Ok(results)
}

/// 通过 HTTP API 调用 Everything 搜索
async fn search_via_http(
    keyword: &str,
    host: &str,
    port: u32,
    max_results: u32,
    search_path: &str,
    filter: &str,
    sort: &str,
    regex: bool,
) -> Result<String, String> {
    // Everything HTTP API 参数:
    // s=search, o=offset, c=count, j=json, r=regex, sort=name|path|date_modified|size
    // 注意：HTTP API 不支持 ext 参数，需要在搜索关键词中添加过滤条件

    // 构建搜索关键词，添加文件类型过滤
    let mut search_keyword = keyword.to_string();
    if !filter.is_empty() {
        // 将 *.exe;*.txt 转换为 Everything 搜索语法: ext:exe|txt
        let extensions: Vec<&str> = filter.split(';').collect();
        let ext_filter: Vec<String> = extensions
            .iter()
            .map(|ext| {
                let ext = ext.trim().trim_start_matches('*').trim_start_matches('.');
                format!("ext:{}", ext)
            })
            .collect();
        if !ext_filter.is_empty() {
            search_keyword = format!("({}) ({})", keyword, ext_filter.join("|"));
        }
    }

    let mut url = format!(
        "http://{}:{}/?s={}&j=1&path_column=1&size_column=1",
        host,
        port,
        urlencoding::encode(&search_keyword)
    );

    if max_results > 0 {
        url.push_str(&format!("&c={}", max_results));
    }

    if !search_path.is_empty() {
        url.push_str(&format!("&path={}", urlencoding::encode(search_path)));
    }

    if !sort.is_empty() && sort != "default" {
        url.push_str(&format!("&sort={}", sort));
    }

    if regex {
        url.push_str("&r=1");
    }

    tracing::info!(keyword, host, port, "执行 Everything HTTP 搜索");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("HTTP 请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("HTTP 请求失败: 状态码 {}", status));
    }

    let body = resp
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    Ok(body)
}

/// 通过 es.exe 调用 Everything 搜索
fn search_via_es(
    keyword: &str,
    es_path: &str,
    max_results: u32,
    search_path: &str,
    filter: &str,
    sort: &str,
    regex: bool,
) -> Result<String, String> {
    let mut args = vec![];

    // 搜索关键词
    args.push(keyword.to_string());

    // 每页结果数
    if max_results > 0 {
        args.push("-n".to_string());
        args.push(max_results.to_string());
    }

    // 搜索路径
    if !search_path.is_empty() {
        args.push("-path".to_string());
        args.push(search_path.to_string());
    }

    // 文件类型过滤
    if !filter.is_empty() {
        args.push("-ext".to_string());
        args.push(filter.to_string());
    }

    // 排序
    if !sort.is_empty() && sort != "default" {
        args.push("-sort".to_string());
        args.push(sort.to_string());
    }

    // 正则搜索
    if regex {
        args.push("-regex".to_string());
    }

    // CSV 输出
    args.push("-csv".to_string());

    tracing::info!(keyword, es_path, "执行 Everything es.exe 搜索");

    let output = Command::new(es_path).args(&args).output().map_err(|e| {
        tracing::error!(%e, "执行 es.exe 失败");
        format!("执行 es.exe 失败: {}", e)
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        tracing::error!(stderr, "es.exe 执行失败");
        return Err(format!("es.exe 执行失败: {}", stderr));
    }

    let raw_stdout = &output.stdout;
    let stdout = if raw_stdout.starts_with(&[0xEF, 0xBB, 0xBF]) {
        // UTF-8 BOM
        String::from_utf8_lossy(&raw_stdout[3..]).to_string()
    } else {
        let (decoded, _, _) = GBK.decode(raw_stdout);
        decoded.to_string()
    };

    Ok(stdout)
}

/// 解析 CSV 格式的搜索结果（es.exe 输出）
fn parse_csv_results(stdout: &str) -> Vec<FileSearchResult> {
    let mut results = vec![];
    let mut skip_header = true;

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // 跳过 CSV 表头
        if skip_header {
            skip_header = false;
            if line.contains("Filename") || line.contains("filename") {
                continue;
            }
        }

        // es.exe CSV 输出格式：每行是一个完整路径（带引号）
        // 格式: "C:\path\to\file.exe"
        let path = line.trim_matches('"').to_string();

        if path.is_empty() {
            continue;
        }

        if let Some(result) = create_result_from_path(&path) {
            results.push(result);
        }
    }

    results
}

/// 解析 JSON 格式的搜索结果（HTTP API 输出）
fn parse_json_results(json_str: &str) -> Vec<FileSearchResult> {
    let mut results = vec![];

    // Everything HTTP API JSON 格式: {"results":[{"name":"...", "path":"...", "type":"file/folder"}]}
    // path 是父目录路径，完整路径需要拼接 name

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(results_arr) = json.get("results").and_then(|v| v.as_array()) {
            for item in results_arr {
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let path = item.get("path").and_then(|v| v.as_str()).unwrap_or("");

                if name.is_empty() {
                    continue;
                }

                // 构建完整路径
                let full_path = if path.is_empty() {
                    name.to_string()
                } else {
                    format!("{}\\{}", path, name)
                };

                if let Some(result) = create_result_from_path(&full_path) {
                    results.push(result);
                }
            }
            return results;
        }
    }

    // 如果 JSON 解析失败，尝试按行解析（简单格式）
    for line in json_str.lines() {
        let path = line.trim().trim_matches('"').to_string();
        if path.is_empty() {
            continue;
        }
        if let Some(result) = create_result_from_path(&path) {
            results.push(result);
        }
    }

    results
}

/// 从路径创建搜索结果
fn create_result_from_path(path: &str) -> Option<FileSearchResult> {
    let path_obj = std::path::Path::new(path);

    // 验证路径格式
    if !path_obj.is_absolute() {
        return None;
    }

    // 提取文件名
    let name = if path_obj.is_dir() {
        path_obj
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string())
    } else {
        path_obj
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string())
    };

    // 提取扩展名
    let extension = if path_obj.is_file() {
        path_obj
            .extension()
            .map(|ext| ext.to_string_lossy().to_string())
    } else {
        None
    };

    // 获取文件大小
    let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

    // 获取文件类型
    let file_type = if path_obj.is_dir() {
        "directory".to_string()
    } else {
        "file".to_string()
    };

    // 获取文件图标
    let icon = get_icon_base64_by_path(path)
        .map(|base64| format!("data:image/png;base64,{}", base64))
        .unwrap_or_default();

    Some(FileSearchResult {
        name,
        path: path.to_string(),
        size,
        icon,
        r#type: file_type,
        extension,
    })
}
