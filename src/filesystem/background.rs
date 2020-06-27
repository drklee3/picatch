use crate::{
    error::Result,
    filesystem::{
        resizer::resize_images,
        utils,
        watcher::{get_watcher_event_action, FileAction},
    },
    model::{config::AppConfig, ImageSize, ResizeOptions},
};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use num_cpus;
use same_file::is_same_file;
use std::fs::{remove_file, rename};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

pub fn startup_resize(
    pool: &ThreadPool,
    config: &AppConfig,
    opts_list: &Vec<ResizeOptions>,
) -> Result<()> {
    // These two do NOT include "./" prefix if original/resized_photos_dir doesn't start with "./"
    let source_files = utils::get_all_files(Path::new(&config.original_photos_dir))?;
    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir))?;

    let jobs = utils::get_files_not_resized(&config, &source_files, &resized_files, opts_list)?;

    if jobs.resize_jobs.len() > 0 {
        info!(
            "{} source images to resize (total {} resized files)",
            jobs.resize_jobs.len(),
            jobs.total_resized_files,
        );
    } else {
        info!("Resized images up to date");
    }

    resize_images(pool, jobs.resize_jobs)?;

    if !jobs.stale_resized_files.is_empty() {
        info!(
            "Found {} resized files without original file, deleting",
            jobs.stale_resized_files.len()
        );
    }

    remove_files(jobs.stale_resized_files.into_iter());

    Ok(())
}

/// Delete all files from an iterator
fn remove_files<I>(paths: I)
where
    I: Iterator<Item = PathBuf>,
{
    for path in paths {
        if let Err(e) = remove_file(&path) {
            warn!("Failed to remove file {}: {}", path.to_string_lossy(), e);
        }
    }
}

/// Rename list of files to list of destinations
/// assumes the two vecs are 1 to 1 sorted
fn rename_files(source_paths: Vec<PathBuf>, dest_paths: Vec<PathBuf>) {
    source_paths
        .iter()
        .zip(dest_paths.iter())
        .for_each(|(source, dest)| {
            if let Err(e) = rename(source, dest) {
                warn!(
                    "Failed to rename file {} -> {}: {}",
                    source.to_string_lossy(),
                    dest.to_string_lossy(),
                    e
                );
            }
        });
}

fn resizer_thread(config: AppConfig) {
    match run_resizer_thread(config) {
        Ok(_) => warn!("Resizer thread stopped"),
        Err(e) => error!("{}", e),
    };
}

fn run_resizer_thread(config: AppConfig) -> Result<()> {
    let workers = num_cpus::get();
    let pool = ThreadPool::new(workers);

    let opts_list: Vec<ResizeOptions> = vec![
        ImageSize::Pixel.into(),
        ImageSize::Small.into(),
        ImageSize::Medium.into(),
        ImageSize::Large.into(),
    ];
    // Scan on startup
    info!("Scanning for pending resizes");
    startup_resize(&pool, &config, &opts_list)?;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("./photos", RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                debug!("New fs event: {:?}", &event);

                // Don't return if error, just keep watching future events
                if let Err(e) = handle_fs_event(&config, &opts_list, &pool, &event) {
                    error!("Failed to handle file event {:?}: {}", event, e);
                }
            }
            Err(e) => warn!("Watch error: {:?}", e),
        }
    }
}

fn handle_fs_event(
    config: &AppConfig,
    opts_list: &Vec<ResizeOptions>,
    pool: &ThreadPool,
    event: &DebouncedEvent,
) -> Result<()> {
    let action = match get_watcher_event_action(event) {
        Some(a) => a,
        None => return Ok(()), // Some NoticeWrite/Remove etc event we don't care about
    };

    // Ignore Write event for directory
    // Modifying "/path/to/photos/img.jpg" will also send a Write to "/path/to/photos"

    match action {
        // New file, resize all files
        FileAction::Resize(path) => {
            // Skip Write events on base original_photos_dir (file contained modified)
            if path.is_dir() && is_same_file(&config.original_photos_dir, &path)? {
                debug!("Write event on original_photos_dir, skipping");
                return Ok(());
            }

            // If it's a new dir, scan entire folder
            // TODO: Only scan modified dir, however inconsistent relative paths
            let jobs = if path.is_dir() {
                let source_files = utils::get_all_files(Path::new(&config.original_photos_dir))?;
                let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir))?;

                utils::get_files_not_resized(config, &source_files, &resized_files, opts_list)?
            } else {
                utils::get_files_not_resized(config, &vec![path], &vec![], opts_list)?
            };

            info!(
                "{} new source image(s) detected, resizing (total {} resized images)",
                jobs.resize_jobs.len(),
                jobs.total_resized_files,
            );
            resize_images(pool, jobs.resize_jobs)?;
        }
        FileAction::Rename(path, dest) => {
            // Move directory, only check destination if is dir since source doesn't exist anymore duh
            if dest.is_dir() {
                let resized_dir_path = utils::get_resized_dir_path(config, &path)?;
                let resized_dir_dest = utils::get_resized_dir_path(config, &dest)?;

                if let Err(e) = rename(&resized_dir_path, &resized_dir_dest) {
                    warn!(
                        "Failed to rename directory {} -> {}: {}",
                        path.to_string_lossy(),
                        dest.to_string_lossy(),
                        e
                    );
                }
                return Ok(());
            }

            let source_paths = utils::get_resized_paths(config, &path, opts_list)?;
            let dest_paths = utils::get_resized_paths(config, &dest, opts_list)?;

            info!(
                "Rename detected, renaming {} resized images",
                source_paths.len()
            );

            rename_files(source_paths, dest_paths);
        }
        // Delete all resized files
        FileAction::Delete(path) => {
            let paths = utils::get_resized_paths(config, &path, opts_list)?;
            info!("Delete detected, deleting {} resized images", paths.len());

            remove_files(paths.into_iter());
        }
    }

    Ok(())
}

pub fn start_resizer_thread(config: &AppConfig) {
    let config = config.clone();
    thread::spawn(move || resizer_thread(config));
}
