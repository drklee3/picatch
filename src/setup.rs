use crate::error::Result;
use crate::model::config;
use std::fs;

pub fn get_config() -> Result<config::Config> {
    fs::read_to_string("config.toml")?.parse()
}

pub fn save_config(cfg: config::Config) -> Result<()> {
    let str_config = toml::to_string_pretty(&cfg)?;
    fs::write("config.toml", &str_config).map_err(Into::into)
}
