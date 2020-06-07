use std::path::Path;
use std::fs::{self, DirEntry};
use crate::error::Result;

pub fn get_all_files(path: &Path) -> Result<Vec<DirEntry>> {
    let mut files = list_files_recursive(path)?;
    files.sort_by(|a, b| a.path().cmp(&b.path()));

    Ok(files)
}

fn list_files_recursive(dir: &Path) -> Result<Vec<DirEntry>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;

            let path = entry.path();
            if path.is_dir() {
                files.append(&mut list_files_recursive(&path)?);
            } else {
                files.push(entry);
            }
        }
    }

    Ok(files)
}
