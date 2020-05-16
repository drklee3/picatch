use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use dotenv;
use std::env;

use picatch_lib::{constants::PHOTOS_DIR, error::Result, routes, utils};

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::logging::setup_logger()?;

    let interface = env::var("PICATCH_INTERFACE").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PICATCH_PORT").unwrap_or("8080".to_string());

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
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
            .service(actix_files::Files::new("/photos", PHOTOS_DIR.clone()))
            .service(web::resource("/{path:.*}").route(web::get().to(routes::static_files::path)))
    })
    .bind(format!("{}:{}", interface, port))?
    .run()
    .await
    .map_err(Into::into)
}
