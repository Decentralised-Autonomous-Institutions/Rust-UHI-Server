use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Authorization error: {0}")]
    ForbiddenError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_code) = match self {
            AppError::ValidationError(_) => (actix_web::http::StatusCode::BAD_REQUEST, "INVALID_INPUT"),
            AppError::NotFoundError(_) => (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::AuthError(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            AppError::ForbiddenError(_) => (actix_web::http::StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::DatabaseError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            AppError::ConfigError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "CONFIG_ERROR"),
            AppError::InternalError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            AppError::ExternalServiceError(_) => (actix_web::http::StatusCode::BAD_GATEWAY, "EXTERNAL_SERVICE_ERROR"),
        };

        let error_response = ErrorResponse {
            error: format!("{:?}", status_code),
            message: self.to_string(),
            code: error_code.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
} 