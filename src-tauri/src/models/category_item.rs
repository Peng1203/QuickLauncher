use rusqlite::{Result, Row};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryItem {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub association_directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl CategoryItem {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            association_directory: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewCategoryItem {
    pub name: String,
    pub parent_id: i32,
    pub association_directory: String,
}
