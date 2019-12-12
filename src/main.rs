use actix_files as fs;
use actix_web::{middleware, App, HttpServer};

mod api;
mod error;
mod resizer;
use api::{album, image};
use error::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                // static files
                fs::Files::new("/static", "./static/").show_files_listing(),
            )
            .service(album)
            .service(image)
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
    .map_err(Into::into)
}
