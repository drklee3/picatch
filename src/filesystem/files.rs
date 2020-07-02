use crate::{
    error::Result,
    filesystem::path::get_destination_path,
    model::{config::AppConfig, ResizeJob, ResizeOptions},
};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

pub struct ResizeJobs {
    pub resize_jobs: HashMap<PathBuf, Vec<ResizeJob>>,
    pub total_resized_files: u64,

    // Resized files that don't have a corresponding original image, to be deleted
    pub stale_resized_files: HashSet<PathBuf>,
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
