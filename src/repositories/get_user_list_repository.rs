use rocksdb::DB;
use std::sync::Arc;
use bincode::{decode_from_slice, config::standard};

use crate::entities::user_entity::UserEntity;
use crate::repositories::repository_error::UserRepositoryError;

pub struct GetUserListRepository {
    db: Arc<DB>,
}

impl GetUserListRepository {
    pub fn new(db: Arc<DB>) -> Self {
        GetUserListRepository { db }
    }

    pub fn get_user_list(&self) -> Result<Vec<UserEntity>, UserRepositoryError> {
        let mut users = Vec::new();
        let iterator = self.db.iterator(rocksdb::IteratorMode::Start);

        for item in iterator {
            let (key, value) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.starts_with("users:") {
                let user: UserEntity = decode_from_slice(value.as_ref(), standard())?.0;
                users.push(user);
            }
        }

        Ok(users)
    }
}