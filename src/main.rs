use actix_files as fs;
use actix_web::{
    get, middleware, web, web::Bytes, App, HttpResponse, HttpServer, Responder, Result,
};

mod resizer;
use resizer::{resize, FitMode, ResizeOptions};

#[get("/album/{album}/{photo}")]
async fn api(info: web::Path<(String, String)>) -> impl Responder {
    format!("Hello! album: {}, photo: {}", info.0, info.1)
}

#[get("/image/{photo}")]
async fn image(file: web::Path<(String)>) -> HttpResponse {
    let opts = ResizeOptions {
        width: 400,
        height: 400,
        mode: FitMode::Scale,
    };

    let file_path = format!("static/{}", file.into_inner());

    let img_buf = resize(&file_path, opts);

    HttpResponse::Ok().body(img_buf)
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
            .service(image)
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
}
