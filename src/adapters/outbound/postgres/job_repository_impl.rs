use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    adapters::outbound::postgres::job_record::JobRecord,
    application::ports::job_repository::JobRepository,
    domain::job::{Job, JobStatus},
    error::{AppError, AppResult},
};

pub struct PostgresJobRepository {
    pool: PgPool,
}

impl PostgresJobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobRepository for PostgresJobRepository {
    async fn save(&self, job: &Job) -> AppResult<()> {
        let record = JobRecord::try_from(job)?;

        // r# is used because For SQL queries, raw strings are convenient
        // as we can write multiline SQL directly without worrying about escaping characters.
        sqlx::query(
            r#"
        INSERT INTO jobs (
            id,
            name,
            payload,
            scheduled_at,
            status,
            retry_count,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        )
        .bind(record.id)
        .bind(record.name)
        .bind(record.payload)
        .bind(record.scheduled_at)
        .bind(record.status)
        .bind(record.retry_count)
        .bind(record.created_at)
        .execute(&self.pool)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

        Ok(())
    }

    // Here 'SELECT *' is not used beacuse in future if table is modified,
    // the query will return an additional column. Depending on how the mapping is handled,
    // this can create unnecessary coupling or mapping issues.
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Job>> {
        let record = sqlx::query_as::<_, JobRecord>(
            r#"
        SELECT
            id,
            name,
            payload,
            scheduled_at,
            status,
            retry_count,
            created_at
        FROM jobs
        WHERE id = $1
        "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

        record.map(Job::try_from).transpose()
    }

    async fn find_due_jobs(&self) -> AppResult<Vec<Job>> {
        let records = sqlx::query_as::<_, JobRecord>(
            r#"
        SELECT
            id,
            name,
            payload,
            scheduled_at,
            status,
            retry_count,
            created_at
        FROM jobs
        WHERE scheduled_at <= $1
          AND status = $2
        ORDER BY scheduled_at ASC
        "#,
        )
        .bind(Utc::now())
        .bind("Scheduled")
        .fetch_all(&self.pool)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

        records.into_iter().map(Job::try_from).collect()
    }

    async fn update(&self, job: &Job) -> AppResult<()> {
        let record = JobRecord::try_from(job)?;

        let result = sqlx::query(
            r#"
        UPDATE jobs
        SET
            name = $1,
            payload = $2,
            scheduled_at = $3,
            status = $4,
            retry_count = $5
        WHERE id = $6
        "#,
        )
        .bind(record.name)
        .bind(record.payload)
        .bind(record.scheduled_at)
        .bind(record.status)
        .bind(record.retry_count)
        .bind(record.id)
        .execute(&self.pool)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::Repository(format!(
                "Job not found: {}",
                record.id
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query(
            r#"
        DELETE FROM jobs
        WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::Repository(format!("Job not found: {id}")));
        }

        Ok(())
    }
}

impl TryFrom<JobRecord> for Job {
    type Error = AppError;

    fn try_from(record: JobRecord) -> Result<Self, Self::Error> {
        let status = match record.status.as_str() {
            "Scheduled" => JobStatus::Scheduled,
            "Running" => JobStatus::Running,
            "Completed" => JobStatus::Completed,
            "Failed" => JobStatus::Failed,
            status => {
                return Err(AppError::Repository(format!(
                    "Invalid job status: {status}"
                )));
            }
        };

        Ok(Job {
            id: record.id,
            name: record.name,
            payload: record.payload.to_string(),
            scheduled_at: record.scheduled_at,
            status,
            retry_count: record.retry_count as u32,
            created_at: record.created_at,
        })
    }
}
