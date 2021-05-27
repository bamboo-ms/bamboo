use crate::media_type::MediaType;
use crate::tags::Tag;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod path;
pub mod person;
pub mod role;
pub mod thumbnail;
pub mod title;

#[derive(Serialize, Deserialize)]
pub struct Media {
    id: Uuid,
    name: String,
    r#type: MediaType,
    tags: Vec<Tag>,
}
