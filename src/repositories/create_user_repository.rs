use rocksdb::DB;
use std::sync::Arc;
use bincode::{encode_to_vec, config::standard};

use crate::entities::user_entity::UserEntity;
use crate::repositories::repository_error::UserRepositoryError;

pub struct CreateUserRepository {
    db: Arc<DB>,
}

impl CreateUserRepository {
    pub fn new(db: Arc<DB>) -> Self {
        CreateUserRepository { db }
    }

    pub fn create_user(&self, username: &str, password_hash: &str) -> Result<(), UserRepositoryError> {
        let user = UserEntity::create(username, password_hash);
        let key = format!("users:{}", user.username);
        let value = encode_to_vec(&user, standard())?;

        self.db.put(key.as_bytes(), value)?;
        Ok(())
    }
}