#[derive(Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub association_directory: Option<String>,
    pub created_at: String,
}
