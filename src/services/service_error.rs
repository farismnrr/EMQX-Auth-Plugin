use thiserror::Error;
use serde::Serialize;
use crate::repositories::repository_error::UserRepositoryError;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] UserRepositoryError),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Bad request")]
    BadRequest(Vec<ValidationError>),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    #[error("User is not active or deleted: {0}")]
    UserNotActive(String),

    #[error("JWT error: {0}")]
    JwtError(String),
}