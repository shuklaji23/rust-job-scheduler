use async_trait::async_trait;

use crate::{application::ports::job_executor::JobExecutor, domain::job::Job, error::AppResult};

pub struct SimpleJobExecutor;

#[async_trait]
impl JobExecutor for SimpleJobExecutor {
    async fn execute(&self, job: &Job) -> AppResult<()> {
        println!("Executing job: {} | payload: {}", job.name, job.payload);

        Ok(())
    }
}
