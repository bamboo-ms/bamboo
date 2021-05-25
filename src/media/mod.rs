use crate::media_type::MediaType;
use crate::tags::Tag;
use identifier::Identifier;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod identifier;
pub mod path;
pub mod person;
pub mod thumbnail;

#[derive(Serialize, Deserialize)]
pub struct Media<T> {
    id: Uuid,
    name: String,
    r#type: MediaType,
    tags: Vec<Tag>,
    provider_ids: Vec<Identifier>,
    content: T,
}
