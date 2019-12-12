use crate::resizer::{resize, ResizeOptions};
use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;

#[get("/album/{album}/{photo}")]
pub async fn album(info: web::Path<(String, String)>) -> impl Responder {
    format!("Hello! album: {}, photo: {}", info.0, info.1)
}

#[get("/image/{photo}")]
pub async fn image(
    query: web::Query<HashMap<String, u32>>,
    file: web::Path<(String)>,
) -> HttpResponse {
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
