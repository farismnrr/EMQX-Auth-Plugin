use serde::{Serialize};

#[derive(Serialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub is_deleted: bool,
}

#[derive(Serialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
}