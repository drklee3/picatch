use crate::{
    error::{Error, Result},
    model::{config::AppConfig, ImageSize, ResizeJob, ResizeOptions},
};
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
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

// Converts path in original_photos_dir to a path in resized_photos_dir
pub fn get_resized_dir_path(config: &AppConfig, path: &Path) -> Result<PathBuf> {
    let path_str = path.to_string_lossy();

    // Assumes config paths do NOT have the "./" prefix
    // Should be handled when creating the config
    let original_photos_dir = PathBuf::from(&config.original_photos_dir);

    // Resized dir + relative file path + size
    let mut dest_path = PathBuf::from(&config.resized_photos_dir);

    let cur_dir = {
        let dir = current_dir()?;

        // Dumb way to add trailing slash, need trailing slash added so the file
        // path is propperly stripped
        let mut dir_str = dir.into_os_string();
        dir_str.push("/");

        PathBuf::from(dir_str)
    };

    // Watcher paths prepend absolute paths, so remove it here
    // eg: /mnt/c/.../picatch/./photos/DSC_3328.jpg
    // We only want the ./photos/DSC_3328.jpg part. We don't need to
    // canonicalize() path beforehand since doesn't matter if there's an extra
    // /./ in the middle the /mnt/c/.../picatch/ part will be stripped
    let path = match path.strip_prefix(cur_dir) {
        Ok(p) => p,
        // If stripping cur_dir doesn't work, there's still the "./"
        Err(_) => path.strip_prefix(".").unwrap_or(path),
    };

    // If is file, get the parent dir. If dir, don't get parent
    // This contains original_photos_dir/path/to/dir
    let source_file_dir = if let Some(ext) = path.extension() {
        let extension = ext.to_string_lossy().to_lowercase();
        if extension == "jpg" || extension == "jpeg" {
            path.parent()
                .ok_or(Error::Picatch(format!("Path missing parent: {}", path_str)))?
        } else {
            path
        }
    } else {
        path
    };

    // Path is relative, so we need to handle absolute paths for original_photos_dir

    // This is the path to dir *without* the original_photos_dir
    let file_dir = source_file_dir
        .strip_prefix(&original_photos_dir)
        .map_err(|_| {
            Error::Picatch(format!(
                "get_resized_dir_path: Failed to strip original_photos_dir ({}) from source_file_dir ({}), path: {}",
                original_photos_dir.to_string_lossy(),
                source_file_dir.to_string_lossy(),
                path_str
            ))
        })?;

    dest_path.push(file_dir);

    Ok(dest_path)
}

// Paths will *not* have "./" prefix
pub fn get_destination_path(
    config: &AppConfig,
    path: &Path,
    opts: &ResizeOptions,
) -> Result<PathBuf> {
    let mut dest_path = get_resized_dir_path(config, path)?;

    let file_name = path
        .file_stem()
        .ok_or(Error::Picatch(format!(
            "Path missing file name: {}",
            path.to_string_lossy()
        )))?
        .to_string_lossy();

    let file_ext = path
        .extension()
        .ok_or(Error::Picatch(format!(
            "Path missing extension: {}",
            path.to_string_lossy()
        )))?
        .to_string_lossy();

    // Create new file name with size attached.
    // Not including hash for now, frontend doesn't know about the hash
    let new_file_name = format!("{}-{}.{}", file_name, opts.name, file_ext);
    dest_path.push(&new_file_name);

    Ok(dest_path)
}

pub struct ResizeJobs {
    pub resize_jobs: HashMap<PathBuf, Vec<ResizeJob>>,
    pub total_resized_files: u64,

    // Resized files that don't have a corresponding original image, to be deleted
    pub stale_resized_files: HashSet<PathBuf>,
}

/// Get the resized image paths from a single original image path
pub fn get_resized_paths(
    config: &AppConfig,
    source: &Path,
    options_list: &Vec<ResizeOptions>,
) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for options in options_list {
        let dest = get_destination_path(&config, &source, &options)?;

        paths.push(dest);
    }

    Ok(paths)
}

pub fn get_files_not_resized(
    config: &AppConfig,
    source_files: &Vec<PathBuf>,
    resized_files: &Vec<PathBuf>,
    options_list: &Vec<ResizeOptions>,
) -> Result<ResizeJobs> {
    let mut resized_files_set: HashSet<PathBuf> = HashSet::from_iter(resized_files.iter().cloned());

    let mut to_resize = HashMap::new();
    let mut total_resized_files = 0;

    for file in source_files {
        let file_jobs = to_resize.entry(file.clone()).or_insert(Vec::new());

        for options in options_list {
            let dest = get_destination_path(&config, &file, &options)?;

            // Skip already resized files
            if resized_files_set.contains(&dest) {
                // Remove valid resized file
                resized_files_set.remove(&dest);
                continue;
            }

            // Cloning stuff here since threadpool needs ownership
            let new_job = ResizeJob {
                source: file.clone(),
                destination: dest,
                options: options.clone(),
            };

            file_jobs.push(new_job);
            total_resized_files += 1;
        }

        // If none are required, remove empty entry from map
        if file_jobs.is_empty() {
            to_resize.remove(file);
        }
    }

    // resized_files_set should contain resized files that don't have orig file now

    Ok(ResizeJobs {
        resize_jobs: to_resize,
        total_resized_files,
        stale_resized_files: resized_files_set,
    })
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
                // Verify this is a jpg image
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();

                    // Skip if isn't a jpg
                    if ext != "jpg" && ext != "jpeg" {
                        continue;
                    }
                } else {
                    // Skip if no extension
                    continue;
                }

                files.push(entry.path());
            }
        }
    }

    Ok(files)
}
