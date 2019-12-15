use crate::error::Result;
use argonautica::{Hasher, Verifier};
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::fs;
use std::io::Write;

lazy_static::lazy_static! {
    static ref SECRET_KEY: String = get_secret_key().expect("Failed to generate secret key");
}

fn get_secret_key() -> Result<String> {
    if let Ok(key) = std::env::var("SECRET_KEY") {
        return Ok(key);
    }

    // create a random one and save it to .env
    let rand_key = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect::<String>();

    // should make sure there isn't already a SECRET_KEY here though, maybe
    // dotenv fails to load it somehow (eg in tests)
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "SECRET_KEY={}", rand_key)?;

    Ok(rand_key)
}

pub fn hash_password(password: &str) -> Result<String> {
    Hasher::default()
        .with_password(password)
        // convert &[u8; N] to &[u8]
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(Into::into)
}

pub fn verify(hash: &str, password: &str) -> Result<bool> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(Into::into)
}
