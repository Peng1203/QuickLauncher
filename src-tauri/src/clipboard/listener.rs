use crate::common::utils::is_valid_url;
use arboard::Clipboard;
use serde::Serialize;
use std::path::Path;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Clone)]
pub enum ClipboardContentType {
    Url,
    Directory,
    Unknown,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClipboardPayload {
    pub content: String,
    pub content_type: ClipboardContentType,
}

pub fn start_clipboard_listener(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut clipboard = Clipboard::new().unwrap();
        let mut last = String::new();
        let mut flag = false;

        loop {
            if let Ok(text) = clipboard.get_text() {
                if text != last && flag {
                    last = text;

                    let payload = ClipboardPayload {
                        content: last.clone(),
                        content_type: detect_clipboard_content(&last),
                    };

                    let _ = app.emit("clipboard", payload);
                }
                flag = true
            }

            sleep(Duration::from_millis(300)).await;
        }
    });
}

pub fn detect_clipboard_content(text: &str) -> ClipboardContentType {
    if is_valid_url(text) {
        return ClipboardContentType::Url;
    }

    if is_directory(text) {
        return ClipboardContentType::Directory;
    }

    ClipboardContentType::Unknown
}

fn is_directory(p: &str) -> bool {
    let path = Path::new(p);
    path.exists() && path.is_dir()
}
