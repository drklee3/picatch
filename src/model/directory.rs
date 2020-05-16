use serde::Serialize;
use std::cmp::Ord;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectoryItemType {
    Dir,
    File,
}

#[derive(Debug, Serialize)]
pub struct DirectoryItem {
    pub r#type: DirectoryItemType,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exif: Option<BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ImageDimensions>,
}

#[derive(Debug, Serialize)]
pub struct DirectoryListing {
    pub current: String,
    pub files: Vec<DirectoryItem>,
}
