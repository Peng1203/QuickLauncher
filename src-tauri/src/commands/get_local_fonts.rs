use font_kit::source::SystemSource;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FontInfo {
    family: String,
    postscript_name: Option<String>,
}

#[tracing::instrument]
#[tauri::command]
pub fn get_local_fonts() -> Vec<FontInfo> {
    let source = SystemSource::new();

    let families = source.all_families().unwrap_or_default();

    families
        .into_iter()
        .map(|family| FontInfo {
            family,
            postscript_name: None,
        })
        .collect()
}
