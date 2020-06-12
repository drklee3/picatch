use crate::{
    error::{Error, Result},
    filesystem::{hash::get_image_hash, utils},
    model::{config::AppConfig, ResizeOptions},
};
use image::{imageops::FilterType, GenericImageView};
use std::path::{Path, PathBuf};

pub fn resize(img: &image::DynamicImage, opts: &ResizeOptions) -> Result<image::DynamicImage> {
    let old_w = img.width();
    let old_h = img.height();

    let new_w = opts.width.unwrap_or(old_w);
    let new_h = opts.height.unwrap_or(old_h);

    if new_w == old_w && new_h == old_h {
        return Err(Error::Picatch("New resize dimensions are the same".into()));
    }

    let filter_type = opts.filter_type.unwrap_or(FilterType::Lanczos3);

    // this is really slow :(
    let resized_img = match opts.mode {
        1 => img.resize(new_w, new_h, filter_type),
        2 => img.resize_exact(new_w, new_h, filter_type),
        3 => img.resize_to_fill(new_w, new_h, filter_type),
        _ => {
            return Err(Error::Picatch(format!(
                "Invalid resize mode: {}",
                opts.mode
            )))
        }
    };

    Ok(resized_img)
}

pub fn get_resized_file_path(
    config: &AppConfig,
    path: &Path,
    img_path_str: &str,
    opts: &ResizeOptions,
) -> Result<PathBuf> {
    // resized dir + relative file path + size
    let mut dest_path = PathBuf::from(&config.resized_photos_dir);

    // Just in case, check if path includes original_photos_dir
    if dest_path.starts_with(&config.original_photos_dir) {
        dest_path = dest_path
            .strip_prefix(&config.original_photos_dir)
            .map_err(|_| Error::Picatch(format!("Failed to strip prefix: {}", img_path_str)))?
            .to_path_buf();
    }

    // Get file stem first, in case there isn't a file name provided
    let file_name = path
        .file_stem()
        .ok_or(Error::Picatch(format!(
            "Path missing file name: {}",
            img_path_str
        )))?
        .to_string_lossy();

    let file_dir = path
        .parent()
        .ok_or(Error::Picatch(format!(
            "Path missing parent: {}",
            img_path_str
        )))?
        .strip_prefix(&config.original_photos_dir)
        .map_err(|_| Error::Picatch(format!("Failed to strip prefix: {}", img_path_str)))?;

    println!("File parent {}", file_dir.to_string_lossy());

    dest_path.push(file_dir);

    let file_ext = path
        .extension()
        .ok_or(Error::Picatch(format!(
            "Path missing extension: {}",
            img_path_str
        )))?
        .to_string_lossy();

    // Create new file name with size attached.
    // Not including hash for now, frontend doesn't know about the hash
    let new_file_name = format!("{}-{}.{}", file_name, opts.name, file_ext);
    dest_path.push(&new_file_name);

    Ok(dest_path)
}

pub fn resize_images(
    config: &AppConfig,
    paths: Vec<std::path::PathBuf>,
    opts_list: Vec<ResizeOptions>,
) -> Result<()> {
    // Paths should be relative to original_photos_dir and *not* include dir
    for path in &paths {
        debug!("Opening image {}", path.to_string_lossy());
        let img = image::open(path)?;
        debug!("Opened image {}", path.to_string_lossy());

        let img_hash = {
            let mut hash = get_image_hash(img.to_bytes());
            hash.truncate(32);

            hash
        };

        debug!("Image hash: {}", &img_hash);
        let img_path_str = path.to_string_lossy();

        for opts in &opts_list {
            let resized_img = resize(&img, &opts)?;

            let dest_path = get_resized_file_path(config, path, &img_path_str, &opts)?;

            utils::dir_exists_or_create(
                dest_path
                    .parent()
                    .ok_or(Error::Picatch("Failed to get resized file parent".into()))?,
            )?;

            debug!("Saving file to {}", dest_path.to_string_lossy());

            resized_img.save(dest_path)?;
        }
    }

    Ok(())
}
