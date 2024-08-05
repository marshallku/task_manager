use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::status::TaskStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub status: TaskStatus,
    pub deadline: NaiveDate,
    pub priority: String,
    pub time: f32,
    pub estimated_hours: f32,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub paused_at: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(
        id: u32,
        name: String,
        status: TaskStatus,
        deadline: NaiveDate,
        priority: String,
        estimated_hours: f32,
    ) -> Task {
        Task {
            id,
            name,
            status,
            deadline,
            priority,
            time: 0.0,
            estimated_hours,
            started_at: None,
            completed_at: None,
            paused_at: None,
        }
    }
}
