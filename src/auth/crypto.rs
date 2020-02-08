/// These are blocking functions, be sure to use web::block when using them
use crate::error::Result;
use crate::model::config::Config;
use argonautica::{Hasher, Verifier};
use rand::distributions::Alphanumeric;
use rand::Rng;

lazy_static::lazy_static! {
    static ref SECRET_KEY: String = get_secret_key().expect("Failed to get secret key");
}

fn get_secret_key() -> Result<String> {
    // Abort on read config errors -- if we can't read config for db info then
    // we don't exactly need this either I guess
    // This means we don't have to handle file not found errors separately
    let mut config = Config::get_from_file()?;

    // If it exists, we don't need to do anything further, just return it
    if let Some(secret_key) = config.secret_key {
        return Ok(secret_key);
    }

    // No secret key so create a random one and save it to config
    let secret_key = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect::<String>();

    config.secret_key = Some(secret_key.clone());

    // If this fails, we better abort too, if we fail to save secret key then
    // encrypted content would be lost
    config.save_to_file()?;

    Ok(secret_key)
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
        // use verify_non_blocking?
        .verify()
        .map_err(Into::into)
}
