use crate::{
    error::{Error, Result},
    model::{ImageSize, ResizeOptions},
};
use std::fs::create_dir_all;
use std::path::Path;

pub fn verify_directories_exist(dirs: Vec<&str>) -> Result<()> {
    for dir in dirs {
        dir_exists_or_create(Path::new(dir))?;
    }

    Ok(())
}

pub fn dir_exists_or_create(path: &Path) -> Result<()> {
    if path.exists() && path.is_dir() {
        return Ok(());
    }

    // Exit if file exists with dir name since we don't want to modify existing files
    // This doesn't work if there is a trailing slash though. Will error later on `create_dir_all`
    if path.is_file() {
        return Err(Error::Picatch(format!(
            "File exists, but isn't a directory: {}",
            path.to_string_lossy()
        )));
    }

    if !path.exists() {
        info!(
            "Directory {} doesn't exist, creating...",
            path.to_string_lossy()
        );
        create_dir_all(path)?;
    }

    Ok(())
}

pub fn get_resize_options(sizes: Vec<ImageSize>) -> Vec<ResizeOptions> {
    sizes.into_iter().map(|size| size.into()).collect()
}
