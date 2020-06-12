use picatch_lib::{
    filesystem::{resizer::resize_images, utils},
    model::{
        config::{AppConfig, PubConfig},
        ResizeOptions,
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

    let opts_list = vec![
        ResizeOptions::new("pixel")
            .set_width(1)
            .set_height(1)
            .set_mode(2),
        ResizeOptions::new("large").set_height(1080),
        ResizeOptions::new("thumb").set_height(128),
    ];

    // Clear resized dir before resizing
    remove_dir_all(&config.resized_photos_dir).unwrap();
    utils::verify_directories_exist(vec![&config.original_photos_dir, &config.resized_photos_dir])
        .unwrap();

    let files = utils::get_all_files(Path::new(&config.original_photos_dir)).unwrap();
    println!("Files to resize: {:#?}", &files);
    assert_eq!(3, files.len());

    resize_images(&config, files, opts_list).unwrap();

    let resized_files = utils::get_all_files(Path::new(&config.resized_photos_dir)).unwrap();

    assert_eq!(9, resized_files.len());
}
