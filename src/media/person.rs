use crate::media::thumbnail::Thumbnail;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize)]
pub struct Person<Role> {
    pub order: Option<i32>,
    pub name: Option<String>,
    pub role: Option<Role>,
    pub thumbnails: Option<Vec<Thumbnail>>,
}
