use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub payload: String,
    pub scheduled_at: DateTime<Utc>,
    pub status: JobStatus,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum JobStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
}

impl Job {
    pub fn new(name: String, payload: String, scheduled_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            payload,
            scheduled_at,
            status: JobStatus::Scheduled,
            retry_count: 0,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn creates_job_with_default_values() {
        let before = Utc::now();

        let job = Job::new(
            "test-job".to_string(),
            r#"{"key":"value"}"#.to_string(),
            Utc::now() + Duration::minutes(5),
        );

        let after = Utc::now();

        assert_eq!(job.name, "test-job");
        assert_eq!(job.payload, r#"{"key":"value"}"#);
        assert_eq!(job.status, JobStatus::Scheduled);
        assert_eq!(job.retry_count, 0);
        assert!(job.created_at >= before);
        assert!(job.created_at <= after);
    }
}
