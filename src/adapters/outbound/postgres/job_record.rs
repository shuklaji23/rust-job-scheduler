use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    domain::job::{Job, JobStatus},
    error::{AppError, AppResult},
};

#[derive(Debug, FromRow)]
pub struct JobRecord {
    pub id: Uuid,
    pub name: String,
    pub payload: Value,
    pub scheduled_at: DateTime<Utc>,
    pub status: String,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Job> for JobRecord {
    type Error = AppError;

    fn try_from(job: &Job) -> AppResult<Self> {
        let payload = serde_json::from_str(&job.payload)
            .map_err(|error| AppError::Validation(error.to_string()))?;

        let status = match &job.status {
            JobStatus::Scheduled => "Scheduled",
            JobStatus::Running => "Running",
            JobStatus::Completed => "Completed",
            JobStatus::Failed => "Failed",
        };

        Ok(Self {
            id: job.id,
            name: job.name.clone(),
            payload,
            scheduled_at: job.scheduled_at,
            status: status.to_string(),
            retry_count: job.retry_count as i32,
            created_at: job.created_at,
        })
    }
}
