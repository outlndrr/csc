use serde::{Deserialize};

use self::{page::Page, component::Component, incident::Incident, scheduled_maintenances::ScheduledMaintenance, status::Status};

pub mod page;
pub mod status;
pub mod scheduled_maintenances;
pub mod component;
pub mod incident;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Summary {
    pub page: Page,
    pub components: Vec<Component>,
    pub incidents: Option<Vec<Incident>>,
    pub scheduled_maintenances: Option<Vec<ScheduledMaintenance>>,
    pub status: Status
}