use crate::{domain::job::Job, error::AppResult};

#[allow(async_fn_in_trait)]
pub trait JobExecutor: Send + Sync {
    async fn execute(&self, job: &Job) -> AppResult<()>;
}
