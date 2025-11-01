use bincode::{Encode, Decode};

#[derive(Encode, Decode)]
pub struct UserEntity {
    pub username: String,
    pub password: String,
    pub is_deleted: bool,
}

impl UserEntity {
    pub fn create(username: impl Into<String>, password: impl Into<String>) -> Self {
        UserEntity {
            username: username.into(),
            password: password.into(),
            is_deleted: false,
        }
    }
}
