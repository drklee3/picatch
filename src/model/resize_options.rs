use crate::model::ImageSize;
use image::imageops::FilterType;
use std::convert::From;

#[derive(Clone, Debug)]
pub struct ResizeOptions {
    pub name: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub mode: u32,
    pub filter_type: Option<FilterType>,
}

impl ResizeOptions {
    pub fn new<T: Into<String>>(name: T) -> Self {
        ResizeOptions {
            name: name.into(),
            width: None,
            height: None,
            mode: 1,
            filter_type: Some(FilterType::Lanczos3),
        }
    }

    pub fn set_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn set_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn set_mode(mut self, mode: u32) -> Self {
        self.mode = mode;
        self
    }

    pub fn set_filter_type(mut self, filter_type: FilterType) -> Self {
        self.filter_type = Some(filter_type);
        self
    }
}

impl From<ImageSize> for ResizeOptions {
    fn from(size: ImageSize) -> Self {
        match size {
            ImageSize::Pixel => ResizeOptions::new("pixel")
                .set_height(1)
                .set_width(1)
                .set_mode(2),
            ImageSize::Thumbnail => ResizeOptions::new("thumbnail").set_height(128),
            ImageSize::Small => ResizeOptions::new("small").set_height(270),
            ImageSize::Medium => ResizeOptions::new("medium").set_height(640),
            ImageSize::Large => ResizeOptions::new("large").set_height(1080),
        }
    }
}
