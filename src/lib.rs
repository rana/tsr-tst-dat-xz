use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dat"]
#[include = "*.xz"]
pub struct Dat;
