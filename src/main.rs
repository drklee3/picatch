use actix_files as fs;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use setup::get_config;

use dphoto_lib::*;

use error::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Exit if no config found
    let config = get_config()?;
    let db_url = config.to_url();

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool :'(");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                // TODO: Update secret key with an actual secret key
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
            .service(
                web::scope("/api")
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
                    .service(api::get_image),
            )
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
    .map_err(Into::into)
}
