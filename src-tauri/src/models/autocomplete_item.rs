use serde::Serialize;

#[derive(Serialize)]
pub struct AutocompleteItem {
    pub id: i64,
    pub query: String,
    pub usage_count: i32,
    pub last_used_at: String,
    pub launch_item_id: Option<i64>,
}
