use actix_files as fs;
use std::collections::HashMap;
use actix_web::{
    get, middleware, web, web::Bytes, App, HttpResponse, HttpServer, Responder, Result,
};

mod resizer;
use resizer::{resize, ResizeOptions};

#[get("/album/{album}/{photo}")]
async fn api(info: web::Path<(String, String)>) -> impl Responder {
    format!("Hello! album: {}, photo: {}", info.0, info.1)
}

#[get("/image/{photo}")]
async fn image(query: web::Query<HashMap<String, u32>>, file: web::Path<(String)>) -> HttpResponse {
    println!("query: {:#?}", &query);
    let opts = ResizeOptions {
        width: *query.get("width").unwrap_or(&400),
        height: *query.get("height").unwrap_or(&400),
        mode: *query.get("mode").unwrap_or(&0),
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
