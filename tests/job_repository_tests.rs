use chrono::{Duration, Timelike, Utc};
use uuid::Uuid;

use job_scheduler::{
    adapters::outbound::postgres::job_repository_impl::PostgresJobRepository,
    application::ports::job_repository::JobRepository,
    domain::job::{Job, JobStatus},
    infrastructure::database::{create_pool, run_migrations},
};

async fn setup() -> PostgresJobRepository {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    PostgresJobRepository::new(pool)
}

fn truncate_to_microseconds(dt: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    dt.with_nanosecond(dt.timestamp_subsec_nanos() / 1_000 * 1_000)
        .unwrap()
}

fn create_test_job(name: &str, scheduled_at: chrono::DateTime<Utc>) -> Job {
    Job::new(
        name.to_string(),
        r#"{"message":"integration test"}"#.to_string(),
        scheduled_at,
    )
}

#[tokio::test]
async fn saves_and_finds_job() {
    let repository = setup().await;

    let job = create_test_job("save-find-test", Utc::now() + Duration::minutes(5));

    repository.save(&job).await.unwrap();

    let result = repository.find_by_id(job.id).await.unwrap();
    let found_job = result.expect("Job should exist");

    assert_eq!(found_job.id, job.id);
    assert_eq!(found_job.name, job.name);
    assert_eq!(found_job.payload, job.payload);
    assert_eq!(
        found_job.scheduled_at,
        truncate_to_microseconds(job.scheduled_at)
    );
    assert_eq!(found_job.status, JobStatus::Scheduled);
    assert_eq!(found_job.retry_count, 0);
    assert_eq!(
        found_job.created_at,
        truncate_to_microseconds(job.created_at)
    );

    repository.delete(job.id).await.unwrap();
}

#[tokio::test]
async fn returns_none_when_job_does_not_exist() {
    let repository = setup().await;

    let result = repository.find_by_id(Uuid::new_v4()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn finds_due_scheduled_jobs() {
    let repository = setup().await;

    let due_job = create_test_job("due-job", Utc::now() - Duration::minutes(5));

    let future_job = create_test_job("future-job", Utc::now() + Duration::minutes(5));

    repository.save(&due_job).await.unwrap();
    repository.save(&future_job).await.unwrap();

    let due_jobs = repository.find_due_jobs().await.unwrap();

    assert!(due_jobs.iter().any(|job| job.id == due_job.id));
    assert!(!due_jobs.iter().any(|job| job.id == future_job.id));

    repository.delete(due_job.id).await.unwrap();
    repository.delete(future_job.id).await.unwrap();
}

#[tokio::test]
async fn does_not_return_non_scheduled_due_job() {
    let repository = setup().await;

    let mut job = create_test_job("completed-job", Utc::now() - Duration::minutes(5));

    repository.save(&job).await.unwrap();

    job.status = JobStatus::Completed;

    repository.update(&job).await.unwrap();

    let due_jobs = repository.find_due_jobs().await.unwrap();

    assert!(!due_jobs.iter().any(|due_job| due_job.id == job.id));

    repository.delete(job.id).await.unwrap();
}

#[tokio::test]
async fn updates_existing_job() {
    let repository = setup().await;

    let mut job = create_test_job("original-name", Utc::now() + Duration::minutes(5));

    repository.save(&job).await.unwrap();

    job.name = "updated-name".to_string();
    job.payload = r#"{"message":"updated"}"#.to_string();
    job.scheduled_at = Utc::now() + Duration::minutes(10);
    job.status = JobStatus::Running;
    job.retry_count = 2;

    repository.update(&job).await.unwrap();

    let updated_job = repository
        .find_by_id(job.id)
        .await
        .unwrap()
        .expect("Job should exist");

    assert_eq!(updated_job.name, "updated-name");
    assert_eq!(updated_job.payload, r#"{"message":"updated"}"#);
    assert_eq!(updated_job.status, JobStatus::Running);
    assert_eq!(updated_job.retry_count, 2);
    assert_eq!(
        updated_job.scheduled_at,
        truncate_to_microseconds(job.scheduled_at)
    );

    repository.delete(job.id).await.unwrap();
}

#[tokio::test]
async fn update_fails_when_job_does_not_exist() {
    let repository = setup().await;

    let job = create_test_job("non-existent-job", Utc::now() + Duration::minutes(5));

    let result = repository.update(&job).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn deletes_existing_job() {
    let repository = setup().await;

    let job = create_test_job("delete-test", Utc::now() + Duration::minutes(5));

    repository.save(&job).await.unwrap();

    repository.delete(job.id).await.unwrap();

    let result = repository.find_by_id(job.id).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn delete_fails_when_job_does_not_exist() {
    let repository = setup().await;

    let result = repository.delete(Uuid::new_v4()).await;

    assert!(result.is_err());
}
