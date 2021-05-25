use crate::media::person::Person;
use crate::media::Media;
use crate::media_type::MediaType;
use crate::music::role::Role;
use crate::tags::Tag;
use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Song {
    artists: Vec<Person<Role>>,
    release: Option<NaiveDate>,
    duration: Option<Duration>,
}
