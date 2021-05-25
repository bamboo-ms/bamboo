use serde::{Deserialize, Serialize};

use super::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ThumbnailType {
    Album,
    Artist,
    Back,
    Background,
    Banner,
    CharacterArt,
    ClearArt,
    ClearLogo,
    DiscArt,
    FanArt,
    KeyArt,
    Headshot,
    Landscape,
    Other(String),
    Poster,
    Spine,
    Thumb, // Images fof a Movie or TV Show Episode itself, commonly used for TV Show episode play previews
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    pub r#type: ThumbnailType,
    pub path: Path,
}
