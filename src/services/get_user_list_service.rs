use std::sync::Arc;

use crate::repositories::get_user_list_repository::GetUserListRepository;
use crate::services::service_error::UserServiceError;
use crate::dtos::user_dto::UserDTO;

pub struct GetUserListService {
    repo: Arc<GetUserListRepository>,
}

impl GetUserListService {
    pub fn new(repo: Arc<GetUserListRepository>) -> Self {
        Self { repo }
    }

    pub fn get_user_list(&self) -> Result<Vec<UserDTO>, UserServiceError> {
        let users = self.repo.get_user_list()?;
        let dto_users: Vec<UserDTO> = users.into_iter().map(|user| UserDTO {
            username: user.username,
            password: user.password,
            is_deleted: user.is_deleted,
        }).collect();
        Ok(dto_users)
    }
}