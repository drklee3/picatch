use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/build/"]
pub struct StaticAssets;
