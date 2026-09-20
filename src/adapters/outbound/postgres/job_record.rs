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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn converts_job_to_job_record() {
        let job = Job::new(
            "test-job".to_string(),
            r#"{"message":"hello"}"#.to_string(),
            Utc::now() + Duration::minutes(5),
        );

        let record = JobRecord::try_from(&job).unwrap();

        assert_eq!(record.id, job.id);
        assert_eq!(record.name, job.name);
        assert_eq!(record.payload["message"], "hello");
        assert_eq!(record.status, "Scheduled");
        assert_eq!(record.retry_count, 0);
        assert_eq!(record.scheduled_at, job.scheduled_at);
        assert_eq!(record.created_at, job.created_at);
    }

    #[test]
    fn rejects_invalid_json_payload() {
        let job = Job::new(
            "test-job".to_string(),
            "invalid-json".to_string(),
            Utc::now(),
        );

        let result = JobRecord::try_from(&job);

        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::Validation(_) => {}
            error => panic!("Expected validation error, got: {error:?}"),
        }
    }

    #[test]
    fn converts_job_record_to_job() {
        let id = Uuid::new_v4();
        let scheduled_at = Utc::now();
        let created_at = Utc::now();

        let record = JobRecord {
            id,
            name: "test-job".to_string(),
            payload: serde_json::json!({"message": "hello"}),
            scheduled_at,
            status: "Scheduled".to_string(),
            retry_count: 2,
            created_at,
        };

        let job = Job::try_from(record).unwrap();

        assert_eq!(job.id, id);
        assert_eq!(job.name, "test-job");
        assert_eq!(job.payload, r#"{"message":"hello"}"#);
        assert_eq!(job.status, JobStatus::Scheduled);
        assert_eq!(job.retry_count, 2);
        assert_eq!(job.scheduled_at, scheduled_at);
        assert_eq!(job.created_at, created_at);
    }

    #[test]
    fn rejects_invalid_job_status() {
        let record = JobRecord {
            id: Uuid::new_v4(),
            name: "test-job".to_string(),
            payload: serde_json::json!({"message": "hello"}),
            scheduled_at: Utc::now(),
            status: "InvalidStatus".to_string(),
            retry_count: 0,
            created_at: Utc::now(),
        };

        let result = Job::try_from(record);

        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::Repository(_) => {}
            error => panic!("Expected repository error, got: {error:?}"),
        }
    }
}
