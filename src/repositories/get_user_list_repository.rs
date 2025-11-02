use rocksdb::{DB, ReadOptions};
use std::sync::Arc;
use bincode::{decode_from_slice, config::standard};
use log::{debug, error};
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

        // Configure read options to possibly improve iteration performance
        let mut read_opts = ReadOptions::default();
        read_opts.set_verify_checksums(false);
        read_opts.fill_cache(true);

        debug!("[Repository | GetUserList] Starting iteration to collect users from DB.");

    for (key, value) in self.db.iterator_opt(rocksdb::IteratorMode::Start, read_opts).flatten() {
            // Try to convert key to UTF-8 string
            let key_str = match String::from_utf8(key.to_vec()) {
                Ok(s) => {
                    debug!("[Repository | GetUserList] Found key: {}", s);
                    s
                }
                Err(e) => {
                    error!("[Repository | GetUserList] Failed to convert key to string: {e}");
                    debug!("[Repository | GetUserList] Key bytes: {:#?}", key);
                    return Err(UserRepositoryError::Utf8(e))
                }
            };

            // Skip non-user keys
            if !key_str.starts_with("users:") {
                debug!("[Repository | GetUserList] Non-user key skipped: {}", key_str);
                continue;
            }

            // Decode user data from bincode
            debug!("[Repository | GetUserList] Decoding user data for key: {}", key_str);
            let user = match decode_from_slice::<UserEntity, _>(&value, standard()) {
                Ok((user, _)) => {
                    debug!("[Repository | GetUserList] Successfully decoded user: {}", user.username);
                    user
                }
                Err(e) => {
                    error!("[Repository | GetUserList] Failed to decode user for key {}: {}", key_str, e);
                    debug!("[Repository | GetUserList] Value bytes for key {}: {:#?}", key_str, value);
                    return Err(UserRepositoryError::Decode(e))
                }
            };

            // Skip deleted users
            if user.is_deleted {
                debug!("[Repository | GetUserList] Deleted flag set for user: {}", user.username);
                continue;
            }

            debug!("[Repository | GetUserList] Adding user to results: {}", user.username);
            users.push(user);
        }

        Ok(users)
    }
}