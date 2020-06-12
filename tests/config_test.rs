use picatch_lib::model::config::{AppConfig, NavLink, PubConfig};

#[test]
fn it_parses_config_from_file() {
    let config = AppConfig::new().unwrap();

    let config_defaults = AppConfig {
        public: PubConfig {
            site_name: "picatch".into(),
            links: vec![
                NavLink {
                    text: "Picatch".into(),
                    url: "https://github.com/drklee3/picatch".into(),
                },
                NavLink {
                    text: "GitHub".into(),
                    url: "https://github.com/drklee3/".into(),
                },
            ],
            version: env!("CARGO_PKG_VERSION").into(),
        },
        original_photos_dir: "./photos".into(),
        resized_photos_dir: "./photos_resized".into(),
        interface: "0.0.0.0".into(),
        port: 8080,
    };

    assert_eq!(config_defaults, config);
}
