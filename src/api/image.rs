use crate::auth;
use crate::error::Result;
use crate::resizer::{resize, ResizeOptions};
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use std::collections::HashMap;

#[get("/image/{image}")]
pub async fn get_image(
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
