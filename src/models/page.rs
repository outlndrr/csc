use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub url: String,
    pub time_zone: String,
    pub updated_at: String
}
