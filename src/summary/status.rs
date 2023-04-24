use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Status {
    pub indicator: String,
    pub description: String
}