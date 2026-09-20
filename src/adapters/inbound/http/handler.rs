use crate::{
    application::{
        job_request::CreateJobRequest, job_service::JobService,
        ports::job_repository::JobRepository,
    },
    error::AppResult,
};
use axum::{Json, extract::State};

pub async fn create_job<R>(
    State(service): State<JobService<R>>,
    Json(request): Json<CreateJobRequest>,
) -> AppResult<Json<crate::domain::job::Job>>
where
    R: JobRepository,
{
    let job = service.create_job(request).await?;

    Ok(Json(job))
}
