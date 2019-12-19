use actix_files as fs;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, App, HttpServer};
use dotenv;

use dphoto_lib::*;

use error::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("dphoto-auth")
                    .secure(false)
                    .same_site(SameSite::Strict),
            ))
            // enable logger - register logger last!
            .wrap(middleware::Logger::default())
            .service(
                // static files
                fs::Files::new("/static", "./static/").show_files_listing(),
            )
            // AUTH routes
            // POST /login
            .service(api::post_login)
            // POST /logout
            .service(api::post_logout)
            .service(api::get_index)
            // API endpoints
            // GET /album/{album}
            .service(api::get_album)
            // GET /image/{image}
            .service(api::get_image)
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
    .map_err(Into::into)
}
