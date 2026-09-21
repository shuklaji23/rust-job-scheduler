use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Repository(String),
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Validation(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Repository(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}
