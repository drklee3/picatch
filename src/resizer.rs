use crate::error::Result;
use image::{FilterType, GenericImageView, ImageOutputFormat};

pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub mode: u32,
}

pub fn resize(img_path: &str, opts: ResizeOptions) -> Result<Vec<u8>> {
    let mut img = image::open(img_path)?;

    let old_w = img.width();
    let old_h = img.height();

    let new_w = opts.width.unwrap_or(old_w);
    let new_h = opts.height.unwrap_or(old_h);

    if new_w != old_w || new_h != old_h {
        println!("resizing!!");
        // this is really slow :(
        img = match opts.mode {
            1 => img.resize_exact(new_w, new_h, FilterType::Lanczos3),
            2 => img.resize_to_fill(new_w, new_h, FilterType::Lanczos3),
            _ => img.resize(new_w, new_h, FilterType::Lanczos3),
        };
        println!("resized!");
    }

    // let buf = Bytes::new();
    let mut vec = Vec::new();
    let _ = img.write_to(&mut vec, ImageOutputFormat::JPEG(90));

    Ok(vec)
}
