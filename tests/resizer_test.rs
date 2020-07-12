use picatch_lib::{
    filesystem::{background, files, utils},
    model::{
        config::{AppConfig, PubConfig},
        ImageSize, ResizeOptions,
    },
    utils::logging,
};
use std::{
    fs::{remove_dir_all, File},
    path::Path,
};
use threadpool::ThreadPool;

#[test]
fn builds_resized_file_path() {
    logging::setup_logger().unwrap();

    let config = AppConfig {
        public: PubConfig::default(),
        original_photos_dir: "tests/test_photos".into(),
        resized_photos_dir: "tests/test_photos_resized".into(),
        interface: "0.0.0.0".into(),
        port: 8080,
    };

    let opts_list: Vec<ResizeOptions> = vec![
        ImageSize::Pixel.into(),
        ImageSize::Small.into(),
        ImageSize::Large.into(),
    ];

    // Clear resized dir before resizing, ignore error since the dir doesn't exist on git repo
    let _ = remove_dir_all(&config.resized_photos_dir);

    utils::verify_directories_exist(vec![
        &config.original_photos_dir,
        &config.resized_photos_dir,
    ])
    .unwrap();

    // Create some stale files, ensure they don't have same file name as any original files
    File::create("./tests/test_photos_resized/TEST_0.jpg").unwrap();
    File::create("./tests/test_photos_resized/TEST_1.jpg").unwrap();

    let workers = num_cpus::get();
    let pool = ThreadPool::new(workers);

    background::startup_resize(&pool, &config, &opts_list).unwrap();

    pool.join();

    let resized_files = files::get_all_files(Path::new(&config.resized_photos_dir)).unwrap();

    assert_eq!(9, resized_files.len());
}
