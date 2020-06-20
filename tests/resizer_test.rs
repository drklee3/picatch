use picatch_lib::{
    filesystem::{utils, background},
    model::{
        config::{AppConfig, PubConfig},
        ImageSize, ResizeOptions,
    },
    utils::logging
};
use std::{fs::remove_dir_all, path::Path};
use num_cpus;
use threadpool::ThreadPool;

#[test]
fn builds_resized_file_path() {
    logging::setup_logger().unwrap();

    let config = AppConfig {
        public: PubConfig::default(),
        original_photos_dir: "./tests/test_photos".into(),
        resized_photos_dir: "./tests/test_photos_resized".into(),
        interface: "0.0.0.0".into(),
        port: 8080,
    };

    let opts_list: Vec<ResizeOptions> = vec![
        ImageSize::Pixel.into(),
        ImageSize::Small.into(),
        ImageSize::Large.into(),
    ];

    // Clear resized dir before resizing
    remove_dir_all(&config.resized_photos_dir).unwrap();
    utils::verify_directories_exist(vec![
        &config.original_photos_dir,
        &config.resized_photos_dir,
    ])
    .unwrap();

    let workers = num_cpus::get();
    let pool = ThreadPool::new(workers);

    background::startup_resize(&pool, &config, opts_list).unwrap();

    pool.join();

    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir)).unwrap();

    assert_eq!(9, resized_files.len());
}
