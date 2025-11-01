use std::sync::Arc;
use crate::repositories::check_user_active_repository::CheckUserActiveRepository;
use crate::services::service_error::{UserServiceError, ValidationError};
use crate::dtos::user_dto::CheckUserActiveDTO;

pub struct CheckUserActiveService {
    repo: Arc<CheckUserActiveRepository>,
}

impl CheckUserActiveService {
    pub fn new(repo: Arc<CheckUserActiveRepository>) -> Self {
        Self { repo }
    }

    pub fn validate_user(&self, dto: CheckUserActiveDTO) -> Result<bool, UserServiceError> {
        self.user_input_validation(&dto)?;

        let user = self.repo.check_user_active(&dto.username)?;
        if user.is_none() {
            return Err(UserServiceError::UserNotFound("User not found".to_string()));
        }

        if user.as_ref().unwrap().is_deleted {
            return Err(UserServiceError::UserNotActive("User is not active or deleted".to_string()));
        }

        if user.as_ref().unwrap().password != dto.password {
            return Err(UserServiceError::InvalidCredentials("Invalid credentials".to_string()));
        }

        Ok(user.is_some())
    }
    
    fn user_input_validation(&self, dto: &CheckUserActiveDTO) -> Result<bool, UserServiceError> {
        let mut errors = Vec::new();
        
        if dto.username.trim().is_empty() {
            errors.push(ValidationError {
                field: "username".to_string(),
                message: "Username cannot be empty".to_string(),
            });
        }

        if dto.password.trim().is_empty() {
            errors.push(ValidationError {
                field: "password".to_string(),
                message: "Password cannot be empty".to_string(),
            });
        }

        if !errors.is_empty() {
            return Err(UserServiceError::BadRequest(errors));
        }

        Ok(true)
    }
}