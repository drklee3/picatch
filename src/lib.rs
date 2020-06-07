// So we can use log macros without having to import them every time
#[macro_use]
extern crate log;

pub mod error;
pub mod filesystem;
pub mod model;
pub mod routes;
pub mod utils;
