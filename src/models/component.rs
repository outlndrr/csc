use std::fmt::Display;

use time::{PrimitiveDateTime, macros::format_description, format_description::well_known::Rfc3339};
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
    #[serde(rename = "under_maintenance")]
    UnderMaintenance,
    #[default]
    Unknown
}

impl Display for ComponentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentStatus::Operational => write!(f, "\x1b[32mOperational\x1b[0m"),
            ComponentStatus::PartialOutage => write!(f, "\x1b[93mRe-routed\x1b[0m"),
            ComponentStatus::UnderMaintenance => write!(f, "\x1b[36mPartially Re-routed\x1b[0m"),
            ComponentStatus::Unknown => write!(f, "Unknown")
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current status for \x1b[1m{}\x1b[0m", self.name)?;
        writeln!(f, " is {}.", self.status)?;

        let date = PrimitiveDateTime::parse(&self.updated_at, &Rfc3339).unwrap();
        write!(f, "Last update was at {}", date.format(format_description!("[day].[month].[year] [hour]:[minute]")).unwrap())
    }
}
