use crate::models::file_info::FileInfo;
use lnk_parser::LNKParser;
use std::fs::{self};
use std::path::{Path, PathBuf};
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

// 解析 lnk 文件 并获取真正的路径
fn get_lnk_real_path(path: &String) -> Option<PathBuf> {
    let info = LNKParser::from_path(path).unwrap();

    let mut relative_path = "";
    if let Some(data) = info.get_relative_path() {
        relative_path = data.string.as_str();
    }

    let mut work_dir = "";
    if let Some(data) = info.get_working_dir() {
        work_dir = data.string.as_str();
    }
    log::info!("-----------------------------------------------------------------------");
    log::info!("path {}", path);
    log::info!("relative_path {}", relative_path);
    log::info!("work_dir {}", work_dir);
    log::info!(
        "get_icon_location {}",
        info.get_icon_location().as_ref().unwrap()
    );
    log::info!(
        "get_name_string {}",
        info.get_name_string().as_ref().unwrap()
    );
    log::info!(
        "get_lnk_file_metadata {}",
        info.get_lnk_file_metadata()
            .as_ref()
            .unwrap()
            .get_full_path()
    );
    log::info!(
        "get_name_string {}",
        info.get_name_string().as_ref().unwrap()
    );
    log::info!(
        "get_name_string {}",
        info.get_name_string().as_ref().unwrap()
    );

    // 从 data_with_file 提取最后的文件名（OsStr）
    let file_name = Path::new(relative_path).file_name()?;
    // 将 base 转为 PathBuf 并 push 文件名（会自动处理分隔符）
    let mut full = PathBuf::from(work_dir);
    full.push(file_name);
    // if (work_dir.is_empty()) {
    //     if let Some(data) = info.get_link_info() {
    //         Some(&data.local_base_path)
    //     }
    // }
    dbg!(&full);
    Some(full)
}

#[tauri::command]
pub fn get_file_info(path: String) -> Result<FileInfo, String> {
    let p = Path::new(&path);
    let metadata = fs::metadata(&p).map_err(|e| e.to_string())?;

    let full_path: String;
    // 检查路径是否是 lnk 文件
    if p.extension().map_or(false, |ext| ext == "lnk") {
        // 解析 lnk 文件 并获取真正的路径
        // let lnk_info = LNKParser::from_path(&path).unwrap();

        match get_lnk_real_path(&path) {
            Some(f_path) => full_path = f_path.to_string_lossy().into_owned(),
            None => full_path = "".to_string(),
        }

        log::info!("full_path {}", full_path);

        // if let Some(f_path) = get_lnk_real_path(&path) {
        //     full_path = f_path.to_string_lossy().into_owned();
        // } else {
        //     full_path = "".to_string();
        // }
    } else {
        full_path = p.to_str().unwrap_or_default().to_string();
    }

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
