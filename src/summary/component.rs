use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub status: ComponentStatus,
    pub created_at: String,
    pub updated_at: String,
    pub position: u32,
    pub description: Option<String>,
    pub showcase: bool,
    pub start_date: Option<String>,
    pub group_id: Option<String>,
    pub page_id: String,
    pub group: bool,
    pub only_show_if_degraded: bool
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct AffectedComponent {
    pub code: String,
    pub name: String,
    pub old_status: String,
    pub new_status: String
}

#[derive(Debug, Default, Clone, Deserialize)]
pub enum ComponentStatus {
    #[serde(rename = "operational")]
    Operational,
    #[serde(rename = "partial_outage")]
    PartialOutage,
    #[default]
    Unknown
}
