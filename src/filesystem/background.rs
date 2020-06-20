use crate::{
    error::Result,
    filesystem::{resizer::resize_images, utils},
    model::{config::AppConfig, ResizeOptions, ImageSize},
};
use notify::{watcher, RecursiveMode, Watcher};
use num_cpus;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

pub fn startup_resize(pool: &ThreadPool, config: &AppConfig, opts_list: Vec<ResizeOptions>) -> Result<()> {
    let source_files = utils::get_all_files(Path::new(&config.original_photos_dir))?;
    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir))?;
    let jobs = utils::get_files_not_resized(&config, source_files, resized_files, opts_list)?;

    let total_jobs = jobs.iter().fold(0, |acc, (_, v)| acc + v.len());
    info!("{} source images to resize (total {} resized files)", jobs.len(), total_jobs);

    resize_images(pool, jobs)?;

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
