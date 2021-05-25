use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub enum MediaType {
    #[cfg(feature = "music")]
    #[serde(rename = "music")]
    Music,
}
