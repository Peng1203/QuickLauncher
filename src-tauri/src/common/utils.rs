use crate::entity::launch_items::Model;
use crate::models::launch_item::LaunchItemDto;

use pinyin::ToPinyin;
use std::{os::windows::process::CommandExt, process::Command};
use url::Url;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromPoint, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, GetDesktopWindow, GetForegroundWindow, GetShellWindow, GetWindowRect,
};

// 获取拼音的全拼和缩写
pub fn get_pinyin_variants(name: &str) -> (String, String) {
    let mut full = String::new();
    let mut abbr = String::new();

    for ch in name.chars() {
        if let Some(p) = ch.to_pinyin() {
            let p_str = p.plain().to_string();
            full.push_str(&p_str);
            abbr.push(p_str.chars().next().unwrap_or_default());
        } else {
            // 非中文字符（保留原字符）
            full.push(ch);
            abbr.push(ch);
        }
    }

    (full.to_lowercase(), abbr.to_lowercase())
}

// 以管理员权限运行
pub fn run_as_admin(launch_item: Model) -> Result<(), String> {
    let mut powershell_cmd = vec![
        "Start-Process".to_string(),
        "-FilePath".to_string(),
        format!("'{}'", launch_item.path),
        "-Verb".to_string(),
        "RunAs".to_string(),
    ];

    let start_dir = launch_item
        .start_dir
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("C:\\Windows\\System32");

    // 如果有参数，则添加到 `-ArgumentList`
    if let Some(arg) = launch_item.args.as_ref().filter(|s| !s.trim().is_empty()) {
        powershell_cmd.push("-ArgumentList".to_string());
        powershell_cmd.push(format!("'{}'", arg));
    }

    if let Err(e) = Command::new("powershell")
        .creation_flags(0x08000000)
        .current_dir(start_dir)
        .args(&powershell_cmd)
        .spawn()
    {
        log::error!("❌ 无法以管理员权限运行: {}", e);
        return Err(format!("❌ 无法以管理员权限运行: {}", e));
    }

    Ok(())
}

// 是否是网站
/// 判断输入是否是有效的网址
///
/// 规则（按优先级）：
/// 1. 快速排除：包含空格 → 返回 false（排除命令组合如 "ping baidu.com"）
/// 2. 有有效网址协议 → 返回 true（http:// https:// ftp:// ftps://）
/// 3. 看起来像纯域名 → 检查是否有有效 TLD
pub fn is_valid_url(input: &str) -> bool {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return false;
    }

    // ============ 快速排除包含空格的输入（命令组合） ============
    // "ping baidu.com", "curl https://...", "command domain.com" 等一律排除
    if trimmed.contains(char::is_whitespace) {
        return false;
    }

    // ============ 排除邮箱 ============
    if is_email(trimmed) {
        return false;
    }

    // ============ 检查是否有有效的网址协议 ============
    let web_protocols = ["http://", "https://", "ftp://", "ftps://"];
    if web_protocols.iter().any(|proto| trimmed.starts_with(proto)) {
        // 协议后面至少要有域名字符
        let after_protocol = trimmed.split("://").nth(1).unwrap_or("");
        return !after_protocol.trim().is_empty();
    }

    // ============ 没有协议，检查是否像纯域名 ============

    // 必须包含 . 才能是域名（不是单个单词）
    if !trimmed.contains('.') {
        return false;
    }

    // 不能只有协议符号（ms-settings: / file: / custom:）
    if trimmed.ends_with(':') {
        return false;
    }

    // 检查是否有有效的 TLD
    const COMMON_TLDS: [&str; 43] = [
        // 🌍 国际通用
        ".com",
        ".net",
        ".org",
        ".info",
        ".biz",
        ".xyz",
        ".top",
        ".site",
        ".online",
        ".club",
        // 🧑‍💻 技术 / 创业 / 产品
        ".io",
        ".ai",
        ".dev",
        ".app",
        ".tech",
        ".cloud",
        ".digital",
        ".software",
        ".tools",
        // 🏢 商业 / 品牌
        ".shop",
        ".store",
        ".company",
        ".inc",
        ".ltd",
        ".group",
        // 🌏 国家 & 地区（高频）
        ".cn",
        ".jp",
        ".kr",
        ".hk",
        ".tw",
        ".uk",
        ".us",
        ".de",
        ".fr",
        ".au",
        ".ca",
        ".sg",
        // 🧩 复合后缀（非常常见）
        ".co",
        ".cc",
        ".me",
        ".tv",
        ".co.jp",
        ".co.uk",
    ];

    let lower = trimmed.to_lowercase();

    // 去掉 path / query 再判断后缀
    let domain_part = lower.split('/').next().unwrap_or(&lower);

    COMMON_TLDS.iter().any(|tld| domain_part.ends_with(tld))
}

