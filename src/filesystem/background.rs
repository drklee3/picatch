use crate::{
    error::Result,
    filesystem::{resizer::resize_images, utils},
    model::{config::AppConfig, ImageSize, ResizeOptions},
};
use notify::{watcher, RecursiveMode, Watcher};
use num_cpus;
use std::collections::HashSet;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

pub fn startup_resize(
    pool: &ThreadPool,
    config: &AppConfig,
    opts_list: Vec<ResizeOptions>,
) -> Result<()> {
    let source_files = utils::get_all_files(Path::new(&config.original_photos_dir))?;
    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir))?;
    let jobs = utils::get_files_not_resized(&config, &source_files, &resized_files, opts_list)?;

    let total_jobs = jobs.resize_jobs.iter().fold(0, |acc, (_, v)| acc + v.len());

    if total_jobs > 0 {
        info!(
            "{} source images to resize (total {} resized files)",
            jobs.resize_jobs.len(),
            total_jobs
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

    remove_stale_files(jobs.stale_resized_files)?;

    Ok(())
}

fn remove_stale_files(paths: HashSet<PathBuf>) -> Result<()> {
    for path in paths {
        remove_file(&path)?;
    }

    Ok(())
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
    startup_resize(&pool, &config, opts_list)?;

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
            Ok(event) => println!("{:?}", event),
            Err(e) => warn!("watch error: {:?}", e),
        }
    }
}

pub fn start_resizer_thread(config: &AppConfig) {
    let config = config.clone();
    thread::spawn(move || resizer_thread(config));
}
