#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Repository(String),
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;
