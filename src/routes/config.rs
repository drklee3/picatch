use actix_web::{web, HttpRequest, HttpResponse};

use crate::{error::Result, model::config::AppConfig};

pub async fn get_config(_req: HttpRequest, config: web::Data<AppConfig>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(config.get_ref()))
}
