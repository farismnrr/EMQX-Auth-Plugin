use rocksdb::{DB, ReadOptions};
use std::sync::Arc;
use bincode::config::standard;
use bincode::decode_from_slice;
use log::{debug, error};
use crate::entities::user_entity::UserEntity;
use crate::repositories::repository_error::UserRepositoryError;

pub struct CheckUserActiveRepository {
    db: Arc<DB>,
}

impl CheckUserActiveRepository {
    pub fn new(db: Arc<DB>) -> Self {
        CheckUserActiveRepository { db }
    }

    pub fn check_user_active(&self, username: &str) -> Result<Option<UserEntity>, UserRepositoryError> {
        // Build RocksDB key
        let key: String = format!("users:{}", username);

        // Configure read options for optimization
        let mut read_opts = ReadOptions::default();
        read_opts.set_verify_checksums(false);
        read_opts.fill_cache(true);

        // Try to fetch record from DB
        debug!("[Repository | CheckUserActive] Attempting to fetch user '{}' from database.", username);
        let value = match self.db.get_opt(key.as_bytes(), &read_opts) {
            Ok(v) => {
                if v.is_some() {
                    debug!("[Repository | CheckUserActive] Database read returned a value for user '{}'.", username);
                } else {
                    debug!("[Repository | CheckUserActive] Database read returned no value for user '{}'.", username);
                }
                v
            }
            Err(e) => {
                error!("[Repository | CheckUserActive] Database read error for user {username}: {e}");
                debug!("[Repository | CheckUserActive] Database read error for user '{}': {:#?}", username, e);
                return Err(UserRepositoryError::Database(e));
            }
        };

        let Some(value) = value else {
            debug!("[Repository | CheckUserActive] User '{}' not found in database.", username);
            return Ok(None);
        };

        debug!("[Repository | CheckUserActive] Decoding user data for '{}'.", username);
        let (user, _) = match decode_from_slice::<UserEntity, _>(&value, standard()) {
            Ok(decoded) => decoded,
            Err(e) => {
                error!("[Repository | CheckUserActive] Failed to decode user data for {username}: {e}");
                debug!("[Repository | CheckUserActive] Decode error for user '{}': {:#?}", username, e);
                return Err(UserRepositoryError::Decode(e));
            }
        };

        Ok(Some(user))
    }
}
