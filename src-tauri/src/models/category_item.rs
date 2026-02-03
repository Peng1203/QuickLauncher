use rusqlite::{Result, Row};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryItem {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub association_directory: Option<String>,
    pub exclude: Option<i32>,
    pub layout: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl CategoryItem {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            parent_id: row.get("parent_id")?,
            association_directory: row.get("association_directory")?,
            exclude: row.get("exclude")?,
            layout: row.get("layout")?,
            icon: row.get("icon")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewCategoryItem {
    pub name: String,
    pub parent_id: i32,
    pub association_directory: String,
    pub exclude: i32,
    pub layout: String,
    pub icon: String,
}
