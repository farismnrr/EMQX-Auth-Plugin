//! Service layer: user creation
//!
//! The service layer must not depend on infrastructure types. It accepts a
//! repository interface (concrete `UserRepository` here) and implements business
//! logic. This keeps the layering: infrastructure => repositories => services => handler.

use std::sync::Arc;

use crate::repositories::user_repository::{UserRepository, User as RepoUser};

/// Service that performs user-related business logic.
///
/// It holds a repository handle (injected from `main`/`server`) and uses it
/// to persist data. No rocksdb types are referenced here.
pub struct UserService {
	repo: Arc<UserRepository>,
}

impl UserService {
	/// Create a new service with a repository.
	pub fn new(repo: Arc<UserRepository>) -> Self {
		UserService { repo }
	}

	/// Create a new user. Build the domain model and delegate persistence to
	/// the repository. Map repository errors to a simple `String`.
	pub fn create_user(&self, username: &str, password: &str) -> Result<(), String> {
		let user = RepoUser::new(username, password, false);
		self.repo
			.create_user(&user)
			.map_err(|e| format!("DB Error: {}", e))
	}
}

