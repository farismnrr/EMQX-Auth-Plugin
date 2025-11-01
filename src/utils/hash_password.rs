use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, Error},
};
use argon2::password_hash::rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

// pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
//     let parsed_hash = PasswordHash::new(hash).map_err(|e| format!("Invalid hash: {}", e))?;
//     Ok(Argon2::default()
//         .verify_password(password.as_bytes(), &parsed_hash)
//         .is_ok())
// }
