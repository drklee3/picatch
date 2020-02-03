use crate::error::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub database: DbConfig,
}

#[derive(Deserialize, Serialize)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub name: String,
    pub table_prefix: Option<String>,
}

impl Config {
    pub fn to_url(&self) -> String {
        let db = &self.database;
        format!(
            "postgresql://{}:{}@{}/{}",
            db.username, db.password, db.host, db.name
        )
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(str_cfg: &str) -> Result<Config> {
        toml::from_str(str_cfg).map_err(Into::into)
    }
}
