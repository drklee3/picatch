use crate::error::Result;
use config;
use serde::{Deserialize, Serialize};
use std::cmp::Eq;

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

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
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
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let mut conf = config::Config::default();

        conf
            // Add in `./picatch.toml`
            .merge(config::File::with_name("picatch"))
            .unwrap()
            // Add in settings from the environment (with a prefix of PICATCH)
            // Eg.. `PICATCH_DEBUG=1 ./target/app` would set the `debug` key
            .merge(config::Environment::with_prefix("PICATCH"))
            .unwrap();

        conf.try_into().map_err(From::from)
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
