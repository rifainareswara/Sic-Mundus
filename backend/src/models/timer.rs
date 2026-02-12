use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveTimer {
    pub id: String,
    pub task_id: String,
    pub task_title: String,
    pub start_time: String,
    pub notes: String,
    pub elapsed_seconds: i64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct StartTimerRequest {
    #[serde(default)]
    pub notes: Option<String>,
}
