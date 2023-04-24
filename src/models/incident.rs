use serde::Deserialize;

use super::component::AffectedComponent;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Incident {
    pub id: String,
    pub status: String,
    pub body: Option<String>,
    pub incident_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub display_at: Option<String>,
    pub affected_components: Option<Vec<AffectedComponent>>,
    pub deliver_notifications: Option<bool>,
    pub custom_tweet: Option<String>,
    pub tweet_id: Option<String>
}