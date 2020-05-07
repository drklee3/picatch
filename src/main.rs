#[macro_use]
extern crate log;

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
use std::io::{self, BufReader};
use std::collections::HashMap;
use std::fs::File;
use serde::Serialize;

#[derive(Debug, Serialize)]
enum DirectoryItemType {
    Dir,
    File,
}

#[derive(Debug, Serialize)]
struct DirectoryItem {
    r#type: DirectoryItemType,
    name: String,
    exif: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
struct DirectoryListing {
    current: String,
    files: Vec<DirectoryItem>,
}

fn get_exif_data(path: &Path) -> Option<HashMap<String, String>> {
    let file = File::open(path).ok()?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;

    let mut exif_map = HashMap::new();
    for f in exif.fields() {
        if let Some(tag_name) = f.tag.description() {
            exif_map.insert(tag_name.to_string(), f.display_value().with_unit(&exif).to_string());
        }
    }

    Some(exif_map)
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
                    let dir_item = DirectoryItem {
                        r#type: DirectoryItemType::Dir,
                        name: format!("{}/", entry.file_name().to_string_lossy().to_string()),
                        exif: None,
                    };

                    files.push(dir_item);
                } else {
                    let dir_item = DirectoryItem {
                        r#type: DirectoryItemType::File,
                        name: entry.file_name().to_string_lossy().to_string(),
                        exif: get_exif_data(&entry.path()),
                    };

                    files.push(dir_item);
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
            .service(
                web::scope("/api")
                    .default_service(
                        // Serve static files
                        fs::Files::new("/", "./static/")
                            .show_files_listing()
                            .files_listing_renderer(render_dir)
                    )
            )

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(Into::into)
}
