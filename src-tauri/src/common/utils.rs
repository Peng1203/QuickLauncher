use crate::models::launch_item::LaunchItem;
use pinyin::ToPinyin;
use std::{os::windows::process::CommandExt, process::Command};
use url::Url;

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
pub fn run_as_admin(launch_item: LaunchItem) -> Result<(), String> {
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
pub fn is_valid_url(input: &str) -> bool {
    // 1️⃣ 能直接解析成功（带协议）
    if Url::parse(input).is_ok() {
        return true;
    }

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

    let lower = input.to_lowercase();

    // 去掉 path / query 再判断后缀
    let domain_part = lower.split('/').next().unwrap_or(&lower);

    COMMON_TLDS.iter().any(|tld| domain_part.ends_with(tld))
}
