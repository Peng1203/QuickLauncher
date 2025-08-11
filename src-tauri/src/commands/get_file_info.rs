use crate::models::file_info::FileInfo;
use lnk_parser::LNKParser;
use parselnk::Lnk;
use std::fs::{self, File};
use std::path::{Component, Path, PathBuf};
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
fn get_lnk_real_path(lnk_path: &String) -> String {
    let mut open_file = File::open(&lnk_path).unwrap();

    let lnk_file = Lnk::from(Lnk::new(&mut open_file).unwrap());

    let lnk_parser_file = LNKParser::from_reader(&mut open_file).unwrap();

    let right_full_path = merge_relative_and_target(
        lnk_parser_file.get_target_full_path().as_deref(),
        lnk_file.relative_path().as_ref().and_then(|p| p.to_str()),
    );

    return right_full_path
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .to_string();
}

fn merge_relative_and_target(
    absolute_maybe_mojibake: Option<&str>,
    relative: Option<&str>,
) -> Option<PathBuf> {
    let rel = relative?;
    let abs = absolute_maybe_mojibake?;

    let rel_path = Path::new(rel);

    // 如果相对路径本身已经是绝对路径，直接规范化并返回
    if rel_path.is_absolute() {
        return Some(normalize_path(rel_path.to_path_buf()));
    }

    // 统计开头连续的 ParentDir ("..") 数量，并收集剩下的 tail
    let mut comps = rel_path.components();
    let mut parent_count = 0usize;
    while let Some(c) = comps.next() {
        match c {
            Component::ParentDir => parent_count += 1,
            other => {
                // 第一个非 ParentDir，开始构建 tail（包含当前组件和剩下的所有组件）
                let mut tail = PathBuf::new();
                tail.push(other.as_os_str());
                for c2 in comps {
                    tail.push(c2.as_os_str());
                }

                // 以绝对路径为参照：先去掉文件名（如果有），再 pop parent_count 次
                let mut base = PathBuf::from(abs);
                // 通常 abs 是文件路径（exe），先 pop 一次到目录；如果不是文件也无害
                base.pop();

                for _ in 0..parent_count {
                    // 如果已经到根了，pop 会失败但我们不崩溃
                    if !base.pop() {
                        break;
                    }
                }

                // 把 tail 拼接上去，最后规范化路径
                base.push(tail);
                return Some(normalize_path(base));
            }
        }
    }

    // 如果 rel 只有一堆 ".."（没有其他 tail），则只从 abs 里 pop 对应层级并返回
    let mut base = PathBuf::from(abs);
    base.pop(); // 去掉文件名
    for _ in 0..parent_count {
        if !base.pop() {
            break;
        }
    }
    Some(normalize_path(base))
}

/// 简单规范化：移除 "."，解析 ".."（尽量向上 pop），保留 Windows 前缀/根
fn normalize_path(p: PathBuf) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in p.components() {
        match comp {
            Component::Prefix(pref) => {
                out.push(pref.as_os_str());
            }
            Component::RootDir => {
                out.push(Component::RootDir.as_os_str());
            }
            Component::Normal(s) => {
                out.push(s);
            }
            Component::CurDir => {
                // skip
            }
            Component::ParentDir => {
                if !out.pop() {
                    // 如果无法 pop（已经到根），则把 ".." 作为普通部分保留（极少见）
                    out.push("..");
                }
            }
        }
    }
    out
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
        full_path = get_lnk_real_path(&path);
    } else {
        full_path = p.to_str().unwrap_or_default().to_string();
    }

    println!("和哈哈哈哈哈哈哈哈哈哈 {}", full_path);
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
