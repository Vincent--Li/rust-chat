use axum::{http::{header::InvalidHeaderValue, StatusCode}, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("invalid credentials: {0}")]
    InvalidCredentials(String),

    #[error("http header error: {0}")]
    HttpHeaderError(#[from] InvalidHeaderValue),

    #[error("email already exists: {0}")]
    EmailAlreadyExists(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            AppError::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
        };

        (status, Json(json!({ "error" : self.to_string()}))).into_response()
    }
}
