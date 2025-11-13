use std::time::Duration;

use base64::{engine::general_purpose, Engine as _};
use tauri::{AppHandle, Url};
use tauri_plugin_http::reqwest::{redirect, Client, Proxy};
use tauri_plugin_pinia::ManagerExt;

const MAX_IMAGE_SIZE: u64 = 2 * 1024 * 1024; // 2MB

#[tauri::command]
pub async fn get_online_img_base64(url: String, app: AppHandle) -> Result<String, String> {
    let proxy = app.pinia().get::<bool>("appConfig", "proxy").unwrap();
    let proxy_host = app.pinia().get::<String>("appConfig", "proxyHost").unwrap();
    let proxy_username = app
        .pinia()
        .get::<String>("appConfig", "proxyUsername")
        .unwrap();
    let proxy_password = app
        .pinia()
        .get::<String>("appConfig", "proxyPassword")
        .unwrap();

    // 解析基础URL，用于后续的相对路径处理
    let base_url = Url::parse(&url).map_err(|e| format!("无效的URL: {}", e))?;

    dbg!(&base_url);

    let mut client_builder = Client::builder()
        .redirect(redirect::Policy::limited(10))
        .cookie_store(true)
        .timeout(Duration::from_secs(10));

    if proxy && !proxy_host.is_empty() {
        // 如果有用户名密码 → http://user:pass@host
        let proxy_url = if !proxy_username.is_empty() && !proxy_password.is_empty() {
            format!("{}:{}@{}", proxy_username, proxy_password, proxy_host)
        } else {
            proxy_host.clone()
        };

        let proxy = Proxy::all(&proxy_url).map_err(|e| format!("创建代理失败: {}", e))?;
        client_builder = client_builder.proxy(proxy);
    }

    let client = client_builder
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    // 发送请求
    let response = client
        .get(base_url.as_str())
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    // 检查响应状态
    if !response.status().is_success() {
        return Err(format!("请求失败，状态码: {}", response.status()));
    }

    // 先提取所有需要的信息，再消费 response
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.starts_with("image/") {
        return Err(format!(
            "请求的资源不是图片，Content-Type: {}",
            content_type
        ));
    }

    // 检查 Content-Length（如果有）
    let content_length = response.content_length();
    if let Some(length) = content_length {
        if length > MAX_IMAGE_SIZE {
            return Err(format!(
                "图片大小超过限制，最大允许 2MB，实际大小: {:.2}MB",
                length as f64 / 1024.0 / 1024.0
            ));
        }
    }

    // 获取图片字节数据（消费 response）
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("读取图片数据失败: {}", e))?;

    // 再次检查实际大小
    if bytes.len() > MAX_IMAGE_SIZE as usize {
        return Err(format!(
            "图片大小超过限制，最大允许 2MB，实际大小: {:.2}MB",
            bytes.len() as f64 / 1024.0 / 1024.0
        ));
    }

    // 根据 Content-Type 提取 MIME 类型
    let mime_type = content_type
        .split(';')
        .next()
        .unwrap_or("image/jpeg")
        .trim();

    // 转换为 base64
    let base64_string = general_purpose::STANDARD.encode(&bytes);

    // 返回 data URL 格式
    Ok(format!("data:{};base64,{}", mime_type, base64_string))
}
