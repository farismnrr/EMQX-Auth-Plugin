use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use argon2::password_hash::rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, String> {
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("Hash error: {}", e))
}

// pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
//     let parsed_hash = PasswordHash::new(hash).map_err(|e| format!("Invalid hash: {}", e))?;
//     Ok(Argon2::default()
//         .verify_password(password.as_bytes(), &parsed_hash)
//         .is_ok())
// }
