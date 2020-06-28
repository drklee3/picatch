use crate::{
    error::{Error, Result},
    filesystem::utils,
    model::{ResizeJob, ResizeOptions},
};
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};
use std::collections::HashMap;
use std::fs::rename;
use std::path::PathBuf;
use threadpool::ThreadPool;

pub fn resize(img: &DynamicImage, opts: &ResizeOptions) -> Result<image::DynamicImage> {
    let old_w = img.width();
    let old_h = img.height();

    let new_w = opts.width.unwrap_or(old_w);
    let new_h = opts.height.unwrap_or(old_h);

    if new_w == old_w && new_h == old_h {
        return Err(Error::Picatch("New resize dimensions are the same".into()));
    }

    let filter_type = opts.filter_type.unwrap_or(FilterType::Lanczos3);

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

fn _execute_resize(job: ResizeJob, img: DynamicImage) -> Result<()> {
    // Make sure destination dir exists
    let dir = job.destination.parent().ok_or(Error::Picatch(format!(
        "Failed to get resized file parent: {}",
        &job.source.to_string_lossy()
    )))?;

    utils::dir_exists_or_create(&dir)?;

    let resized_img = resize(&img, &job.options)?;

    // Copy dest and add ".tmp" to end
    let mut dest_str = job.destination.as_os_str().to_os_string();
    dest_str.push(".tmp");

    // Save to .tmp file first in case of failures or exits
    // Use .save_with_format() since .save() uses extension to determine format
    resized_img.save_with_format(&dest_str, ImageFormat::Jpeg)?;

    // Rename file to correct destination after write to disk
    // Files not renamed will be cleaned up on startup scan
    rename(dest_str, job.destination)?;

    Ok(())
}

/// Wraper around resize fn to catch error messages
fn execute_resize(job: ResizeJob, img: DynamicImage) {
    if let Err(e) = _execute_resize(job, img) {
        error!("Failed to resize image: {}", e);
    }
}

pub fn resize_images(pool: &ThreadPool, jobs_map: HashMap<PathBuf, Vec<ResizeJob>>) -> Result<()> {
    if jobs_map.is_empty() {
        return Ok(());
    }

    for (source, jobs) in jobs_map {
        let img = image::open(source)?;

        // jobs is a list
        for job in jobs {
            let img = img.clone();

            pool.execute(move || execute_resize(job, img));
        }
    }

    Ok(())
}
