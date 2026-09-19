use uuid::Uuid;

use crate::{domain::job::Job, error::AppResult};

pub trait JobRepository: Send + Sync {
    async fn save(&self, job: &Job) -> AppResult<()>;

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Job>>;

    async fn find_due_jobs(&self) -> AppResult<Vec<Job>>;

    async fn update(&self, job: &Job) -> AppResult<()>;

    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
