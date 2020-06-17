use crate::{
    error::{Error, Result},
    filesystem::{hash::get_image_hash, utils},
    model::{config::AppConfig, ResizeJob, ResizeOptions},
};
use image::{imageops::FilterType, DynamicImage, GenericImageView};

pub fn resize(img: &DynamicImage, opts: &ResizeOptions) -> Result<image::DynamicImage> {
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

pub fn _resize_images(
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

        for opts in &opts_list {
            let resized_img = resize(&img, &opts)?;

            let dest_path = utils::get_destination_path(config, path, &opts)?;

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

pub fn resize_images(jobs: Vec<ResizeJob>) -> Result<()> {
    if jobs.is_empty() {
        return Ok(());
    }

    // Make mutable
    let mut jobs = jobs;

    // Sort so that same source jobs are next to each other, open source once and reuse buf
    jobs.sort_by(|a, b| a.source.cmp(&b.source));

    // Safe to unwrap, checked if empty earlier
    let mut prev_job = jobs.first().unwrap();
    let mut img = image::open(prev_job.source)?;
    
    println!("Resize jobs (sorted): {:#?}", &jobs);

    for job in &jobs {
        // Open image if it's a new one
        if prev_job.source != job.source {
            println!("New image, opening file");
            img = image::open(job.source)?;
        } else {
            println!("Image already in memory, continuing to resize");
        }
        prev_job = job;

        // Make sure destination dir exists
        utils::dir_exists_or_create(&job.destination.parent().ok_or(Error::Picatch(format!(
            "Failed to get resized file parent: {}",
            &job.source.to_string_lossy()
        )))?)?;

        let resized_img = resize(&img, job.options)?;
        resized_img.save(&job.destination)?;
    }

    Ok(())
}
