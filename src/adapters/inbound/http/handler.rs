use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    application::{job_request::CreateJobRequest, job_service::JobService},
    domain::job::Job,
    error::AppError,
};

pub type AppState = Arc<JobService>;

pub async fn create_job(
    State(service): State<AppState>,
    Json(request): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<Job>), AppError> {
    let job = service.create_job(request).await?;

    Ok((StatusCode::CREATED, Json(job)))
}
