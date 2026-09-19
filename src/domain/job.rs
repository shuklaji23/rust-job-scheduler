use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub payload: String,
    pub scheduled_at: DateTime<Utc>,
    pub status: JobStatus,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
}

pub enum JobStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
}
