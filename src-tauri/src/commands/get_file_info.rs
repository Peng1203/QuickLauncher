use crate::models::file_info::FileInfo;
use encoding_rs::GBK;
use lnk_parser::LNKParser;
use std::fs;
use std::path::Path;
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

    let mut full_path: String;
    // 检查路径是否是 lnk 文件
    if p.extension().map_or(false, |ext| ext == "lnk") {
        log::info!("if");
        // 解析 lnk 文件 并获取真正的路径
        let lnk_info = LNKParser::from_path(&path).unwrap();
        full_path = lnk_info
            .get_target_full_path()
            .as_deref()
            .unwrap()
            .to_string();

        // 解析出来的带中文的路径会存在乱码 转换为正常的内容
        let raw_bytes: Vec<u8> = full_path.chars().map(|c| c as u8).collect();
        let (fixed, _, _) = GBK.decode(&raw_bytes);
        full_path = fixed.into_owned();
    } else {
        log::info!("else");
        full_path = p.to_str().unwrap_or_default().to_string();
    }

    log::info!("full_path --->{}", full_path);

    // 获取文件或目录名称
    let name = get_name(p);

    // 提取扩展名（目录没有扩展名）
    let extension = p.extension().map(|ext| ext.to_string_lossy().to_string());

    // dbg!(&extension.unwrap());

    let base64 = get_icon_base64_by_path(&full_path).unwrap();
    let icon = "data:image/png;base64,".to_string() + &base64;

    Ok(FileInfo {
        name,
        path: full_path,
        extension,
        icon,
        size: metadata.len(),
        r#type: if metadata.is_file() {
            "file".to_string()
        } else {
            "directory".to_string()
        },
    })
}
