use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}
