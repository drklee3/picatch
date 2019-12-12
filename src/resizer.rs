use actix_web::web::Bytes;
use image::{DynamicImage, FilterType, ImageOutputFormat, ImageResult};

pub struct ResizeOptions {
    pub width: u32,
    pub height: u32,
    pub mode: u32,
}

pub fn resize(img_path: &str, opts: ResizeOptions) -> Vec<u8> {
    let mut img = image::open(img_path).unwrap();
    // this is really slow :(
    img = match opts.mode {
        1 => img.resize_exact(opts.width, opts.height, FilterType::Lanczos3),
        2 => img.resize_to_fill(opts.width, opts.height, FilterType::Lanczos3),
        _ => img.resize(opts.width, opts.height, FilterType::Lanczos3),
    };

    // let buf = Bytes::new();
    let mut vec = Vec::new();
    let _ = img.write_to(&mut vec, ImageOutputFormat::JPEG(90));

    vec
}
