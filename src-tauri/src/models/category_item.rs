use crate::entity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewCategoryItem {
    pub name: String,
    pub parent_id: Option<i32>,
    pub association_directory: Option<String>,
    pub icon: Option<String>,
    pub exclude: bool,
    pub layout: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryItem {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub association_directory: Option<String>,
    pub exclude: bool,
    pub layout: Option<String>,
    pub icon: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<entity::categories::Model> for CategoryItem {
    fn from(model: entity::categories::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            parent_id: model.parent_id,
            association_directory: model.association_directory,
            exclude: model.exclude,
            layout: model.layout,
            icon: model.icon,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
