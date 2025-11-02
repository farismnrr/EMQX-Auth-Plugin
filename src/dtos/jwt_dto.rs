use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}
