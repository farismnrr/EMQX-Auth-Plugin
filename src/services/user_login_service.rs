use std::sync::Arc;
use log::debug;
use crate::repositories::user_login_repository::UserLoginRepository;
use crate::services::service_error::{UserServiceError, ValidationError};
use crate::dtos::user_dto::{AuthType, CheckUserActiveDTO};
use crate::utils::hash_password::verify_password;
use crate::utils::jwt_sign::create_jwt;

pub struct UserLoginService {
    repo: Arc<UserLoginRepository>,
    secret_key: String,
}

impl UserLoginService {
    pub fn new(repo: Arc<UserLoginRepository>, secret_key: String) -> Self {
        Self { repo, secret_key }
    }

    pub fn login_with_credentials(&self, dto: CheckUserActiveDTO) -> Result<(bool, String), UserServiceError> {
        self.user_input_credentials_validation(&dto)?;

        let user = match self.repo.login_with_credentials(&dto.username)? {
            Some(u) => u,
            None => {
                debug!("[Service | CheckUserActive] User not found: {}", dto.username);
                return Err(UserServiceError::UserNotFound("User not found".into()));
            }
        };

        if user.is_deleted {
            debug!("[Service | CheckUserActive] User is deleted or inactive: {}", dto.username);
            return Err(UserServiceError::UserNotActive("User is not active or deleted".into()));
        }

        match dto.method.unwrap() {
            AuthType::Credentials => {
                let is_valid = verify_password(&dto.password, &user.password);
                if !is_valid {
                    debug!("[Service | CheckUserActive] Invalid credentials for user: {}", dto.username);
                    return Err(UserServiceError::InvalidCredentials("Invalid credentials".into()));
                }

                Ok((true, String::new()))
            }
            AuthType::Jwt => {
                let token = create_jwt(&dto.username, &self.secret_key)
                    .map_err(|e| UserServiceError::JwtError(e.to_string()))?;
                debug!("[Service | CheckUserActive] JWT token created for user: {}", dto.username);
                Ok((true, token))
            }
        }
    }
    
    fn user_input_credentials_validation(&self, dto: &CheckUserActiveDTO) -> Result<bool, UserServiceError> {
        let mut errors = Vec::new();
        if dto.username.trim().is_empty() {
            errors.push(ValidationError {
                field: "username".to_string(),
                message: "Username cannot be empty".to_string(),
            });
        }

        let method = match dto.method {
            Some(ref m) => m,
            None => {
                errors.push(ValidationError {
                    field: "method".into(),
                    message: "method cannot be empty".into(),
                });
                return Err(UserServiceError::BadRequest(errors));
            }
        };

        if matches!(method, AuthType::Credentials) && dto.password.trim().is_empty() {
            errors.push(ValidationError {
                field: "password".into(),
                message: "Password is required for credentials login".into(),
            });
        }

        if !errors.is_empty() {
            return Err(UserServiceError::BadRequest(errors));
        }

        debug!("[Service | CheckUserActive] User input validation passed.");
        Ok(true)
    }
}