use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};

use picatch_lib::{error::Result, model::config::AppConfig, routes, utils};

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = AppConfig::new()?;
    let config_clone = config.clone();

    utils::logging::setup_logger()?;

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .data(config_clone.clone())
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
                    .finish(),
            )
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
                "/photos/orig/",
                &config_clone.original_photos_dir,
            ))
            .service(actix_files::Files::new(
                "/photos/scaled/",
                &config_clone.resized_photos_dir,
            ))
            .service(web::resource("/{path:.*}").route(web::get().to(routes::static_files::path)))
    })
    .bind(format!("{}:{}", &config.interface, &config.port))?
    .run()
    .await
    .map_err(Into::into)
}
