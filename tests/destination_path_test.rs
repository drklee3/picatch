use picatch_lib::{
    filesystem::utils::get_destination_path,
    model::{config::AppConfig, ImageSize, ResizeOptions},
};

use std::env::current_dir;
use std::path::PathBuf;

#[test]
fn it_builds_destination_path() {
    let configs = vec![
        AppConfig::default(), // Default config with "photos_resized" dir
        // Do not need to test with "./" prefix since they're handled by config initialization
        {
            // conf with trailing slashes
            let mut config = AppConfig::default();
            config.original_photos_dir = "photos/".into();
            config.resized_photos_dir = "photos_resized/".into();
            config
        },
    ];

    let opts_list: Vec<ResizeOptions> = vec![
        ImageSize::Pixel.into(),
        ImageSize::Small.into(),
        ImageSize::Large.into(),
    ];

    let cur_dir = current_dir().unwrap();
    let cur_dir_str = cur_dir.to_str().unwrap(); // Just lazy way to reuse cur_dir

    let paths = vec![
        PathBuf::from(&format!("{}/./photos/DSC_1111.jpg", cur_dir_str)),
        PathBuf::from("./photos/DSC_1111.jpg"),
        PathBuf::from("photos/DSC_1111.jpg"),
    ];

    let dest_paths = vec![
        PathBuf::from("photos_resized/DSC_1111-pixel.jpg"),
        PathBuf::from("photos_resized/DSC_1111-small.jpg"),
        PathBuf::from("photos_resized/DSC_1111-large.jpg"),
    ];

    for config in configs {
        for path in &paths {
            for (opts, expected_dest) in opts_list.iter().zip(dest_paths.iter()) {
                let dest = get_destination_path(&config, path, &opts).unwrap();

                assert_eq!(dest, *expected_dest);
            }
        }
    }
}
