use base64::Engine;
use scraper::{Html, Selector};
use std::time::Duration;
use tauri::{AppHandle, Url};
use tauri_plugin_http::reqwest::{self, redirect, Client, Proxy};
use tauri_plugin_pinia::ManagerExt;
use ua_generator::ua::spoof_ua;

#[derive(serde::Serialize)]
pub struct WebsiteInfo {
    pub title: String,
    pub icon: Option<String>,
}

#[tauri::command]
pub async fn get_website_info(
    url: String,
    // state: State<'_, Mutex<AppConfigState>>,
    app: AppHandle,
) -> Result<WebsiteInfo, String> {
    // let (proxy, proxy_host, proxy_username, proxy_password) = {
    //     let guard = state.lock().unwrap();
    //     (
    //         guard.proxy,
    //         guard.proxy_host.clone(),
    //         guard.proxy_username.clone(),
    //         guard.proxy_password.clone(),
    //     )
    // };

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

    let mut client_builder = Client::builder()
        .redirect(redirect::Policy::limited(10))
        .cookie_store(true)
        .timeout(Duration::from_secs(10));

    if proxy && !proxy_host.is_empty() {
        // 如果有用户名密码 → http://user:pass@host
        let proxy_url = if !proxy_username.is_empty() && !proxy_password.is_empty() {
            format!("{}:{}@{}", proxy_username, proxy_password, proxy_host)
        } else {
            format!("{}", proxy_host)
        };

        let proxy = Proxy::all(&proxy_url).map_err(|e| format!("创建代理失败: {}", e))?;
        client_builder = client_builder.proxy(proxy);
    }

    let client = client_builder
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))
        .unwrap();

    // 发送 GET 请求
    let resp = client
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml;q=0.9,*/*;q=0.8")
        .header("User-Agent", spoof_ua())
        .header("Accept-Language", "zh-CN,zh;q=0.9")
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?
        .text()
        .await
        .map_err(|e| format!("获取文本失败: {}", e))?;

    // 解析 HTML
    let (title, icon_url) = {
        let document = Html::parse_document(&resp);

        // 获取 title
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|t| t.inner_html())
            .unwrap_or_default();

        // 获取 favicon
        let icon_selector =
            Selector::parse(r#"link[rel="icon"], link[rel="shortcut icon"]"#).unwrap();

        let icon_path = document
            .select(&icon_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|s| s.to_string());

        let icon_url = process_icon_path(&icon_path.unwrap_or_default(), &base_url);
        (title, icon_url)
    };
    // TODO 处理 meta 刷新重定向
    // let selector = Selector::parse(r#"meta[http-equiv="refresh" i]"#).unwrap();
    // let refresh_meta = document.select(&selector).next();

    // 将 icon 转换为 base64
    let icon_base64 = icon_to_base64(client, icon_url).await;

    Ok(WebsiteInfo {
        title,
        icon: icon_base64,
    })
}

async fn icon_to_base64(client: Client, url: String) -> Option<String> {
    if url.is_empty() {
        return None;
    }

    match client.get(&url).send().await {
        Ok(resp) => {
            // 获取 Content-Type，默认为 image/png
            let mime = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("image/png")
                .to_string();

            match resp.bytes().await {
                Ok(bytes) => {
                    // 检查是否为空或过大的文件
                    if bytes.is_empty() {
                        log::warn!("Icon file is empty: {}", url);
                        return None;
                    }

                    // 限制文件大小1MB
                    if bytes.len() > 1024 * 1024 {
                        log::warn!("Icon file too large ({}KB): {}", bytes.len() / 1024, url);
                        return None;
                    }

                    // 编码为 base64
                    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                    Some(format!("data:{};base64,{}", mime, b64))
                }
                Err(e) => {
                    log::error!("Failed to read icon bytes from {}: {}", url, e);
                    None
                }
            }
        }
        Err(e) => {
            log::error!("Failed to fetch icon from {}: {}", url, e);
            None
        }
    }
}

fn process_icon_path(icon_path: &str, base_url: &Url) -> String {
    // 如果已经是完整的URL，直接返回
    if icon_path.starts_with("http://") || icon_path.starts_with("https://") {
        return icon_path.to_string();
    }

    // 判断是否有双斜杠 (协议相对URL)
    if icon_path.starts_with("//") {
        // 使用原始URL的协议
        return format!("{}:{}", base_url.scheme(), icon_path);
    }

    // 获取域名部分 (协议 + 域名，不包含路径和参数)
    let domain_part = format!(
        "{}://{}",
        base_url.scheme(),
        base_url.host_str().unwrap_or("")
    );

    // 如果是绝对路径 (以 / 开头)
    if icon_path.starts_with("/") {
        return format!("{}{}", domain_part, icon_path);
    }

    // 相对路径，拼接域名
    format!("{}/{}", domain_part, icon_path.trim_start_matches('/'))
}
