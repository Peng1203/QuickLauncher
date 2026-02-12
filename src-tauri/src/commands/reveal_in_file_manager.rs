use std::env::consts::OS;
use std::path::PathBuf;
use std::process::Command;

#[tauri::command]
pub fn reveal_in_file_manager(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);

    if !path.exists() {
        log::error!("路径不存在{}", path.to_string_lossy().to_string());
        return Err("路径不存在".into());
    }

    let status = match OS {
        "windows" => Command::new("explorer").arg("/select,").arg(path).status(),
        "macos" => Command::new("open").arg(path).status(),
        "linux" => Command::new("xdg-open").arg(path).status(),
        _ => return Err("不支持的操作系统".into()),
    };
    match status {
        Ok(s) if s.success() => Ok(()),
        // Windows 特殊处理，exit code 1 也视为成功
        Ok(s) if s.code() == Some(1) => Ok(()),
        Ok(s) => {
            log::error!("命令执行失败，状态码: {}", s);
            Err(format!("命令执行失败，状态码: {}", s))
        }
        Err(e) => {
            log::error!("无法启动打开命令: {}", e);
            return Err(format!("无法启动打开命令: {}", e));
        }
    }
}
