use crate::{
    error::{Error, Result},
    model::config::AppConfig,
};
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

pub fn verify_directories_exist(config: &AppConfig) -> Result<()> {
    dir_exists_or_create(Path::new(&config.original_photos_dir))?;
    dir_exists_or_create(Path::new(&config.resized_photos_dir))?;

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

pub fn get_all_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = list_files_recursive(path)?;
    files.sort();

    Ok(files)
}

fn list_files_recursive(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;

            let path = entry.path();
            if path.is_dir() {
                files.append(&mut list_files_recursive(&path)?);
            } else {
                files.push(entry.path());
            }
        }
    }

    Ok(files)
}
