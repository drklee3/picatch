use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlbumInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DirectoryAlbum {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<AlbumInfo>,
}

#[derive(Debug, Serialize)]
pub struct DirectoryFile {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exif: Option<BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ImageDimensions>,
}

#[derive(Debug, Serialize)]
pub struct DirectoryListing {
    pub current: String,
    pub albums: Vec<DirectoryAlbum>,
    pub files: Vec<DirectoryFile>,
}
