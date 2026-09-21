use std::sync::Arc;

use axum::{Router, routing::post};

use job_scheduler::{
    adapters::{
        inbound::http::handler::{AppState, create_job},
        outbound::{
            executor::simple_executor::SimpleJobExecutor,
            postgres::job_repository_impl::PostgresJobRepository,
        },
    },
    application::{
        job_service::JobService,
        ports::{job_executor::JobExecutor, job_repository::JobRepository},
        scheduler::Scheduler,
    },
    infrastructure::{
        config::Config,
        database::{create_pool, run_migrations},
    },
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load configuration");

    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    let repository: Arc<dyn JobRepository> = Arc::new(PostgresJobRepository::new(pool));
    let executor: Arc<dyn JobExecutor> = Arc::new(SimpleJobExecutor);
    let service: AppState = Arc::new(JobService::new(
        Arc::clone(&repository),
        Arc::clone(&executor),
    ));

    let scheduler = Arc::new(Scheduler::new(
        Arc::clone(&repository),
        Arc::clone(&executor),
    ));

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

        loop {
            interval.tick().await;

            if let Err(error) = scheduler.run_once().await {
                eprintln!("Scheduler error: {:?}", error);
            }
        }
    });
    let app = Router::new()
        .route("/jobs", post(create_job))
        .with_state(service);

    let address = format!("{}:{}", config.server_host, config.server_port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind server");

    println!("Server running on http://{}", address);

    axum::serve(listener, app).await.expect("Server failed");
}
