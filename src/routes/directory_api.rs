use actix_web::{web, HttpRequest, HttpResponse};
use image::image_dimensions;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::Path;


use crate::{
    error::Result,
    model::{
        config::AppConfig,
        directory::{AlbumInfo, DirectoryAlbum, DirectoryFile, DirectoryListing, ImageDimensions},
    },
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

pub async fn dir_listing(
    _req: HttpRequest,
    path: web::Path<String>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    let listing = get_dir_listing(path.into_inner(), config.get_ref())?;

    Ok(HttpResponse::Ok().json(listing))
}

fn get_album_data(dir: &Path) -> Option<AlbumInfo> {
    // TODO: Just push to the dir path, don't have to iterate
    // Look for _picatch.album.toml
    let album_conf = dir
        .read_dir()
        .ok()?
        .find(|entry| {
            entry
                .as_ref()
                .map(|e| e.file_name() == OsString::from("_picatch.album.toml"))
                .unwrap_or(false)
        })?
        .ok()?;

    let info: AlbumInfo = toml::from_str(&read_to_string(album_conf.path()).ok()?).ok()?;

    Some(info)
}

pub fn get_dir_listing(path: String, config: &AppConfig) -> Result<DirectoryListing> {
    let base = Path::new(&*config.original_photos_dir);

    // replacement should end in "/" to remove the "/" at beginning of relative_path
    // If relative_path starts with "/", it will replace base.
    let relative_path = path.replace("/api/photos/", "");
    let album_path = base.join(relative_path);

    let mut albums = Vec::new();
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
            if name.starts_with('.') {
                continue;
            }
        }

        // if file is a directory, add '/' to the end of the name
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                let info = get_album_data(&entry.path());

                let dir_item = DirectoryAlbum {
                    name: format!("{}/", entry.file_name().to_string_lossy().to_string()),
                    info,
                };

                albums.push(dir_item);
            } else {
                let file_path = entry.path();
                let extension = match file_path
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_lowercase())
                {
                    Some(e) => e,
                    None => continue,
                };

                if extension != "jpg" && extension != "jpeg" {
                    continue;
                }

                let dir_item = DirectoryFile {
                    name: entry.file_name().to_string_lossy().to_string(),
                    exif: get_exif_data(&file_path),
                    dimensions: get_image_dimensions(&file_path),
                };

                files.push(dir_item);
            }
        } else {
            continue;
        }
    }

    // Sort by filename, doesn't need to be stable
    files.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    albums.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    Ok(DirectoryListing {
        current: album_path.to_string_lossy().to_string(),
        files,
        albums,
    })
}
