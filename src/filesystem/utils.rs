use crate::{
    error::{Error, Result},
    model::{config::AppConfig, ImageSize, ResizeJob, ResizeOptions},
};
use std::collections::HashSet;
use std::fs::{self, create_dir_all};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

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

pub fn get_destination_path(
    config: &AppConfig,
    path: &Path,
    opts: &ResizeOptions,
) -> Result<PathBuf> {
    let img_path_str = path.to_string_lossy();
    // resized dir + relative file path + size
    let mut dest_path = PathBuf::from(&config.resized_photos_dir);

    // Just in case, check if path includes original_photos_dir
    if dest_path.starts_with(&config.original_photos_dir) {
        dest_path = dest_path
            .strip_prefix(&config.original_photos_dir)
            .map_err(|_| Error::Picatch(format!("Failed to strip prefix: {}", img_path_str)))?
            .to_path_buf();
    }

    // Get file stem first, in case there isn't a file name provided
    let file_name = path
        .file_stem()
        .ok_or(Error::Picatch(format!(
            "Path missing file name: {}",
            img_path_str
        )))?
        .to_string_lossy();

    let file_dir = path
        .parent()
        .ok_or(Error::Picatch(format!(
            "Path missing parent: {}",
            img_path_str
        )))?
        .strip_prefix(&config.original_photos_dir)
        .map_err(|_| Error::Picatch(format!("Failed to strip prefix: {}", img_path_str)))?;

    println!("File parent {}", file_dir.to_string_lossy());

    dest_path.push(file_dir);

    let file_ext = path
        .extension()
        .ok_or(Error::Picatch(format!(
            "Path missing extension: {}",
            img_path_str
        )))?
        .to_string_lossy();

    // Create new file name with size attached.
    // Not including hash for now, frontend doesn't know about the hash
    let new_file_name = format!("{}-{}.{}", file_name, opts.name, file_ext);
    dest_path.push(&new_file_name);

    Ok(dest_path)
}

pub fn get_files_not_resized<'a>(
    config: &AppConfig,
    source_files: &'a Vec<PathBuf>,
    resized_files: Vec<PathBuf>,
    options_list: &'a Vec<ResizeOptions>,
) -> Result<Vec<ResizeJob<'a>>> {
    // let orig_files = get_all_files(Path::new(&config.original_photos_dir))?;
    let resized_files: HashSet<PathBuf> = HashSet::from_iter(resized_files);

    // Correct number of files, just exit ez
    if source_files.len() * options_list.len() == resized_files.len() {
        return Ok(Vec::new());
    }

    let mut to_resize = Vec::new();

    for file in source_files {
        for options in options_list {
            let dest = get_destination_path(&config, &file, &options)?;

            if resized_files.contains(&dest) {
                continue;
            }

            let new_job = ResizeJob {
                source: &file,
                destination: dest,
                options: &options,
            };

            to_resize.push(new_job);
        }
    }

    Ok(to_resize)
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
