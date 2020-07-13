#[macro_use]
extern crate log;

use actix_cors::Cors;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use std::process;

use picatch_lib::{
    error::Result,
    filesystem::{background, utils::verify_directories_exist},
    model::config::AppConfig,
    routes, utils,
};

#[actix_rt::main]
async fn main() {
    // Wrap run fn so we can catch all errors and print them properly
    match run().await {
        Ok(_) => {
            info!("bye bye");
            process::exit(0)
        }
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    }
}

async fn run() -> Result<()> {
    dotenv::dotenv().ok();
    utils::logging::setup_logger()?;
    let config = AppConfig::new()?;
    debug!("Loaded config: {:#?}", config);

    verify_directories_exist(vec![
        &config.original_photos_dir,
        &config.resized_photos_dir,
    ])?;

    background::start_resizer_thread(&config);

    let config_clone = config.clone();

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .data(config_clone.clone())
            .wrap(Cors::new().send_wildcard().max_age(3600).finish())
            .wrap(middleware::Compress::default())
            // enable logger - register logger last!
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/").service(
                    web::resource("/photos/{path:.*}")
                        .route(web::get().to(routes::directory_api::dir_listing)),
                ),
            )
            .service(actix_files::Files::new(
                "/photo/",
                &config_clone.original_photos_dir,
            ))
            .service(actix_files::Files::new(
                "/photo_sized/",
                &config_clone.resized_photos_dir,
            ))
            .service(
                web::resource("/{path:.*}")
                    .route(web::get().to(routes::static_files::path))
                    .route(web::head().to(HttpResponse::Ok)),
            )
    })
    .bind(format!("{}:{}", &config.interface, &config.port))?
    .run()
    .await
    .map_err(Into::into)
}
