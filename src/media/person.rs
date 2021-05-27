use super::role::Role;
use crate::media::thumbnail::Thumbnail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Person {
    pub order: Option<i32>,
    pub name: Option<String>,
    pub role: Option<Role>,
    pub thumbnails: Option<Vec<Thumbnail>>,
}
