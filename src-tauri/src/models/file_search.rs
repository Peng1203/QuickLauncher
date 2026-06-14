use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FileSearchResult {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub icon: String,
    pub r#type: String,
    pub extension: Option<String>,
}
