use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::job::{Job, JobStatus};

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub name: String,
    pub payload: String,
    pub scheduled_at: DateTime<Utc>,
    pub status: JobStatus,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
}

impl From<Job> for JobResponse {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            name: job.name,
            payload: job.payload,
            scheduled_at: job.scheduled_at,
            status: job.status,
            retry_count: job.retry_count,
            created_at: job.created_at,
        }
    }
}
