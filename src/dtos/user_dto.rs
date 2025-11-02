use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub is_deleted: bool,
}

#[derive(Serialize)]
pub struct GetUserListDTO {
    pub users: Vec<UserDTO>,
}

#[derive(Serialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CheckUserActiveDTO {
    pub username: String,
    pub password: String,
    pub method: Option<AuthType>,
}

#[derive(Serialize)]
pub struct UserJwtDTO {
    pub token: String,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Credentials,
    Jwt,
}
