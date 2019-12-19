use actix_web::{get, web, Responder};

#[get("/album/{album}")]
pub async fn get_album(info: web::Path<String>) -> impl Responder {
    format!("Hello! album: {}", info)
}
