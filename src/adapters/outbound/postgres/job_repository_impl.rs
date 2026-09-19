use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    adapters::outbound::postgres::job_record::JobRecord,
    application::ports::job_repository::JobRepository,
    domain::job::{Job, JobStatus},
    error::{AppError, AppResult},
};

pub struct PostgresJobRepository {
    pool: PgPool,
}

impl PostgresJobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl JobRepository for PostgresJobRepository {
    async fn save(&self, job: &Job) -> AppResult<()> {
        todo!()
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> AppResult<Option<Job>> {
        todo!()
    }

    async fn find_due_jobs(&self) -> AppResult<Vec<Job>> {
        todo!()
    }

    async fn update(&self, job: &Job) -> AppResult<()> {
        todo!()
    }

    async fn delete(&self, id: uuid::Uuid) -> AppResult<()> {
        todo!()
    }
}

impl TryFrom<JobRecord> for Job {
    type Error = AppError;

    fn try_from(record: JobRecord) -> Result<Self, Self::Error> {
        let status = match record.status.as_str() {
            "Scheduled" => JobStatus::Scheduled,
            "Running" => JobStatus::Running,
            "Completed" => JobStatus::Completed,
            "Failed" => JobStatus::Failed,
            status => {
                return Err(AppError::Repository(format!(
                    "Invalid job status: {status}"
                )));
            }
        };

        Ok(Job {
            id: record.id,
            name: record.name,
            payload: record.payload.to_string(),
            scheduled_at: record.scheduled_at,
            status,
            retry_count: record.retry_count as u32,
            created_at: record.created_at,
        })
    }
}
