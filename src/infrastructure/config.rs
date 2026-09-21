use crate::error::{AppError, AppResult};

pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| AppError::Internal("DATABASE_URL must be set".to_string()))?;

        let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let server_port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| AppError::Internal("SERVER_PORT must be a valid port".to_string()))?;

        Ok(Self {
            database_url,
            server_host,
            server_port,
        })
    }
}
