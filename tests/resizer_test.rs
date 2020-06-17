use picatch_lib::{
    filesystem::{resizer::resize_images, utils},
    model::{
        config::{AppConfig, PubConfig},
        ResizeOptions, ImageSize
    },
};
use std::{fs::remove_dir_all, path::Path};

#[test]
fn builds_resized_file_path() {
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

    let source_files = utils::get_all_files(Path::new(&config.original_photos_dir)).unwrap();
    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir)).unwrap();
    let jobs = utils::get_files_not_resized(&config, &source_files, resized_files, &opts_list).unwrap();

    println!("Resize jobs: {:#?}", &jobs);
    assert_eq!(9, jobs.len());

    resize_images(jobs).unwrap();

    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir)).unwrap();

    assert_eq!(9, resized_files.len());
}
