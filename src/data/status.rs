use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Paused,
    Done,
}
