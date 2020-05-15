use actix_cors::Cors;
use actix_files as fs;
use actix_web::dev::ServiceResponse;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use dotenv;
use image::image_dimensions;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::result::Result as StdResult;

use picatch_lib::{
    error::Result,
    model::directory::{DirectoryItem, DirectoryItemType, DirectoryListing, ImageDimensions},
    routes, utils,
};

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
    static ref PHOTOS_DIR: String =
        env::var("PICATCH_PHOTOS_DIR").unwrap_or("./photos".to_string());
    static ref PUBLIC_DIR: String =
        env::var("PICATCH_PUBLIC_DIR").unwrap_or("./web/build".to_string());
}

fn render_dir(dir: &fs::Directory, req: &HttpRequest) -> StdResult<ServiceResponse, io::Error> {
    let base = Path::new(req.path());
    let mut files = Vec::new();

    for entry in dir.path.read_dir()? {
        if dir.is_visible(&entry) {
            if entry.is_err() {
                continue;
            }

            let entry = entry.unwrap();

            let _p = match entry.path().strip_prefix(&dir.path) {
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

    // First sort by filename, doesn't need to be stable
    files.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    // Second sort by dir/file, needs to be stable to preserve file order
    files.sort_by(|a, b| a.r#type.cmp(&b.r#type));

    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Ok().json(DirectoryListing {
            current: dir.base.to_string_lossy().to_string(),
            files,
        }),
    ))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::logging::setup_logger()?;

    let interface = env::var("PICATCH_INTERFACE").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PICATCH_PORT").unwrap_or("8080".to_string());

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
                    .finish(),
            )
            .wrap(middleware::Compress::default())
            // enable logger - register logger last!
            .wrap(middleware::Logger::default())
            .service(
                fs::Files::new("/photos", PHOTOS_DIR.clone())
                    .files_listing_renderer(render_dir)
                    .show_files_listing(),
            )
            .service(web::resource("/{path:.*}").route(web::get().to(routes::static_files::path)))
    })
    .bind(format!("{}:{}", interface, port))?
    .run()
    .await
    .map_err(Into::into)
}
