#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_files as fs;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, HttpRequest, HttpResponse};
use actix_web::dev::ServiceResponse;
use dotenv;
use image::image_dimensions;
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::result::Result as StdResult;

use dphoto_lib::*;
use error::Result;

#[derive(Debug, Serialize)]
struct ImageDimensions {
    width: u32,
    height: u32,
}

#[derive(Debug, Serialize)]
enum DirectoryItemType {
    Dir,
    File,
}

#[derive(Debug, Serialize)]
struct DirectoryItem {
    r#type: DirectoryItemType,
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    exif: Option<BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<ImageDimensions>,
}

#[derive(Debug, Serialize)]
struct DirectoryListing {
    current: String,
    files: Vec<DirectoryItem>,
}

fn get_exif_data(path: &Path) -> Option<BTreeMap<String, String>> {
    let file = File::open(path).ok()?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;

    let mut exif_map = BTreeMap::new();
    for f in exif.fields() {
        let key = format!("{}", f.tag);
        exif_map.insert(key, f.display_value().with_unit(&exif).to_string());
    }

    Some(exif_map)
}

fn get_image_dimensions(path: &Path) -> Option<ImageDimensions> {
    image_dimensions(path).ok().map(|x| ImageDimensions {
        width: x.0,
        height: x.1,
    })
}

lazy_static! {
    static ref PHOTOS_DIR: String = env::var("DPHOTO_PHOTOS_DIR").unwrap_or("./photos".to_string());
    static ref PUBLIC_DIR: String = env::var("DPHOTO_PUBLIC_DIR").unwrap_or("./web/build".to_string());
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
                        dimensions: None,
                    };

                    files.push(dir_item);
                } else {
                    let dir_item = DirectoryItem {
                        r#type: DirectoryItemType::File,
                        name: entry.file_name().to_string_lossy().to_string(),
                        exif: get_exif_data(&entry.path()),
                        dimensions: get_image_dimensions(&entry.path()),
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
                current: dir.base.to_string_lossy().to_string(),
                files,
            }
        )
    ))
}

async fn index(req: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("{}/index.html", *PUBLIC_DIR))?)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::logging::setup_logger()?;

    let interface = env::var("DPHOTO_INTERFACE").unwrap_or("0.0.0.0".to_string());
    let port = env::var("DPHOTO_PORT").unwrap_or("8080".to_string());

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .wrap(
                Cors::new()
                  .send_wildcard()
                  .allowed_methods(vec!["GET", "POST"])
                  .max_age(3600)
                  .finish())
            .wrap(IdentityService::new(
                // TODO: Update secret key with an actual secret key
                CookieIdentityPolicy::new(&[0; 32])
                    .name("dphoto-auth")
                    .secure(false)
                    .same_site(SameSite::Strict),
            ))
            // enable logger - register logger last!
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/photos", PHOTOS_DIR.clone())
                .files_listing_renderer(render_dir)
                .show_files_listing()
            )
            .service(
                // TODO: Keep static files in memory?
                fs::Files::new("/", PUBLIC_DIR.clone())
                    .index_file("index.html")
            )
            .default_service(
                web::resource("")
                    .route(web::get().to(index))
            )

    })
    .bind(format!("{}:{}", interface, port))?
    .run()
    .await
    .map_err(Into::into)
}
