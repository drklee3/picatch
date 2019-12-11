use actix_files as fs;
use actix_web::{get, middleware, App, web, HttpServer, Responder};

#[get("/album/{album}/{photo}")]
async fn api(info: web::Path<(String, String)>) -> impl Responder {
    format!("Hello! album: {}, photo: {}", info.0, info.1)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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
            .service(api)
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
}

