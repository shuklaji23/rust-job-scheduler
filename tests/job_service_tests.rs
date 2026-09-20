use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use job_scheduler::{
    application::{
        job_request::CreateJobRequest, job_service::JobService,
        ports::job_repository::JobRepository,
    },
    domain::job::{Job, JobStatus},
    error::{AppError, AppResult},
};

struct MockJobRepository {
    jobs: Mutex<Vec<Job>>,
}

impl MockJobRepository {
    fn new() -> Self {
        Self {
            jobs: Mutex::new(Vec::new()),
        }
    }
}

fn clone_job(job: &Job) -> Job {
    Job {
        id: job.id,
        name: job.name.clone(),
        payload: job.payload.clone(),
        scheduled_at: job.scheduled_at,
        status: job.status.clone(),
        retry_count: job.retry_count,
        created_at: job.created_at,
    }
}

impl JobRepository for MockJobRepository {
    async fn save(&self, job: &Job) -> AppResult<()> {
        self.jobs.lock().unwrap().push(Job {
            id: job.id,
            name: job.name.clone(),
            payload: job.payload.clone(),
            scheduled_at: job.scheduled_at,
            status: job.status.clone(),
            retry_count: job.retry_count,
            created_at: job.created_at,
        });

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Job>> {
        let jobs = self.jobs.lock().unwrap();

        Ok(jobs.iter().find(|job| job.id == id).map(clone_job))
    }

    async fn find_due_jobs(&self) -> AppResult<Vec<Job>> {
        let now = Utc::now();
        let jobs = self.jobs.lock().unwrap();

        Ok(jobs
            .iter()
            .filter(|job| job.scheduled_at <= now && job.status == JobStatus::Scheduled)
            .map(clone_job)
            .collect())
    }

    async fn update(&self, job: &Job) -> AppResult<()> {
        let mut jobs = self.jobs.lock().unwrap();

        let existing = jobs.iter_mut().find(|existing| existing.id == job.id);

        match existing {
            Some(existing) => {
                *existing = clone_job(job);
                Ok(())
            }
            None => Err(AppError::Repository("Job not found".to_string())),
        }
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut jobs = self.jobs.lock().unwrap();

        let original_len = jobs.len();

        jobs.retain(|job| job.id != id);

        if jobs.len() == original_len {
            return Err(AppError::Repository("Job not found".to_string()));
        }

        Ok(())
    }
}

#[tokio::test]
async fn create_job_creates_and_saves_job() {
    let repository = MockJobRepository::new();
    let service = JobService::new(repository);

    let scheduled_at = Utc::now();
    let temp = CreateJobRequest {
        name: "test-job".to_string(),
        payload: serde_json::value::Value::String(r#"{"message":"hello"}"#.to_string()),
        scheduled_at,
    };
    let job = service.create_job(temp).await.unwrap();

    assert_eq!(job.name, "test-job");
    assert_eq!(job.payload, r#"{"message":"hello"}"#);
    assert_eq!(job.scheduled_at, scheduled_at);
    assert_eq!(job.retry_count, 0);
}

#[tokio::test]
async fn get_job_returns_existing_job() {
    let repository = MockJobRepository::new();

    let job = Job::new(
        "test-job".to_string(),
        r#"{"message":"hello"}"#.to_string(),
        Utc::now(),
    );

    repository.save(&job).await.unwrap();

    let service = JobService::new(repository);

    let result = service.get_job(job.id).await.unwrap();

    let found = result.expect("Job should exist");

    assert_eq!(found.id, job.id);
    assert_eq!(found.name, job.name);
}

#[tokio::test]
async fn get_job_returns_none_for_missing_job() {
    let repository = MockJobRepository::new();
    let service = JobService::new(repository);

    let result = service.get_job(Uuid::new_v4()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn get_due_jobs_returns_only_due_jobs() {
    let repository = MockJobRepository::new();

    let due_job = Job::new(
        "due-job".to_string(),
        r#"{"message":"due"}"#.to_string(),
        Utc::now() - chrono::Duration::minutes(5),
    );

    let future_job = Job::new(
        "future-job".to_string(),
        r#"{"message":"future"}"#.to_string(),
        Utc::now() + chrono::Duration::minutes(5),
    );

    repository.save(&due_job).await.unwrap();
    repository.save(&future_job).await.unwrap();

    let service = JobService::new(repository);

    let due_jobs = service.get_due_jobs().await.unwrap();

    assert_eq!(due_jobs.len(), 1);
    assert_eq!(due_jobs[0].id, due_job.id);
}

#[tokio::test]
async fn update_job_updates_existing_job() {
    let repository = MockJobRepository::new();

    let job = Job::new(
        "original".to_string(),
        r#"{"message":"hello"}"#.to_string(),
        Utc::now(),
    );

    repository.save(&job).await.unwrap();

    let service = JobService::new(repository);

    let mut updated_job = clone_job(&job);
    updated_job.name = "updated".to_string();

    service.update_job(&updated_job).await.unwrap();

    let result = service.get_job(job.id).await.unwrap().unwrap();

    assert_eq!(result.name, "updated");
}

#[tokio::test]
async fn delete_job_removes_existing_job() {
    let repository = MockJobRepository::new();

    let job = Job::new(
        "test-job".to_string(),
        r#"{"message":"hello"}"#.to_string(),
        Utc::now(),
    );

    repository.save(&job).await.unwrap();

    let service = JobService::new(repository);

    service.delete_job(job.id).await.unwrap();

    let result = service.get_job(job.id).await.unwrap();

    assert!(result.is_none());
}
