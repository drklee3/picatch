use crate::error::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub secret_key: Option<String>,
    pub database: DbConfig,
}

#[derive(Deserialize, Serialize)]
pub struct DbConfig {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub name: Option<String>,
    pub table_prefix: Option<String>,
}

impl Config {
    pub fn get_from_file() -> Result<Self> {
        fs::read_to_string("config.toml").map_err(|e| {
            error!("Failed to read config.toml from file");

            e
        })?.parse()
    }

    pub fn save_to_file(&self) -> Result<()> {
        let str_config = toml::to_string_pretty(&self)?;
        fs::write("config.toml", &str_config).map_err(Into::into)
    }

    /// Check either if a database url or equivalent fields exist
    pub fn is_valid(&self) -> bool {
        let db = &self.database;
        if db.url.is_some() {
            return true;
        }

        [&db.username, &db.password, &db.host, &db.name]
            .iter()
            .all(|x| x.is_some())
    }

    /// Converts config to a database url.  If a database url is provided in the
    /// config, it will have priority.
    pub fn to_url(&self) -> Option<String> {
        let db = &self.database;

        if !self.is_valid() {
            return None;
        }

        if db.url.is_some() {
            return db.url.clone();
        }

        // Okay to unwrap here since is_valid() verifies they are all Some<String>
        Some(format!(
            "postgresql://{}:{}@{}:{}/{}",
            db.username.as_ref().unwrap(),
            db.password.as_ref().unwrap(),
            db.host.as_ref().unwrap(),
            // This isn't checked in is_valid() since we can just use the default port
            db.port.as_ref().unwrap_or(&"5432".to_owned()),
            db.name.as_ref().unwrap()
        ))
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(str_cfg: &str) -> Result<Config> {
        toml::from_str(str_cfg).map_err(Into::into)
    }
}
