use picatch_lib::{
    model::config::{AppConfig, PubConfig},
    routes::directory_api::get_dir_listing,
    utils::logging,
};

#[test]
fn directory_api_lists_files() {
    logging::setup_logger().unwrap();

    let config = AppConfig {
        public: PubConfig::default(),
        original_photos_dir: "./tests/test_photos".into(),
        resized_photos_dir: "./tests/test_photos_resized".into(),
        interface: "0.0.0.0".into(),
        port: 8080,
    };

    let listing = get_dir_listing("".into(), &config).unwrap();

    assert_eq!(3, listing.files.len());

    let test_album = listing.files[0].album.clone().unwrap();
    println!("Album: {:?}", &test_album);

    assert_eq!(
        Some("a test album description".to_string()),
        test_album.description
    );
    assert_eq!(Some("DSC_5644.jpg".to_string()), test_album.cover);
}
