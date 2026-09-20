use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    domain::job::Job,
    error::{AppError, AppResult},
};

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub payload: Value,
    pub scheduled_at: DateTime<Utc>,
}

impl TryFrom<CreateJobRequest> for Job {
    type Error = AppError;

    fn try_from(request: CreateJobRequest) -> AppResult<Self> {
        if request.name.trim().is_empty() {
            return Err(AppError::Validation("Job name cannot be empty".to_string()));
        }

        Ok(Job::new(
            request.name,
            request.payload.to_string(),
            request.scheduled_at,
        ))
    }
}
