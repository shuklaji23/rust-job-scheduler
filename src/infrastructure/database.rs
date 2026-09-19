use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::error::{AppError, AppResult};

pub async fn create_pool(database_url: &str) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|error| AppError::Repository(error.to_string()))?;

    Ok(())
}
