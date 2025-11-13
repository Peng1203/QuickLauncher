use crate::models::launch_item::LaunchItem;
use pinyin::ToPinyin;
use std::{os::windows::process::CommandExt, process::Command};

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
