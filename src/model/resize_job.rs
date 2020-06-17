use crate::model::ResizeOptions;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ResizeJob<'a> {
    pub source: &'a Path,
    pub destination: PathBuf,
    pub options: &'a ResizeOptions,
}
