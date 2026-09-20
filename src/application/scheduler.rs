use crate::{
    application::ports::{job_executor::JobExecutor, job_repository::JobRepository},
    domain::job::JobStatus,
    error::AppResult,
};

pub struct Scheduler<R, E>
where
    R: JobRepository,
    E: JobExecutor,
{
    repository: R,
    executor: E,
}

impl<R, E> Scheduler<R, E>
where
    R: JobRepository,
    E: JobExecutor,
{
    pub fn new(repository: R, executor: E) -> Self {
        Self {
            repository,
            executor,
        }
    }

    pub async fn run_once(&self) -> AppResult<()> {
        let mut jobs = self.repository.find_due_jobs().await?;

        for job in &mut jobs {
            job.status = JobStatus::Running;
            self.repository.update(&job).await?;

            self.executor.execute(&job).await?;

            job.status = JobStatus::Completed;
            self.repository.update(&job).await?;
        }

        Ok(())
    }
}
