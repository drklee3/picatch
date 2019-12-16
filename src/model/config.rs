use crate::error::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use toml;

#[derive(Deserialize, Serialize)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub name: String,
    pub table_prefix: String,
}

impl FromStr for DbConfig {
    type Err = Error;

    fn from_str(str_cfg: &str) -> Result<DbConfig> {
        toml::from_str(str_cfg).map_err(Into::into)
    }
}
