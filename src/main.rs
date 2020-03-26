#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;

use actix_files as fs;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::run_pending_migrations;
use dotenv;
use dphoto_lib::*;
use error::Result;
use model::config::Config;
use model::pool::Pool;

// Embed SQL migrations into compiled binary
embed_migrations!("migrations");

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::logging::setup_logger()?;

    // Exit if no config found
    let config = Config::get_from_file()?;
    let db_url = config.to_url().expect("Invalid database url");

    // Connect to database and create connection pool with r2d2
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: Pool = r2d2::Pool::builder()
        // TODO: Remove this, just added cause I'm lazy and using a free ElephantSQL
        // database with max 5 concurrent connections
        .max_size(2)
        .build(manager)
        .expect("Failed to create pool :'(");

    debug!("Connected to database");

    {
        // New scope so this connection gets dropped after migrations
        let conn = &pool.get().unwrap();
        run_pending_migrations(conn)?;
        debug!("Pending migrations finished");
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(web::JsonConfig::default().limit(4096))
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
                web::scope("/api")
                    // AUTH routes
                    .service(api::post_register)
                    // POST /login
                    .service(api::post_login)
                    // POST /logout
                    .service(api::post_logout)
                    .service(api::get_username_exists)
                    .service(api::get_current_user)
                    .service(api::get_index)
                    // API endpoints
                    // GET /album/{album}
                    .service(api::get_album)
                    // GET /image/{image}
                    .service(api::get_image),
            )
            .service(
                // Serve static files
                fs::Files::new("/", "./static/")
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(Into::into)
}
