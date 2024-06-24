use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub deadline: NaiveDate,
    pub priority: String,
    pub time: f32,
    pub estimated_hours: f32,
}

impl Task {
    pub fn new(
        id: u32,
        name: String,
        status: String,
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
        }
    }
}