// ============ 邮箱检测函数 ============
fn is_email(input: &str) -> bool {
    let trimmed = input.trim();

    // 检查是否包含 @
    if !trimmed.contains('@') {
        return false;
    }

    // 按 @ 分割，应该恰好有两部分
    let parts: Vec<&str> = trimmed.split('@').collect();
    if parts.len() != 2 {
        return false; // 多个 @ 或没有 @
    }

    let local = parts[0]; // @ 前面的部分
    let domain = parts[1]; // @ 后面的部分

    // 检查本地部分和域名部分都不为空
    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // 本地部分检查：允许字母、数字、点、下划线、连字符等
    // 不能以点开头或结尾，不能有连续的点
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }

    // 本地部分只能包含字母、数字、点、下划线、连字符、加号
    let valid_local_chars =
        |c: char| c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '+' || c == '&';
    if !local.chars().all(valid_local_chars) {
        return false;
    }

    // 域名部分检查：必须包含 . 和有效的 TLD
    if !domain.contains('.') {
        return false;
    }

    // 检查域名部分的有效性：不能以 . 开头或结尾，不能有连续的点
    if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return false;
    }

    // 域名部分只能包含字母、数字、点、连字符
    let valid_domain_chars = |c: char| c.is_alphanumeric() || c == '.' || c == '-';
    if !domain.chars().all(valid_domain_chars) {
        return false;
    }

    // 检查是否有有效的 TLD
    const EMAIL_TLDS: [&str; 61] = [
        // 常见国际 TLD
        ".com", ".net", ".org", ".edu", ".gov", ".mil", ".info", ".biz", ".xyz", ".top", ".site",
        ".online", ".club", ".cc", ".tv", ".io", ".ai", ".dev", ".app",
        // 国家代码 TLD
        ".cn", ".jp", ".kr", ".uk", ".us", ".de", ".fr", ".it", ".es", ".ca", ".au", ".sg", ".hk",
        ".tw", ".ru", ".br", ".in", ".mx", ".nl", ".ch", ".se", ".no", ".dk", ".nz", ".za", ".tw",
        ".th", ".vn", ".ph", ".id", ".my", ".co", ".me", ".be", ".gr",
        // 企业/特殊
        ".company", ".ltd", ".inc", ".corp", ".shop", ".bank", ".qq",
    ];

    let lower = domain.to_lowercase();
    EMAIL_TLDS.iter().any(|tld| lower.ends_with(tld))
}
// #[tauri::command]
// #[cfg(target_os = "windows")]
// pub fn is_foreground_fullscreen() -> bool {
//     use windows::Win32::Foundation::RECT;
//     use windows::Win32::UI::WindowsAndMessaging::{
//         GetDesktopWindow, GetForegroundWindow, GetShellWindow, GetWindowRect,
//     };

//     unsafe {
//         let hwnd = GetForegroundWindow();
//         let desktop = GetDesktopWindow();
//         let shell = GetShellWindow();

//         // 排除桌面和 shell 本身
//         if hwnd == desktop || hwnd == shell {
//             return false;
//         }

//         let mut app_rect = RECT::default();
//         GetWindowRect(hwnd, &mut app_rect).ok();

//         // 获取该窗口所在显示器的尺寸
//         use windows::Win32::Graphics::Gdi::{
//             GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
//         };
//         let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
//         let mut monitor_info = MONITORINFO {
//             cbSize: std::mem::size_of::<MONITORINFO>() as u32,
//             ..Default::default()
//         };
//         GetMonitorInfoW(monitor, &mut monitor_info);

//         let screen = monitor_info.rcMonitor;

//         // 判断窗口是否覆盖整个显示器
//         app_rect.left == screen.left
//             && app_rect.top == screen.top
//             && app_rect.right == screen.right
//             && app_rect.bottom == screen.bottom
//     }
// }
#[tauri::command]
#[cfg(target_os = "windows")]
pub fn is_foreground_fullscreen() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        let desktop = GetDesktopWindow();
        let shell = GetShellWindow();

        if hwnd == desktop || hwnd == shell {
            return false;
        }

        // 获取鼠标当前位置
        let mut cursor_pos = POINT::default();
        GetCursorPos(&mut cursor_pos);

        // 通过鼠标位置找到对应的显示器
        let cursor_monitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);

        // 获取前台窗口所在的显示器
        let mut app_rect = RECT::default();
        GetWindowRect(hwnd, &mut app_rect).ok();
        let window_monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);

        // 前台窗口不在鼠标所在的显示器上，直接返回 false
        if cursor_monitor != window_monitor {
            return false;
        }

        // 获取鼠标所在显示器的尺寸
        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        GetMonitorInfoW(cursor_monitor, &mut monitor_info);

        let screen = monitor_info.rcMonitor;

        // 判断窗口是否覆盖整个显示器
        app_rect.left == screen.left
            && app_rect.top == screen.top
            && app_rect.right == screen.right
            && app_rect.bottom == screen.bottom
    }
}
