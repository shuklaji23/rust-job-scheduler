use std::sync::Arc;

use crate::{
    application::ports::{job_executor::JobExecutor, job_repository::JobRepository},
    domain::job::JobStatus,
    error::AppResult,
};

pub struct Scheduler {
    repository: Arc<dyn JobRepository>,
    executor: Arc<dyn JobExecutor>,
}

impl Scheduler {
    pub fn new(repository: Arc<dyn JobRepository>, executor: Arc<dyn JobExecutor>) -> Self {
        Self {
            repository,
            executor,
        }
    }

    pub async fn run_once(&self) -> AppResult<()> {
        let mut jobs = self.repository.find_due_jobs().await?;

        for job in &mut jobs {
            job.status = JobStatus::Running;
            self.repository.update(job).await?;

            match self.executor.execute(job).await {
                Ok(()) => {
                    job.status = JobStatus::Completed;
                }
                Err(error) => {
                    job.status = JobStatus::Failed;
                    self.repository.update(job).await?;

                    return Err(error);
                }
            }
            self.repository.update(job).await?;
        }

        Ok(())
    }
}
