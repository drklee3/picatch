#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;

use actix_files as fs;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, HttpRequest, HttpResponse};
use actix_web::dev::ServiceResponse;
use dotenv;
use dphoto_lib::*;
use std::path::Path;
use error::Result;
use std::result::Result as StdResult;
use std::io;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DirectoryListing {
    current: String,
    files: Vec<String>,
}

fn render_dir(dir: &fs::Directory, req: &HttpRequest
) -> StdResult<ServiceResponse, io::Error> {
    let base = Path::new(req.path());
    let mut files = Vec::new();

    for entry in dir.path.read_dir()? {
        if dir.is_visible(&entry) {
            let entry = entry.unwrap();

            let p = match entry.path().strip_prefix(&dir.path) {
                Ok(p) if cfg!(windows) => base.join(p).to_string_lossy().replace("\\", "/"),
                Ok(p) => base.join(p).to_string_lossy().into_owned(),
                Err(_) => continue,
            };

            // if file is a directory, add '/' to the end of the name
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    files.push(format!("{}/", entry.file_name().to_string_lossy().to_string()));
                } else {
                    files.push(entry.file_name().to_string_lossy().to_string());
                }
            } else {
                continue;
            }
        }
    }

    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Ok().json(
            DirectoryListing {
                current: req.path().to_string(),
                files,
            }
        )
    ))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::logging::setup_logger()?;

    HttpServer::new(move || {
        App::new()
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
            /*
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
            */
            .default_service(
                // Serve static files
                fs::Files::new("/", "./static/")
                    .show_files_listing()
                    .files_listing_renderer(render_dir)
                    //.index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(Into::into)
}
