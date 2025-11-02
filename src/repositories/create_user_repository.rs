use rocksdb::{DB, WriteOptions};
use std::sync::Arc;
use bincode::{encode_to_vec, config::standard};
use log::{debug, error};
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
    debug!("[Repository | CreateUser] Starting user creation for username: {}", username);
        
        // Build user entity
        let user = UserEntity::create(username, password_hash);
        let key = format!("users:{}", user.username);
    debug!("[Repository | CreateUser] Created user entity with key: {}", key);

        // Encode user to binary
        let value = match encode_to_vec(&user, standard()) {
            Ok(v) => {
                debug!("[Repository | CreateUser] Successfully encoded user to binary, size: {} bytes", v.len());
                v
            }
            Err(e) => {
                error!("[Repository | CreateUser] Failed to serialize user {}: {e}", username);
                return Err(UserRepositoryError::Encode(e));
            }
        };

        // Configure write options for performance tuning
        let mut opts: WriteOptions = WriteOptions::default();
        opts.set_sync(false);
        opts.disable_wal(true);
    debug!("[Repository | CreateUser] Write options configured: sync=false, wal=disabled");

        // Write to RocksDB
        match self.db.put_opt(key.as_bytes(), value, &opts) {
            Ok(_) => {
                debug!("[Repository | CreateUser] User {} successfully written to database", username);
                Ok(())
            }
            Err(e) => {
                error!("[Repository | CreateUser] Failed to write user {} to database: {e}", username);
                Err(UserRepositoryError::Database(e))
            }
        }
    }
}