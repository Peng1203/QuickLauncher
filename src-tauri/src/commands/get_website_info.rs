use scraper::{Html, Selector};
use tauri_plugin_http::reqwest::{self, Proxy};

#[derive(serde::Serialize)]
pub struct WebsiteInfo {
    pub title: String,
    pub icon: Option<String>,
}

#[tauri::command]
pub async fn get_website_info(url: String) -> Result<WebsiteInfo, String> {
    log::info!("Fetching website info for URL: {}", url);

    // TODO 根据用户设置决定是否使用代理
    // 构建支持代理的 Client
    // use reqwest::Proxy;
    // let proxy = Proxy::all("http://username:password@127.0.0.1:7890").expect("创建代理失败");
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();
    // let client = reqwest::Client::builder()
    //     .proxy(Proxy::all("http://127.0.0.1:10090").unwrap())
    //     .build()
    //     .map_err(|e| format!("创建客户端失败: {}", e))?;
    // // 发送 GET 请求
    // let resp = client
    //     .get(&url)
    //     .send()
    //     .await
    //     .map_err(|e| format!("请求失败: {}", e))?
    //     .text()
    //     .await
    //     .map_err(|e| format!("获取文本失败: {}", e))?;

    // 发送 GET 请求
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("请求失败: {}", e))?
        .text()
        .await
        .map_err(|e| format!("获取文本失败: {}", e))?;

    // 解析 HTML
    let document = Html::parse_document(&resp);

    // 获取 title
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|t| t.inner_html())
        .unwrap_or_default();

    // 获取 favicon
    let icon_selector = Selector::parse(r#"link[rel="icon"], link[rel="shortcut icon"]"#).unwrap();
    let icon_path = document
        .select(&icon_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(|s| s.to_string());

    Ok(WebsiteInfo {
        title,
        icon: icon_path,
    })
}
