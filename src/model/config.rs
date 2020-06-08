use crate::error::Result;
use config::{self, ConfigError};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::default::Default;
use std::env;
use log::LevelFilter;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct NavLink {
    pub text: String,
    pub url: String,
}

/// Config for sending data to frontend
#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct PubConfig {
    #[serde(default = "default_site_name")]
    pub site_name: String,

    #[serde(default = "default_links")]
    pub links: Vec<NavLink>,

    #[serde(default = "default_version")]
    pub version: String,
}

impl Default for PubConfig {
    fn default() -> Self {
        PubConfig {
            site_name: default_site_name(),
            links: default_links(),
            version: default_version(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub public: PubConfig,

    #[serde(default = "default_original_photos_dir")]
    pub original_photos_dir: String,

    #[serde(default = "default_resized_photos_dir")]
    pub resized_photos_dir: String,

    /// actix listening interface
    #[serde(default = "default_interface")]
    pub interface: String,

    /// actix listening port
    #[serde(default = "default_port")]
    pub port: u32,

    /// Log level
    #[serde(default = "default_log")]
    pub log: LevelFilter,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let mut conf = config::Config::default();

        let config_file_path = env::var("PICATCH_CONFIG").unwrap_or("picatch".into());

        // Add in `./picatch.toml`
        if let Err(e) = conf.merge(config::File::with_name(&config_file_path).required(false)) {
            match e {
                ConfigError::Foreign(e) => {
                    warn!("Config error: {}", e);
                },
                _ => {
                    return Err(e.into())
                }
            }
        }

        // Add in settings from the environment (with a prefix of PICATCH)
        // Eg.. `PICATCH_DEBUG=1 ./target/app` would set the `debug` key
        conf.merge(config::Environment::with_prefix("PICATCH"))?;

        conf.try_into().map_err(Into::into)
    }
}

fn default_site_name() -> String {
    "picatch".into()
}

fn default_links() -> Vec<NavLink> {
    Vec::new()
}

fn default_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

fn default_original_photos_dir() -> String {
    "./photos/".into()
}

fn default_resized_photos_dir() -> String {
    "./photos_resized/".into()
}

fn default_interface() -> String {
    "0.0.0.0".into()
}

fn default_port() -> u32 {
    8080
}

fn default_log() -> LevelFilter {
    LevelFilter::Info
}
