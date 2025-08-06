use crate::models::file_info::FileInfo;
use lnk_parser::LNKParser;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use windows_icons::get_icon_base64_by_path;

// 获取文件名/目录名
fn get_name(p: &Path) -> String {
    if p.is_file() {
        p.file_stem()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| "未知".into())
            .to_string()
    } else {
        p.file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| "未知".into())
            .to_string()
    }
}

#[tauri::command]
pub fn get_file_info(path: String) -> Result<FileInfo, String> {
    let p = Path::new(&path);
    let metadata = fs::metadata(&p).map_err(|e| e.to_string())?;

    let full_path: String;
    // 检查路径是否是 lnk 文件
    if p.extension().map_or(false, |ext| ext == "lnk") {
        // 解析 lnk 文件 并获取真正的路径
        let lnk_info = LNKParser::from_path(&path).unwrap();
        full_path = lnk_info
            .get_target_full_path()
            .as_deref()
            .unwrap()
            .to_string();
    } else {
        full_path = p.to_str().unwrap_or_default().to_string();
    }
    // let created = metadata
    //     .created()
    //     .ok()
    //     .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
    //     .map(|d| d.as_secs());

    // let modified = metadata
    //     .modified()
    //     .ok()
    //     .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
    //     .map(|d| d.as_secs());

    // 获取文件或目录名称
    let name = get_name(p);

    let base64 = get_icon_base64_by_path(&full_path).unwrap();
    let icon = "data:image/png;base64,".to_string() + &base64;

    Ok(FileInfo {
        name,
        path: full_path,
        icon,
        size: metadata.len(),
        r#type: if metadata.is_file() {
            "file".to_string()
        } else {
            "directory".to_string()
        },
    })
}
