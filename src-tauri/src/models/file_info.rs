#[derive(serde::Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub icon: String,
    pub extension: Option<String>,
    // pub created: Option<u64>,
    // pub modified: Option<u64>,
    pub r#type: String, // "file" or "directory" or "url"
}
