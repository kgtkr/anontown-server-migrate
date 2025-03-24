use actix_web::{error::ResponseError, HttpResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_message) = match self {
            AppError::Database(e) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::NotFound => (actix_web::http::StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Internal(e) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, e.clone()),
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": error_message,
        }))
    }
} 