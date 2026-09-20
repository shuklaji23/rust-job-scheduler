use job_scheduler::infrastructure::database::{create_pool, run_migrations};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    println!("Database initialized successfully");
}
