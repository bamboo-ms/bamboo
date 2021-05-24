use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum MediaType {
    #[cfg(feature = "music")]
    #[serde(rename = "music")]
    Music,
}