use sha2::{Sha256, Digest};
use subtle::ConstantTimeEq;
use hex;

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let hashed_input = hash_password(password);
    hashed_input.as_bytes().ct_eq(stored_hash.as_bytes()).into()
}
