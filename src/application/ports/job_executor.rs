use async_trait::async_trait;

use crate::{domain::job::Job, error::AppResult};

#[async_trait]
pub trait JobExecutor: Send + Sync {
    async fn execute(&self, job: &Job) -> AppResult<()>;
}
