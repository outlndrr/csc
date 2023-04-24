use serde::Deserialize;

use super::{component::Component, incident::Incident};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ScheduledMaintenance {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub monitoring_at: Option<String>,
    pub resolved_at: Option<String>,
    pub impact: String,
    pub shortlink: String,
    pub started_at: String,
    pub page_id: String,
    pub incident_updates: Vec<Incident>,
    pub components: Vec<Component>,
    pub scheduled_for: String,
    pub scheduled_until: String
}