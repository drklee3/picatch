use crate::auth;
use crate::error::Result;
use crate::resizer::{resize, ResizeOptions};
use crate::setup;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::collections::HashMap;

#[get("/")]
async fn index(id: Identity) -> String {
    let config = setup::get_config();

    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

#[get("/login")]
async fn login(id: Identity) -> HttpResponse {
    id.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

#[get("/logout")]
async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

#[get("/album/{album}")]
pub async fn album(info: web::Path<String>) -> impl Responder {
    format!("Hello! album: {}", info)
}

#[get("/image/{image}")]
pub async fn image(
    query: web::Query<HashMap<String, u32>>,
    file: web::Path<(String)>,
) -> Result<HttpResponse> {
    println!("query: {:#?}", &query);
    let opts = ResizeOptions {
        width: query.get("width").cloned(),
        height: query.get("height").cloned(),
        mode: *query.get("mode").unwrap_or(&0),
    };

    let file_path = format!("static/{}", file.into_inner());

    let img_buf = resize(&file_path, opts)?;

    Ok(HttpResponse::Ok().body(img_buf))
}
