use actix_web::{web, HttpRequest, HttpResponse};
use image::image_dimensions;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::{
    constants::PHOTOS_DIR,
    error::Result,
    model::directory::{DirectoryItem, DirectoryItemType, DirectoryListing, ImageDimensions},
};

pub fn get_exif_data(path: &Path) -> Option<BTreeMap<String, String>> {
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

pub fn get_image_dimensions(path: &Path) -> Option<ImageDimensions> {
    image_dimensions(path).ok().map(|x| ImageDimensions {
        width: x.0,
        height: x.1,
    })
}

pub async fn dir_listing(_req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse> {
    let listing = get_dir_listing(path.into_inner())?;

    Ok(HttpResponse::Ok().json(listing))
}

pub fn get_dir_listing(path: String) -> Result<DirectoryListing> {
    let base = Path::new(&*PHOTOS_DIR);

    // replacement should end in "/" to remove the "/" at beginning of relative_path
    // If relative_path starts with "/", it will replace base.
    let relative_path = path.replace("/api/photos/", "");
    let album_path = base.join(relative_path);

    let mut files = Vec::new();

    // album_path.is_dir

    for entry in album_path.read_dir()? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("Failed to read file in directory {}", e);
                continue;
            }
        };

        // Ignore dotfiles
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with(".") {
                continue;
            }
        }

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

    // First sort by filename, doesn't need to be stable
    files.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    // Second sort by dir/file, needs to be stable to preserve file order
    files.sort_by(|a, b| a.r#type.cmp(&b.r#type));

    Ok(DirectoryListing {
        current: album_path.to_string_lossy().to_string(),
        files,
    })
}
