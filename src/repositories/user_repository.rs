use rocksdb::Error;
use rocksdb::DB;
use std::sync::Arc;

/// Simple user model used by the repository create function.
/// We only store `username`, `password` and `is_deleted` as requested.
pub struct User {
    pub username: String,
    pub password: String,
    pub is_deleted: bool,
}

impl User {
    pub fn new(username: impl Into<String>, password: impl Into<String>, is_deleted: bool) -> Self {
        User {
            username: username.into(),
            password: password.into(),
            is_deleted,
        }
    }
}

/// Repository wrapper that owns a DB handle and exposes repository methods.
///
/// The service layer should use this repository instead of interacting with
/// `rocksdb::DB` directly.
pub struct UserRepository {
    db: Arc<DB>,
}

impl UserRepository {
    /// Create a new repository that uses the provided `Arc<rocksdb::DB>`.
    pub fn new(db: Arc<DB>) -> Self {
        UserRepository { db }
    }

    /// Create a new user record in RocksDB.
    pub fn create_user(&self, user: &User) -> Result<(), Error> {
        let key = format!("user:{}", user.username);

        let value = format!(
            r#"{{"username":"{}","password":"{}","is_deleted":{}}}"#,
            user.username,
            user.password,
            if user.is_deleted { "true" } else { "false" }
        );

        self.db.put(key.as_bytes(), value.as_bytes())
    }
}