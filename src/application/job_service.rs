use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        job_request::CreateJobRequest,
        ports::{job_executor::JobExecutor, job_repository::JobRepository},
    },
    domain::job::{Job, JobStatus},
    error::AppResult,
};

pub struct JobService {
    repository: Arc<dyn JobRepository>,
    executor: Arc<dyn JobExecutor>,
}

impl JobService {
    pub fn new(repository: Arc<dyn JobRepository>, executor: Arc<dyn JobExecutor>) -> Self {
        Self {
            repository,
            executor,
        }
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

    pub async fn execute_due_jobs(&self) -> AppResult<()> {
        let mut jobs = self.repository.find_due_jobs().await?;

        for job in &mut jobs {
            job.status = JobStatus::Running;
            self.repository.update(&job).await?;

            match self.executor.execute(&job).await {
                Ok(()) => {
                    job.status = JobStatus::Completed;
                }
                Err(error) => {
                    job.status = JobStatus::Failed;
                    self.repository.update(&job).await?;
                    return Err(error);
                }
            }

            self.repository.update(&job).await?;
        }

        Ok(())
    }
}
