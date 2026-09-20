use uuid::Uuid;

use crate::{
    application::{job_request::CreateJobRequest, ports::job_repository::JobRepository},
    domain::job::Job,
    error::AppResult,
};

pub struct JobService<R>
where
    R: JobRepository,
{
    repository: R,
}

impl<R> JobService<R>
where
    R: JobRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_job(&self, request: CreateJobRequest) -> AppResult<Job> {
        let job = Job::try_from(request)?;

        self.repository.save(&job).await?;

        Ok(job)
    }

    pub async fn get_job(&self, id: Uuid) -> AppResult<Option<Job>> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_due_jobs(&self) -> AppResult<Vec<Job>> {
        self.repository.find_due_jobs().await
    }

    pub async fn update_job(&self, job: &Job) -> AppResult<()> {
        self.repository.update(job).await
    }

    pub async fn delete_job(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
