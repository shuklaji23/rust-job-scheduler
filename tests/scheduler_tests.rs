use std::sync::{Arc, Mutex};

use chrono::{Duration, Utc};
use job_scheduler::{
    application::{
        ports::{job_executor::JobExecutor, job_repository::JobRepository},
        scheduler::Scheduler,
    },
    domain::job::Job,
    error::AppResult,
};
use uuid::Uuid;

struct MockRepository {
    jobs: Vec<Job>,
}

impl MockRepository {
    fn new(jobs: Vec<Job>) -> Self {
        Self { jobs }
    }
}

impl JobRepository for MockRepository {
    async fn save(&self, _job: &Job) -> AppResult<()> {
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Job>> {
        Ok(self.jobs.iter().find(|job| job.id == id).map(|job| Job {
            id: job.id,
            name: job.name.clone(),
            payload: job.payload.clone(),
            scheduled_at: job.scheduled_at,
            status: job.status.clone(),
            retry_count: job.retry_count,
            created_at: job.created_at,
        }))
    }

    async fn find_due_jobs(&self) -> AppResult<Vec<Job>> {
        Ok(self
            .jobs
            .iter()
            .map(|job| Job {
                id: job.id,
                name: job.name.clone(),
                payload: job.payload.clone(),
                scheduled_at: job.scheduled_at,
                status: job.status.clone(),
                retry_count: job.retry_count,
                created_at: job.created_at,
            })
            .collect())
    }

    async fn update(&self, _job: &Job) -> AppResult<()> {
        Ok(())
    }

    async fn delete(&self, _id: Uuid) -> AppResult<()> {
        Ok(())
    }
}

struct MockExecutor {
    executed_jobs: Arc<Mutex<Vec<Uuid>>>,
}

impl MockExecutor {
    fn new() -> Self {
        Self {
            executed_jobs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl JobExecutor for MockExecutor {
    async fn execute(&self, job: &Job) -> AppResult<()> {
        self.executed_jobs.lock().unwrap().push(job.id);
        Ok(())
    }
}

#[tokio::test]
async fn run_once_executes_due_jobs() {
    let job = Job::new(
        "test-job".to_string(),
        r#"{"key":"value"}"#.to_string(),
        Utc::now() - Duration::minutes(1),
    );

    let job_id = job.id;

    let repository = MockRepository::new(vec![job]);
    let executor = MockExecutor::new();

    let executed_jobs = Arc::clone(&executor.executed_jobs);

    let scheduler = Scheduler::new(repository, executor);

    scheduler.run_once().await.unwrap();

    let executed = executed_jobs.lock().unwrap();

    assert_eq!(executed.len(), 1);
    assert_eq!(executed[0], job_id);
}
