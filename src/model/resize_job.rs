use crate::model::ResizeOptions;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ResizeJob {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub options: ResizeOptions,
}
